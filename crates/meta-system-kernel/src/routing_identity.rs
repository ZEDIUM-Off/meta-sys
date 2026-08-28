//! Strong identities local to Event routing lifecycles.

use crate::ComponentRuntimeId;

/// Identifies one Event independently of its routing operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(u64);

impl EventId {
    /// Creates an opaque Event identity from a Component-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies the declared type contract carried by an Event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventTypeId(u64);

impl EventTypeId {
    /// Creates an opaque Event type identity from a contract-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Stable logical address of a Room across concrete Runtime lifecycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomAddress(u64);

impl RoomAddress {
    /// Creates a stable logical Room address from a declaration value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one concrete Room attached to one Component Runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomRuntimeId {
    /// Concrete Component Runtime that owns this Room lifecycle.
    runtime: ComponentRuntimeId,
    /// Declaration position unique within the owning Component Definition.
    ordinal: usize,
}

impl RoomRuntimeId {
    /// Creates one lifecycle-local Room identity during activation.
    #[must_use]
    pub(crate) const fn new(runtime: ComponentRuntimeId, ordinal: usize) -> Self {
        Self { runtime, ordinal }
    }

    /// Returns the concrete Component Runtime that owns this Room.
    #[must_use]
    pub const fn runtime(self) -> ComponentRuntimeId {
        self.runtime
    }

    /// Returns the declaration position within the owning Definition.
    #[must_use]
    pub const fn ordinal(self) -> usize {
        self.ordinal
    }
}

/// Monotonic FIFO acceptance position local to one concrete Room.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoomSequence(u64);

impl RoomSequence {
    /// Creates a Room-local sequence after successful acceptance.
    #[must_use]
    pub(crate) const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Returns the Room-local FIFO position for diagnostics and tests.
    #[must_use]
    pub const fn value(self) -> u64 {
        self.0
    }
}
