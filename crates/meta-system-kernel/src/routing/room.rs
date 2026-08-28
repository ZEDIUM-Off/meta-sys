//! Declared logical Rooms and their concrete FIFO Runtime lifecycles.

use std::collections::VecDeque;

use super::{Event, QueueCapacity, RoomAddress, RoomRuntimeId, RoomSequence};
use crate::{runtime::KernelError, system::ComponentInstanceId};

/// Complete routing contribution declared by a Component Definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoomDeclaration {
    /// Stable logical address preserved across Runtime lifecycles.
    address: RoomAddress,
    /// Explicit bound of the concrete Room distribution queue.
    capacity: QueueCapacity,
}

impl RoomDeclaration {
    /// Declares one logical Room and its concrete queue bound.
    #[must_use]
    pub const fn new(address: RoomAddress, capacity: QueueCapacity) -> Self {
        Self { address, capacity }
    }

    /// Returns the stable logical address.
    #[must_use]
    pub const fn address(self) -> RoomAddress {
        self.address
    }

    /// Returns the concrete Room queue bound.
    #[must_use]
    pub const fn capacity(self) -> QueueCapacity {
        self.capacity
    }
}

/// One Event accepted at a Room-local FIFO position for distribution.
#[derive(Debug)]
pub struct AcceptedEvent {
    /// FIFO position assigned by the accepting Room.
    pub(crate) sequence: RoomSequence,
    /// Event removed for exactly one distribution step.
    pub(crate) event: Event,
}

/// Concrete bounded Room attached to one Active Component Runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    /// Identity unique to this concrete Runtime lifecycle.
    id: RoomRuntimeId,
    /// Stable logical address used by senders.
    address: RoomAddress,
    /// Active Component Instance that owns the Room.
    owner: ComponentInstanceId,
    /// Explicit distribution queue bound.
    capacity: QueueCapacity,
    /// FIFO Events awaiting their single distribution step.
    queue: VecDeque<Event>,
    /// Next Room-local FIFO position.
    next_sequence: u64,
}

impl Room {
    /// Materializes one concrete Room for an activating Component Runtime.
    #[must_use]
    pub(crate) const fn new(
        id: RoomRuntimeId,
        declaration: RoomDeclaration,
        owner: ComponentInstanceId,
    ) -> Self {
        Self {
            id,
            address: declaration.address(),
            owner,
            capacity: declaration.capacity(),
            queue: VecDeque::new(),
            next_sequence: 1,
        }
    }

    /// Accepts one Event and removes exactly one FIFO distribution step.
    pub(crate) fn accept(&mut self, event: Event) -> Result<AcceptedEvent, KernelError> {
        if self.queue.len() == self.capacity.get() {
            return Err(KernelError::RoomQueueFull(self.address));
        }
        let sequence = RoomSequence::new(self.next_sequence);
        self.next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or(KernelError::RoomSequenceExhausted(self.address))?;
        self.queue.push_back(event);
        Ok(AcceptedEvent {
            sequence,
            event: self.queue.pop_front().expect("accepted Event is queued"),
        })
    }

    /// Returns the concrete Room lifecycle identity.
    #[must_use]
    pub const fn id(&self) -> RoomRuntimeId {
        self.id
    }

    /// Returns the stable logical address.
    #[must_use]
    pub const fn address(&self) -> RoomAddress {
        self.address
    }

    /// Returns the active Component Instance that owns this Room.
    #[must_use]
    pub const fn owner(&self) -> ComponentInstanceId {
        self.owner
    }

    /// Returns the explicit distribution queue bound.
    #[must_use]
    pub const fn capacity(&self) -> QueueCapacity {
        self.capacity
    }

    /// Returns Events still awaiting a future distribution step.
    #[must_use]
    pub fn queued(&self) -> usize {
        self.queue.len()
    }
}
