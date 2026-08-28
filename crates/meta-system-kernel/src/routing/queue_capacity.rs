//! Validated positive bounds shared by Rooms and Component Mailboxes.

use std::num::NonZeroUsize;

/// Explicit positive capacity of one bounded routing queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueCapacity(NonZeroUsize);

impl QueueCapacity {
    /// Prototype default used when a Definition does not override its Mailbox.
    pub const DEFAULT: Self = Self(NonZeroUsize::new(16).expect("sixteen is non-zero"));

    /// Validates and creates a positive queue capacity.
    #[must_use]
    pub const fn new(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the positive queue bound.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0.get()
    }
}

impl Default for QueueCapacity {
    fn default() -> Self {
        Self::DEFAULT
    }
}
