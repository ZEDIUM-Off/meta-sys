//! Matchable failures produced while evolving a Kernel Runtime.

use crate::{
    CapabilityId, ComponentDefinitionId, ComponentInstanceId, ContextId, DriverError, EffectId,
    FacetId, FacetSchemaId, FacetValueKind, GraphEntityKind, RequirementId,
};
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
    /// The Event referenced a Component Instance absent from this Runtime.
    #[error("Component Instance {0:?} does not exist")]
    UnknownComponentInstance(ComponentInstanceId),
    /// The Event attempted to insert an Effect identity already in the graph.
    #[error("Effect {0:?} already exists")]
    DuplicateEffect(EffectId),
    /// An Effect can only be introduced by an Active lifecycle owner.
    #[error("Component Instance {0:?} is not Active and cannot own a living Effect")]
    InactiveEffectOwner(ComponentInstanceId),
    /// The configured Driver could not stop execution for a deactivating Instance.
    #[error("could not stop Component Instance {instance_id:?}")]
    DriverStop {
        /// Component Instance whose execution failed to stop.
        instance_id: ComponentInstanceId,
        /// Failure reported by the configured Driver.
        #[source]
        error: DriverError,
    },
    /// The Event attempted to insert an existing Context identity.
    #[error("Context {0:?} already exists")]
    DuplicateContext(ContextId),
    /// The Event referenced a Context absent from this Runtime.
    #[error("Context {0:?} does not exist")]
    UnknownContext(ContextId),
    /// The Event attempted to insert an existing Facet Schema identity.
    #[error("Facet Schema {0:?} already exists")]
    DuplicateFacetSchema(FacetSchemaId),
    /// The Event referenced a Facet Schema absent from this Runtime.
    #[error("Facet Schema {0:?} does not exist")]
    UnknownFacetSchema(FacetSchemaId),
    /// The Event attempted to insert an existing Facet identity.
    #[error("Facet {0:?} already exists")]
    DuplicateFacet(FacetId),
    /// Facet data does not match the kind declared by its Schema.
    #[error("Facet value kind mismatch: expected {expected:?}, received {actual:?}")]
    FacetValueMismatch {
        /// Data kind required by the Facet Schema.
        expected: FacetValueKind,
        /// Data kind carried by the rejected Facet.
        actual: FacetValueKind,
    },
    /// Facet target does not match the graph category declared by its Schema.
    #[error("Facet target kind mismatch: expected {expected:?}, received {actual:?}")]
    FacetTargetMismatch {
        /// Graph category accepted by the Facet Schema.
        expected: GraphEntityKind,
        /// Graph category carried by the rejected Facet.
        actual: GraphEntityKind,
    },
    /// A Facet targeted an entity not represented in the current System Graph.
    #[error("Facet target does not exist in the current System Graph")]
    UnknownFacetTarget,
    /// The Runtime exhausted its local Component Runtime identity space.
    #[error("Component Runtime identity space is exhausted")]
    RuntimeIdentityExhausted,
}
