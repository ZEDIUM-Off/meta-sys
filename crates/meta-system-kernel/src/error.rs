//! Matchable failures produced while evolving a Kernel Runtime.

use crate::{CapabilityId, ComponentDefinitionId, ComponentInstanceId, DriverError, RequirementId};
use thiserror::Error;

/// A violation that prevents an Event from changing the System Graph.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KernelError {
    /// The Event attempted to insert an existing Component Definition identity.
    #[error("Component Definition {0:?} already exists")]
    DuplicateComponentDefinition(ComponentDefinitionId),
    /// The Event attempted to insert an existing Component Instance identity.
    #[error("Component Instance {0:?} already exists")]
    DuplicateComponentInstance(ComponentInstanceId),
    /// The Event attempted to insert a Requirement identity already in the graph.
    #[error("Requirement {0:?} already exists")]
    DuplicateRequirement(RequirementId),
    /// The Event attempted to insert a Capability identity already in the graph.
    #[error("Capability {0:?} already exists")]
    DuplicateCapability(CapabilityId),
    /// The configured Driver could not start execution for an activating Instance.
    #[error("could not start Component Instance {instance_id:?}")]
    DriverStart {
        /// Component Instance whose execution failed to start.
        instance_id: ComponentInstanceId,
        /// Failure reported by the configured Driver.
        #[source]
        error: DriverError,
    },
}
