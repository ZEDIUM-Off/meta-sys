//! Neutral graph-entity categories eligible for Facet attachment.

use super::{
    CapabilityId, ComponentDefinitionId, ComponentInstanceId, ComponentRuntimeId, ContextId,
    EffectId, RequirementId,
};

/// Neutral category of an entity represented in the System Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphEntityKind {
    /// Complete Component declaration.
    ComponentDefinition,
    /// Living Component occurrence.
    ComponentInstance,
    /// Capability need.
    Requirement,
    /// Capability offer.
    Capability,
    /// Concrete execution lifecycle.
    ComponentRuntime,
    /// Living lifecycle-owned consequence.
    Effect,
    /// Structural visibility and lifecycle scope.
    Context,
}

/// Strongly typed target of a Facet attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacetTarget {
    /// Targets a Component Definition.
    ComponentDefinition(ComponentDefinitionId),
    /// Targets a Component Instance.
    ComponentInstance(ComponentInstanceId),
    /// Targets a Requirement.
    Requirement(RequirementId),
    /// Targets a Capability.
    Capability(CapabilityId),
    /// Targets a Component Runtime.
    ComponentRuntime(ComponentRuntimeId),
    /// Targets an Effect.
    Effect(EffectId),
    /// Targets a Context.
    Context(ContextId),
}

impl FacetTarget {
    /// Returns the neutral graph category addressed by this target.
    #[must_use]
    pub const fn kind(self) -> GraphEntityKind {
        match self {
            Self::ComponentDefinition(_) => GraphEntityKind::ComponentDefinition,
            Self::ComponentInstance(_) => GraphEntityKind::ComponentInstance,
            Self::Requirement(_) => GraphEntityKind::Requirement,
            Self::Capability(_) => GraphEntityKind::Capability,
            Self::ComponentRuntime(_) => GraphEntityKind::ComponentRuntime,
            Self::Effect(_) => GraphEntityKind::Effect,
            Self::Context(_) => GraphEntityKind::Context,
        }
    }
}
