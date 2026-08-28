//! Single inspectable Event concept shared by every routing operation.

use crate::{EventId, EventTypeId};

/// Typed message that provokes or propagates a state evolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    /// Stable identity supplied by the Event producer.
    id: EventId,
    /// Declared payload contract understood by participating Components.
    contract: EventTypeId,
    /// Opaque contract-owned bytes never interpreted by the Kernel.
    payload: Vec<u8>,
}

impl Event {
    /// Creates one typed Event with opaque contract-owned payload bytes.
    #[must_use]
    pub fn new(id: EventId, event_type: EventTypeId, payload: impl Into<Vec<u8>>) -> Self {
        Self {
            id,
            contract: event_type,
            payload: payload.into(),
        }
    }

    /// Returns the stable Event identity.
    #[must_use]
    pub const fn id(&self) -> EventId {
        self.id
    }

    /// Returns the declared Event payload contract.
    #[must_use]
    pub const fn event_type(&self) -> EventTypeId {
        self.contract
    }

    /// Returns the opaque payload bytes.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
}
