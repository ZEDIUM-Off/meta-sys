//! Cleanup plan for removing a Component Instance and affected Bindings.

use std::collections::BTreeSet;

use crate::{
    Binding, ComponentDefinitionId, ComponentInstanceId, EffectId, KernelError, ResolutionState,
    graph::GraphState,
};

/// Cleanup required for one consumer bound to a disappearing provider.
#[derive(Debug)]
pub struct ConsumerDeactivation {
    /// Active consumer that must return to Pending.
    pub(super) instance_id: ComponentInstanceId,
    /// Bindings to the disappearing provider.
    pub(super) bindings: Vec<Binding>,
    /// Effects governed by the consumer lifecycle.
    pub(super) effects: Vec<EffectId>,
}

/// Complete cleanup required before one Instance can disappear.
#[derive(Debug)]
pub struct RemovalPlan {
    /// Component Instance removed by the Event.
    pub(super) instance_id: ComponentInstanceId,
    /// Definition owned by the removed occurrence in the prototype graph.
    definition_id: ComponentDefinitionId,
    /// Stable resolution state observed before removal.
    pub(super) previous: ResolutionState,
    /// Consumers that depend on Capabilities published by the removed Instance.
    pub(super) consumers: Vec<ConsumerDeactivation>,
    /// Bindings owned by Requirements of the removed Instance itself.
    pub(super) own_bindings: Vec<Binding>,
    /// Effects governed by the removed Instance lifecycle.
    pub(super) effects: Vec<EffectId>,
}

impl GraphState {
    /// Derives all Driver stops and graph cleanup caused by Instance removal.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError::UnknownComponentInstance`] when the target is absent.
    pub(super) fn removal_plan(
        &self,
        instance_id: ComponentInstanceId,
    ) -> Result<RemovalPlan, KernelError> {
        let instance = self
            .instances
            .get(&instance_id)
            .ok_or(KernelError::UnknownComponentInstance(instance_id))?;
        let definition_id = instance.definition_id();
        Ok(RemovalPlan {
            instance_id,
            definition_id,
            previous: instance.resolution(),
            consumers: self.affected_consumers(instance_id),
            own_bindings: self.bindings_for_definition(definition_id),
            effects: self.effect_ids(instance_id),
        })
    }

    /// Applies a cleanup plan after every required Driver stop succeeds.
    pub(super) fn apply_removal(&mut self, plan: &RemovalPlan) {
        for consumer in &plan.consumers {
            self.remove_consumer_lifecycle(consumer);
        }
        for binding in &plan.own_bindings {
            self.bindings.remove(&binding.requirement_id());
        }
        for effect_id in &plan.effects {
            self.effects.remove(effect_id);
        }
        self.runtimes.remove(&plan.instance_id);
        self.instances.remove(&plan.instance_id);
        if let Some(definition) = self.definitions.remove(&plan.definition_id) {
            for requirement in definition.requirements() {
                self.requirements.remove(&requirement.id());
            }
            for capability in definition.capabilities() {
                self.capabilities.remove(&capability.id());
            }
        }
    }

    /// Groups Bindings by consumers selected against the disappearing provider.
    fn affected_consumers(&self, provider_id: ComponentInstanceId) -> Vec<ConsumerDeactivation> {
        let affected = self.affected_instance_ids(provider_id);
        let mut unavailable_providers = affected.clone();
        unavailable_providers.insert(provider_id);
        affected
            .into_iter()
            .map(|instance_id| ConsumerDeactivation {
                instance_id,
                bindings: self
                    .bindings
                    .values()
                    .filter(|binding| {
                        self.consumer_for(binding) == Some(instance_id)
                            && unavailable_providers.contains(&binding.provider_id())
                    })
                    .cloned()
                    .collect(),
                effects: self.effect_ids(instance_id),
            })
            .collect()
    }

    /// Computes the transitive closure of consumers losing an Active provider.
    fn affected_instance_ids(
        &self,
        root_provider: ComponentInstanceId,
    ) -> BTreeSet<ComponentInstanceId> {
        let mut unavailable_providers = BTreeSet::from([root_provider]);
        let mut affected = BTreeSet::new();
        loop {
            let next = self.bindings.values().find_map(|binding| {
                let consumer_id = self.consumer_for(binding)?;
                (unavailable_providers.contains(&binding.provider_id())
                    && consumer_id != root_provider
                    && !affected.contains(&consumer_id))
                .then_some(consumer_id)
            });
            let Some(consumer_id) = next else {
                break;
            };
            affected.insert(consumer_id);
            unavailable_providers.insert(consumer_id);
        }
        affected
    }

    /// Finds the living Instance whose Definition owns a Requirement Binding.
    fn consumer_for(&self, binding: &Binding) -> Option<ComponentInstanceId> {
        self.instances.values().find_map(|instance| {
            let definition = self.definitions.get(&instance.definition_id())?;
            definition
                .requirements()
                .iter()
                .any(|requirement| requirement.id() == binding.requirement_id())
                .then_some(instance.id())
        })
    }

    /// Collects Bindings owned by Requirements of one Definition.
    fn bindings_for_definition(&self, definition_id: ComponentDefinitionId) -> Vec<Binding> {
        self.definitions
            .get(&definition_id)
            .into_iter()
            .flat_map(crate::ComponentDefinition::requirements)
            .filter_map(|requirement| self.bindings.get(&requirement.id()).cloned())
            .collect()
    }

    /// Collects Effects governed by one Component Instance lifecycle.
    fn effect_ids(&self, owner_id: ComponentInstanceId) -> Vec<EffectId> {
        self.effects
            .values()
            .filter(|effect| effect.owner() == owner_id)
            .map(crate::Effect::id)
            .collect()
    }

    /// Removes a consumer's Binding, Runtime, and lifecycle-owned Effects.
    fn remove_consumer_lifecycle(&mut self, consumer: &ConsumerDeactivation) {
        for binding in &consumer.bindings {
            self.bindings.remove(&binding.requirement_id());
        }
        for effect_id in &consumer.effects {
            self.effects.remove(effect_id);
        }
        self.runtimes.remove(&consumer.instance_id);
        if let Some(instance) = self.instances.get_mut(&consumer.instance_id) {
            instance.deactivate();
        }
    }
}
