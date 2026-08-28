//! Matchable failures produced while evolving a Kernel Runtime.

use crate::{ComponentDefinitionId, ComponentInstanceId, RequirementId};
use thiserror::Error;

/// A violation that prevents an Event from changing the System Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
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
}
