//! Declarative and living relations from Rooms to Component Mailboxes.

use super::RoomAddress;
use crate::system::ComponentInstanceId;

/// Room relation contributed by one subscribing Component Definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubscriptionDeclaration {
    /// Logical Room whose Deliveries the Component accepts.
    room: RoomAddress,
}

impl SubscriptionDeclaration {
    /// Declares a Component's interest in one logical Room.
    #[must_use]
    pub const fn new(room: RoomAddress) -> Self {
        Self { room }
    }

    /// Returns the subscribed logical Room address.
    #[must_use]
    pub const fn room(self) -> RoomAddress {
        self.room
    }
}

/// Living routing relation to one Active Component Runtime Mailbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subscription {
    /// Logical Room that distributes Events.
    room: RoomAddress,
    /// Active Component Instance whose Runtime owns the destination Mailbox.
    subscriber: ComponentInstanceId,
}

impl Subscription {
    /// Materializes one declared relation during Component activation.
    #[must_use]
    pub(crate) const fn new(room: RoomAddress, subscriber: ComponentInstanceId) -> Self {
        Self { room, subscriber }
    }

    /// Returns the subscribed logical Room address.
    #[must_use]
    pub const fn room(self) -> RoomAddress {
        self.room
    }

    /// Returns the Active destination Component Instance.
    #[must_use]
    pub const fn subscriber(self) -> ComponentInstanceId {
        self.subscriber
    }
}
