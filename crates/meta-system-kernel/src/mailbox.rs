//! Bounded Delivery queue owned by one concrete Component Runtime.

use std::collections::VecDeque;

use crate::{Delivery, QueueCapacity};

/// Bounded aggregate of Deliveries received from subscribed Rooms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mailbox {
    /// Component-declared maximum number of pending Deliveries.
    capacity: QueueCapacity,
    /// Pending Deliveries in their Runtime-local arrival order.
    deliveries: VecDeque<Delivery>,
}

impl Mailbox {
    /// Creates an empty Mailbox for one new Component Runtime lifecycle.
    #[must_use]
    pub(crate) const fn new(capacity: QueueCapacity) -> Self {
        Self {
            capacity,
            deliveries: VecDeque::new(),
        }
    }

    /// Attempts one Delivery without retrying or exceeding the explicit bound.
    pub(crate) fn deliver(&mut self, delivery: Delivery) -> bool {
        if self.deliveries.len() == self.capacity.get() {
            return false;
        }
        self.deliveries.push_back(delivery);
        true
    }

    /// Removes a Delivery synchronously confirmed as processed by the Driver.
    pub(crate) fn mark_processed(&mut self, processed: &Delivery) {
        if self
            .deliveries
            .front()
            .is_some_and(|delivery| delivery == processed)
        {
            self.deliveries.pop_front();
        }
    }

    /// Returns the explicit pending Delivery bound.
    #[must_use]
    pub const fn capacity(&self) -> QueueCapacity {
        self.capacity
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
