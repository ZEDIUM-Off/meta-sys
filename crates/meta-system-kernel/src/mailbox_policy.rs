//! Component-owned bounds and overflow decisions for one Runtime Mailbox.

use crate::QueueCapacity;

/// Component strategy invoked when its bounded Mailbox is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MailboxOverflowStrategy {
    /// Rejects the new Delivery and preserves every pending Delivery.
    #[default]
    RejectNew,
    /// Drops the oldest pending Delivery and accepts the new Delivery once.
    DropOldest,
}

/// Complete Component-owned policy of one concrete Runtime Mailbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MailboxPolicy {
    /// Maximum number of pending Deliveries.
    capacity: QueueCapacity,
    /// Decision invoked exactly once when the bound is reached.
    overflow: MailboxOverflowStrategy,
}

impl MailboxPolicy {
    /// Creates one explicit bounded Mailbox policy.
    #[must_use]
    pub const fn new(capacity: QueueCapacity, overflow: MailboxOverflowStrategy) -> Self {
        Self { capacity, overflow }
    }

    /// Returns the positive pending Delivery bound.
    #[must_use]
    pub const fn capacity(self) -> QueueCapacity {
        self.capacity
    }

    /// Returns the Component strategy invoked when the Mailbox is full.
    #[must_use]
    pub const fn overflow(self) -> MailboxOverflowStrategy {
        self.overflow
    }
}

impl Default for MailboxPolicy {
    fn default() -> Self {
        Self::new(QueueCapacity::DEFAULT, MailboxOverflowStrategy::RejectNew)
    }
}
