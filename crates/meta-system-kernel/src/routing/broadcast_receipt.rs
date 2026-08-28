//! Inspectable listener outcomes returned by one broadcast operation.

use super::DeliveryReceipt;

/// Complete observation returned after one broadcast distribution.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "Broadcast Receipts expose every listener Delivery outcome"]
pub struct BroadcastReceipt {
    /// Listener-local Delivery outcomes in deterministic Component order.
    deliveries: Vec<DeliveryReceipt>,
}

impl BroadcastReceipt {
    /// Records every listener-local outcome of one broadcast Event.
    pub(crate) const fn new(deliveries: Vec<DeliveryReceipt>) -> Self {
        Self { deliveries }
    }

    /// Returns every registered listener Delivery outcome.
    #[must_use]
    pub fn deliveries(&self) -> &[DeliveryReceipt] {
        &self.deliveries
    }
}
