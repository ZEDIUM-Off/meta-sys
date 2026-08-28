//! Read-only observation of one Runtime's current System Graph.

use crate::{
    Binding, Capability, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstance, ComponentInstanceId, ComponentRuntime, Context, ContextId, Effect, EffectId,
    Facet, FacetId, FacetSchema, FacetSchemaId, Requirement, RequirementId, graph::GraphState,
};

/// A read-only observation of one Kernel Runtime's current System Graph.
#[derive(Debug, Clone, Copy)]
#[must_use = "a System Graph view must be queried to observe runtime state"]
pub struct SystemGraph<'graph> {
    /// Runtime-owned graph state borrowed for this observation.
    state: &'graph GraphState,
}

impl<'graph> SystemGraph<'graph> {
    /// Creates an observation tied to one Runtime's graph state.
    pub(crate) const fn new(state: &'graph GraphState) -> Self {
        Self { state }
    }

    /// Finds a complete Component Definition by identity.
    #[must_use]
    pub fn definition(&self, id: ComponentDefinitionId) -> Option<&ComponentDefinition> {
        self.state.definitions.get(&id)
    }

    /// Finds a living Component Instance by identity.
    #[must_use]
    pub fn instance(&self, id: ComponentInstanceId) -> Option<&ComponentInstance> {
        self.state.instances.get(&id)
    }

    /// Finds an inspectable Requirement by identity.
    #[must_use]
    pub fn requirement(&self, id: RequirementId) -> Option<&Requirement> {
        self.state.requirements.get(&id)
    }

    /// Finds an inspectable Capability by identity.
    #[must_use]
    pub fn capability(&self, id: CapabilityId) -> Option<&Capability> {
        self.state
            .capabilities
            .get(&id)
            .map(|placement| &placement.capability)
    }

    /// Finds the explicit Binding resolving a Requirement, when one exists.
    #[must_use]
    pub fn binding(&self, id: RequirementId) -> Option<&Binding> {
        self.state.bindings.get(&id)
    }

    /// Finds the living execution attached to an Active Component Instance.
    #[must_use]
    pub fn component_runtime(&self, id: ComponentInstanceId) -> Option<&ComponentRuntime> {
        self.state.runtimes.get(&id)
    }

    /// Finds a living lifecycle-owned Effect by identity.
    #[must_use]
    pub fn effect(&self, id: EffectId) -> Option<&Effect> {
        self.state.effects.get(&id)
    }

    /// Finds a structural Context by identity.
    #[must_use]
    pub fn context(&self, id: ContextId) -> Option<&Context> {
        self.state.contexts.get(&id)
    }

    /// Finds an Addon-owned Facet Schema by identity.
    #[must_use]
    pub fn facet_schema(&self, id: FacetSchemaId) -> Option<&FacetSchema> {
        self.state.facet_schemas.get(&id)
    }

    /// Finds a typed Facet attachment by identity.
    #[must_use]
    pub fn facet(&self, id: FacetId) -> Option<&Facet> {
        self.state.facets.get(&id)
    }
}
