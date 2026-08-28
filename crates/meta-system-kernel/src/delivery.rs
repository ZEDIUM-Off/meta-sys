//! Observable attempts to place routed Events into Component Mailboxes.

use crate::{ComponentInstanceId, Event, RoomAddress, RoomSequence};

/// One immutable Event placement offered to a subscribed Component Mailbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    /// Logical Room that distributed this Event.
    room: RoomAddress,
    /// Room-local FIFO position assigned at acceptance.
    sequence: RoomSequence,
    /// Component Runtime whose Mailbox receives the Event.
    recipient: ComponentInstanceId,
    /// Sole common Event concept carried by every routing operation.
    event: Event,
}

impl Delivery {
    /// Creates one delivery attempt for an active subscription.
    #[must_use]
    pub(crate) const fn new(
        room: RoomAddress,
        sequence: RoomSequence,
        recipient: ComponentInstanceId,
        event: Event,
    ) -> Self {
        Self {
            room,
            sequence,
            recipient,
            event,
        }
    }

    /// Returns the logical Room that distributed this Event.
    #[must_use]
    pub const fn room(&self) -> RoomAddress {
        self.room
    }

    /// Returns the Room-local FIFO position.
    #[must_use]
    pub const fn sequence(&self) -> RoomSequence {
        self.sequence
    }

    /// Returns the destination Component Instance.
    #[must_use]
    pub const fn recipient(&self) -> ComponentInstanceId {
        self.recipient
    }

    /// Returns the single routed Event.
    #[must_use]
    pub const fn event(&self) -> &Event {
        &self.event
    }
}

/// Processing observation available from the configured Event Loop Driver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryProgress {
    /// The Driver cannot confirm processing at send time.
    Unobserved,
    /// The Driver confirms that the recipient processed this Delivery.
    Processed,
}
