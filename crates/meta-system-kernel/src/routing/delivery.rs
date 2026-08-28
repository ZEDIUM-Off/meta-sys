//! Observable attempts to place routed Events into Component Mailboxes.

use super::{Event, RoomAddress, RoomSequence};
use crate::system::ComponentInstanceId;

/// Routing operation that produced one Mailbox Delivery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryOrigin {
    /// Distribution from one concrete FIFO Room acceptance step.
    Room {
        /// Stable logical Room address.
        address: RoomAddress,
        /// FIFO position local to the concrete Room lifecycle.
        sequence: RoomSequence,
    },
    /// Direct distribution to registered broadcast listeners.
    Broadcast,
}

/// One immutable Event placement offered to a subscribed Component Mailbox.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delivery {
    /// Routing operation that produced this Delivery.
    origin: DeliveryOrigin,
    /// Component Runtime whose Mailbox receives the Event.
    recipient: ComponentInstanceId,
    /// Sole common Event concept carried by every routing operation.
    event: Event,
}

impl Delivery {
    /// Creates one delivery attempt for an active subscription.
    #[must_use]
    pub(crate) const fn from_room(
        room: RoomAddress,
        sequence: RoomSequence,
        recipient: ComponentInstanceId,
        event: Event,
    ) -> Self {
        Self {
            origin: DeliveryOrigin::Room {
                address: room,
                sequence,
            },
            recipient,
            event,
        }
    }

    /// Creates one direct Delivery to a registered broadcast listener.
    #[must_use]
    pub(crate) const fn from_broadcast(recipient: ComponentInstanceId, event: Event) -> Self {
        Self {
            origin: DeliveryOrigin::Broadcast,
            recipient,
            event,
        }
    }

    /// Returns the routing operation that produced this Delivery.
    #[must_use]
    pub const fn origin(&self) -> DeliveryOrigin {
        self.origin
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
