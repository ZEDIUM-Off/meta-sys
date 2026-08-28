//! Inspectable acceptance, delivery, and processing observations for send.

use crate::{ComponentInstanceId, RoomAddress, RoomSequence};

/// Observable result of one Delivery attempt without hidden retry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeliveryState {
    /// The bounded Mailbox accepted the Delivery; processing is unobserved.
    Delivered,
    /// The Mailbox accepted and the Driver confirmed processing.
    Processed,
    /// The bounded Mailbox could not accept this Delivery.
    MailboxFull,
}

/// Recipient-local observation included in one Send Receipt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeliveryReceipt {
    /// Component Instance targeted by the Subscription.
    recipient: ComponentInstanceId,
    /// Observable Delivery outcome at send time.
    state: DeliveryState,
}

impl DeliveryReceipt {
    /// Creates one recipient-local Delivery observation.
    #[must_use]
    pub(crate) const fn new(recipient: ComponentInstanceId, state: DeliveryState) -> Self {
        Self { recipient, state }
    }

    /// Returns the subscribed Component recipient.
    #[must_use]
    pub const fn recipient(self) -> ComponentInstanceId {
        self.recipient
    }

    /// Returns the observable Delivery state without implying retry.
    #[must_use]
    pub const fn state(self) -> DeliveryState {
        self.state
    }

    /// Reports whether the recipient Mailbox accepted the Delivery.
    #[must_use]
    pub const fn delivered(self) -> bool {
        matches!(
            self.state,
            DeliveryState::Delivered | DeliveryState::Processed
        )
    }

    /// Reports whether the Driver confirmed processing at send time.
    #[must_use]
    pub const fn processed(self) -> bool {
        matches!(self.state, DeliveryState::Processed)
    }
}

/// Complete observation returned after one Room accepts and distributes send.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "Send Receipts expose acceptance and every Delivery outcome"]
pub struct SendReceipt {
    /// Logical Room that accepted the Event.
    room: RoomAddress,
    /// FIFO position assigned by the concrete Room lifecycle.
    sequence: RoomSequence,
    /// Recipient-local Delivery outcomes in deterministic Subscription order.
    deliveries: Vec<DeliveryReceipt>,
}

impl SendReceipt {
    /// Records one accepted Room distribution step and its Delivery outcomes.
    pub(crate) const fn new(
        room: RoomAddress,
        sequence: RoomSequence,
        deliveries: Vec<DeliveryReceipt>,
    ) -> Self {
        Self {
            room,
            sequence,
            deliveries,
        }
    }

    /// Reports Room acceptance; unavailable Rooms return an error instead.
    #[must_use]
    pub const fn accepted(&self) -> bool {
        true
    }

    /// Returns the accepting logical Room address.
    #[must_use]
    pub const fn room(&self) -> RoomAddress {
        self.room
    }

    /// Returns the Room-local FIFO acceptance position.
    #[must_use]
    pub const fn sequence(&self) -> RoomSequence {
        self.sequence
    }

    /// Returns every recipient-local Delivery outcome.
    #[must_use]
    pub fn deliveries(&self) -> &[DeliveryReceipt] {
        &self.deliveries
    }
}
