//! Storage and read-only observation of a Runtime's single System Graph.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Binding, Capability, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstance, ComponentInstanceId, ComponentRuntime, Context, ContextId, Effect, EffectId,
    Facet, FacetId, FacetSchema, FacetSchemaId, KernelError, Requirement, RequirementId,
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
    /// Living Effects indexed by identity and governed by their owner lifecycle.
    pub(super) effects: BTreeMap<EffectId, Effect>,
    /// Structural visibility and lifecycle scopes indexed by identity.
    pub(super) contexts: BTreeMap<ContextId, Context>,
    /// Addon-owned Facet contracts indexed by identity.
    pub(super) facet_schemas: BTreeMap<FacetSchemaId, FacetSchema>,
    /// Typed Facet attachments indexed by identity.
    pub(super) facets: BTreeMap<FacetId, Facet>,
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
