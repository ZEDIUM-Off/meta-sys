//! Storage and read-only observation of a Runtime's single System Graph.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Binding, Capability, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstance, ComponentInstanceId, ComponentRuntime, KernelError, Requirement,
    RequirementId,
};

/// A declared Capability together with the Instance that publishes it.
#[derive(Debug)]
pub struct CapabilityPlacement {
    /// Inspectable offer contributed by the provider Definition.
    pub(super) capability: Capability,
    /// Living Component Instance that publishes the offer.
    pub(super) provider_id: ComponentInstanceId,
}

/// Owned mutable graph state private to one Kernel Runtime.
#[derive(Debug, Default)]
pub struct GraphState {
    /// Complete Component declarations indexed by identity.
    pub(super) definitions: BTreeMap<ComponentDefinitionId, ComponentDefinition>,
    /// Living Component occurrences indexed by identity.
    pub(super) instances: BTreeMap<ComponentInstanceId, ComponentInstance>,
    /// Requirements contributed by every declaration.
    pub(super) requirements: BTreeMap<RequirementId, Requirement>,
    /// Capability offers and their publishing Component Instances.
    pub(super) capabilities: BTreeMap<CapabilityId, CapabilityPlacement>,
    /// Explicit resolution relations indexed by Requirement identity.
    pub(super) bindings: BTreeMap<RequirementId, Binding>,
    /// Living execution state indexed by Component Instance identity.
    pub(super) runtimes: BTreeMap<ComponentInstanceId, ComponentRuntime>,
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
        for capability in definition.capabilities().iter().cloned() {
            self.capabilities.insert(
                capability.id(),
                CapabilityPlacement {
                    capability,
                    provider_id: instance_id,
                },
            );
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
        let mut event_capability_ids = BTreeSet::new();
        for capability in definition.capabilities() {
            let id = capability.id();
            if self.capabilities.contains_key(&id) || !event_capability_ids.insert(id) {
                return Err(KernelError::DuplicateCapability(id));
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
}
