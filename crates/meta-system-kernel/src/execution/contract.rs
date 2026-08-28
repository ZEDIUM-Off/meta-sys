//! Replaceable execution contract shared by Runner strategies.

use super::RuntimeStart;
use crate::{
    routing::{Delivery, DeliveryProgress},
    system::{ComponentInstanceId, ComponentRuntimeId},
};
use thiserror::Error;

/// Observable result of advancing an execution strategy once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverProgress {
    /// No queued Runtime work was available.
    Idle,
    /// One queued Runtime was advanced.
    Advanced(ComponentInstanceId),
}

/// A failure reported by an [`EventLoopDriver`] lifecycle operation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message}")]
pub struct DriverError {
    /// Human-readable context owned by the adapter boundary.
    message: String,
}

impl DriverError {
    /// Creates a matchable Driver failure with adapter-owned context.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }

    /// Returns the adapter-provided failure context.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// Common contract for starting, advancing, and stopping Component execution.
pub trait EventLoopDriver: std::fmt::Debug {
    /// Starts one dependency-free frontier, potentially concurrently.
    ///
    /// The default adapter preserves deterministic slice order. Drivers may
    /// overlap entries because the frontier declares them independent.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when any Runtime in the frontier cannot start.
    fn start_front(&mut self, starts: &[RuntimeStart]) -> Result<(), DriverError> {
        for start in starts {
            self.start(start.instance(), start.runtime())?;
        }
        Ok(())
    }

    /// Starts one concrete Component Runtime lifecycle.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if execution resources cannot be created.
    fn start(
        &mut self,
        instance_id: ComponentInstanceId,
        runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError>;

    /// Advances at most one queued Component Runtime.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if the selected Runtime cannot be advanced.
    fn advance(&mut self) -> Result<DriverProgress, DriverError>;

    /// Processes one accepted Delivery when the adapter can observe completion.
    ///
    /// The default leaves processing unobserved and the Delivery pending in its
    /// Component Runtime Mailbox.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when observable processing fails.
    fn process_delivery(&mut self, _delivery: &Delivery) -> Result<DeliveryProgress, DriverError> {
        Ok(DeliveryProgress::Unobserved)
    }

    /// Processes one dependency-free Delivery frontier, potentially concurrently.
    ///
    /// The default preserves slice order. Drivers may overlap entries because
    /// the caller declares that their routing work is independent.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] when any Delivery cannot be processed.
    fn process_delivery_front(
        &mut self,
        deliveries: &[Delivery],
    ) -> Result<Vec<DeliveryProgress>, DriverError> {
        deliveries
            .iter()
            .map(|delivery| self.process_delivery(delivery))
            .collect()
    }

    /// Stops one Component Runtime and releases Driver-owned execution state.
    ///
    /// # Errors
    ///
    /// Returns [`DriverError`] if the Runtime cannot be stopped cleanly.
    fn stop(&mut self, instance_id: ComponentInstanceId) -> Result<(), DriverError>;
}
