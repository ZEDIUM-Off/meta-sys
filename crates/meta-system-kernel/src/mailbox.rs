//! Bounded Delivery queue owned by one concrete Component Runtime.

use std::collections::VecDeque;

use crate::{Delivery, DeliveryState, MailboxOverflowStrategy, MailboxPolicy, QueueCapacity};

/// Result of applying one Component-owned Mailbox placement strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MailboxPlacement {
    /// The Mailbox had capacity and accepted the Delivery.
    Accepted,
    /// `RejectNew` preserved pending work and rejected the Delivery.
    RejectedFull,
    /// `DropOldest` removed one pending Delivery and accepted the new one.
    AcceptedAfterDroppingOldest,
}

impl MailboxPlacement {
    /// Converts the private placement result into its public Receipt state.
    pub const fn delivery_state(self) -> DeliveryState {
        match self {
            Self::Accepted => DeliveryState::Delivered,
            Self::RejectedFull => DeliveryState::RejectedFull,
            Self::AcceptedAfterDroppingOldest => DeliveryState::DeliveredAfterDroppingOldest,
        }
    }
}

/// Bounded aggregate of Deliveries received from subscribed Rooms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mailbox {
    /// Component-declared maximum number of pending Deliveries.
    policy: MailboxPolicy,
    /// Pending Deliveries in their Runtime-local arrival order.
    deliveries: VecDeque<Delivery>,
}

impl Mailbox {
    /// Creates an empty Mailbox for one new Component Runtime lifecycle.
    #[must_use]
    pub(crate) const fn new(policy: MailboxPolicy) -> Self {
        Self {
            policy,
            deliveries: VecDeque::new(),
        }
    }

    /// Attempts one Delivery without retrying or exceeding the explicit bound.
    pub(crate) fn deliver(&mut self, delivery: Delivery) -> MailboxPlacement {
        if self.deliveries.len() < self.policy.capacity().get() {
            self.deliveries.push_back(delivery);
            return MailboxPlacement::Accepted;
        }
        if self.policy.overflow() == MailboxOverflowStrategy::RejectNew {
            return MailboxPlacement::RejectedFull;
        }
        self.deliveries.pop_front();
        self.deliveries.push_back(delivery);
        MailboxPlacement::AcceptedAfterDroppingOldest
    }

    /// Removes a Delivery synchronously confirmed as processed by the Driver.
    pub(crate) fn mark_processed(&mut self, processed: &Delivery) {
        let position = self
            .deliveries
            .iter()
            .position(|delivery| delivery == processed);
        if let Some(position) = position {
            self.deliveries.remove(position);
        }
    }

    /// Returns the explicit pending Delivery bound.
    #[must_use]
    pub const fn capacity(&self) -> QueueCapacity {
        self.policy.capacity()
    }

    /// Returns the complete Component-owned placement policy.
    #[must_use]
    pub const fn policy(&self) -> MailboxPolicy {
        self.policy
    }

    /// Returns the number of pending Deliveries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.deliveries.len()
    }

    /// Reports whether this Mailbox has no pending Delivery.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.deliveries.is_empty()
    }

    /// Returns pending Deliveries in Runtime-local arrival order.
    pub fn deliveries(&self) -> impl Iterator<Item = &Delivery> {
        self.deliveries.iter()
    }
}
