//! Deterministic sequential reference adapter for the Driver seam.

use std::collections::{BTreeSet, VecDeque};

use crate::system::{ComponentInstanceId, ComponentRuntimeId};

use super::{DriverError, DriverProgress, EventLoopDriver};

/// Deterministic reference Driver that advances queued Instances one at a time.
#[derive(Debug, Default)]
pub struct SequentialExecutor {
    /// Instances with live Driver-owned execution state.
    active: BTreeSet<ComponentInstanceId>,
    /// FIFO frontier of Instances ready for one advancement.
    ready: VecDeque<ComponentInstanceId>,
}

impl EventLoopDriver for SequentialExecutor {
    fn start(
        &mut self,
        instance_id: ComponentInstanceId,
        _runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError> {
        if !self.active.insert(instance_id) {
            return Err(DriverError::new("Component Instance is already started"));
        }
        self.ready.push_back(instance_id);
        Ok(())
    }

    fn advance(&mut self) -> Result<DriverProgress, DriverError> {
        Ok(self
            .ready
            .pop_front()
            .map_or(DriverProgress::Idle, DriverProgress::Advanced))
    }

    fn stop(&mut self, instance_id: ComponentInstanceId) -> Result<(), DriverError> {
        self.active.remove(&instance_id);
        self.ready.retain(|queued| *queued != instance_id);
        Ok(())
    }
}
