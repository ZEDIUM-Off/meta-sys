//! Typed emit routes and broadcast listener declarations.

use super::{EventTypeId, RoomAddress};

/// Contract route from an emitted Event type to one owned logical Room.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmissionDeclaration {
    /// Event payload contract eligible for this emit route.
    event_type: EventTypeId,
    /// Declared Room that distributes matching Events to subscribers.
    room: RoomAddress,
}

impl EmissionDeclaration {
    /// Declares one typed Event route owned by the emitting Component.
    #[must_use]
    pub const fn new(event_type: EventTypeId, room: RoomAddress) -> Self {
        Self { event_type, room }
    }

    /// Returns the Event payload contract eligible for this route.
    #[must_use]
    pub const fn event_type(self) -> EventTypeId {
        self.event_type
    }

    /// Returns the owned logical Room used for distribution.
    #[must_use]
    pub const fn room(self) -> RoomAddress {
        self.room
    }
}

/// Registration of one Component as a listener for a broadcast Event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BroadcastSubscription {
    /// Event payload contract accepted from broadcast.
    event_type: EventTypeId,
}

impl BroadcastSubscription {
    /// Registers one broadcast Event payload contract.
    #[must_use]
    pub const fn new(event_type: EventTypeId) -> Self {
        Self { event_type }
    }

    /// Returns the broadcast Event payload contract.
    #[must_use]
    pub const fn event_type(self) -> EventTypeId {
        self.event_type
    }
}
