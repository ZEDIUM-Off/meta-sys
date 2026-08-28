//! Storage and read-only observation of a Runtime's single System Graph.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Binding, ComponentDefinition, ComponentDefinitionId, ComponentInstance, ComponentInstanceId,
    ComponentRuntime, KernelError, Requirement, RequirementId,
};

/// Owned mutable graph state private to one Kernel Runtime.
#[derive(Debug, Default)]
pub struct GraphState {
    /// Complete Component declarations indexed by identity.
    definitions: BTreeMap<ComponentDefinitionId, ComponentDefinition>,
    /// Living Component occurrences indexed by identity.
    instances: BTreeMap<ComponentInstanceId, ComponentInstance>,
    /// Requirements contributed by every declaration.
    requirements: BTreeMap<RequirementId, Requirement>,
    /// Explicit resolution relations indexed by Requirement identity.
    bindings: BTreeMap<RequirementId, Binding>,
    /// Living execution state indexed by Component Instance identity.
    runtimes: BTreeMap<ComponentInstanceId, ComponentRuntime>,
}

impl GraphState {
    /// Inserts a complete declaration and an unresolved living occurrence atomically.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] before mutation when any contributed identity is
    /// already present or repeated within the Event.
    pub fn register_pending(
        &mut self,
        definition: ComponentDefinition,
        instance_id: ComponentInstanceId,
    ) -> Result<(), KernelError> {
        self.validate_registration(&definition, instance_id)?;
        let definition_id = definition.id();
        for requirement in definition.requirements().iter().cloned() {
            self.requirements.insert(requirement.id(), requirement);
        }
        self.definitions.insert(definition_id, definition);
        self.instances.insert(
            instance_id,
            ComponentInstance::pending(instance_id, definition_id),
        );
        Ok(())
    }

    /// Checks every identity contributed by a registration before graph mutation.
    fn validate_registration(
        &self,
        definition: &ComponentDefinition,
        instance_id: ComponentInstanceId,
    ) -> Result<(), KernelError> {
        if self.definitions.contains_key(&definition.id()) {
            return Err(KernelError::DuplicateComponentDefinition(definition.id()));
        }
        if self.instances.contains_key(&instance_id) {
            return Err(KernelError::DuplicateComponentInstance(instance_id));
        }
        let mut event_requirement_ids = BTreeSet::new();
        for requirement in definition.requirements() {
            let id = requirement.id();
            if self.requirements.contains_key(&id) || !event_requirement_ids.insert(id) {
                return Err(KernelError::DuplicateRequirement(id));
            }
        }
        Ok(())
    }
}

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
}
