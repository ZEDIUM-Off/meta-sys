//! Cleanup plan for removing a Component Instance and affected Bindings.

use std::collections::BTreeSet;

use crate::{
    Binding, ComponentDefinitionId, ComponentInstanceId, ContextId, ContextOwner, EffectId,
    FacetId, FacetTarget, KernelError, ResolutionState, graph::GraphState,
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
    /// Contexts governed directly or structurally by the removed Instance.
    contexts: Vec<ContextId>,
    /// Facets contained by or targeting removed lifecycle entities.
    facets: Vec<FacetId>,
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
        let consumers = self.affected_consumers(instance_id);
        let effects = self.effect_ids(instance_id);
        let contexts = self.context_ids(instance_id);
        let facets = self.facet_ids_for_removal(instance_id, &consumers, &contexts, &effects);
        Ok(RemovalPlan {
            instance_id,
            definition_id,
            previous: instance.resolution(),
            consumers,
            own_bindings: self.bindings_for_definition(definition_id),
            effects,
            contexts,
            facets,
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
        for facet_id in &plan.facets {
            self.facets.remove(facet_id);
        }
        for context_id in &plan.contexts {
            self.contexts.remove(context_id);
        }
        self.deactivate_routing(plan.instance_id);
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

    /// Collects Component-owned Contexts and all structurally nested scopes.
    fn context_ids(&self, owner_id: ComponentInstanceId) -> Vec<ContextId> {
        let mut contexts = self
            .contexts
            .values()
            .filter(|context| context.owner() == ContextOwner::Component(owner_id))
            .map(crate::Context::id)
            .collect::<BTreeSet<_>>();
        loop {
            let nested = self.contexts.values().find_map(|context| {
                context
                    .parent()
                    .is_some_and(|parent| contexts.contains(&parent))
                    .then_some(context.id())
                    .filter(|id| !contexts.contains(id))
            });
            let Some(context_id) = nested else {
                break;
            };
            contexts.insert(context_id);
        }
        contexts.into_iter().collect()
    }

    /// Collects Facets contained by or targeting resources removed by the plan.
    fn facet_ids_for_removal(
        &self,
        instance_id: ComponentInstanceId,
        consumers: &[ConsumerDeactivation],
        contexts: &[ContextId],
        effects: &[EffectId],
    ) -> Vec<FacetId> {
        let affected_instances = consumers
            .iter()
            .map(|consumer| consumer.instance_id)
            .chain([instance_id])
            .collect::<BTreeSet<_>>();
        let runtime_ids = self
            .runtimes
            .values()
            .filter(|runtime| affected_instances.contains(&runtime.instance_id()))
            .map(crate::ComponentRuntime::id)
            .collect::<BTreeSet<_>>();
        let effect_ids = consumers
            .iter()
            .flat_map(|consumer| consumer.effects.iter().copied())
            .chain(effects.iter().copied())
            .collect::<BTreeSet<_>>();
        let context_ids = contexts.iter().copied().collect::<BTreeSet<_>>();
        self.facets
            .values()
            .filter(|facet| {
                context_ids.contains(&facet.context())
                    || match facet.target() {
                        FacetTarget::ComponentInstance(id) => affected_instances.contains(&id),
                        FacetTarget::ComponentRuntime(id) => runtime_ids.contains(&id),
                        FacetTarget::Effect(id) => effect_ids.contains(&id),
                        FacetTarget::Context(id) => context_ids.contains(&id),
                        FacetTarget::ComponentDefinition(_)
                        | FacetTarget::Requirement(_)
                        | FacetTarget::Capability(_) => false,
                    }
            })
            .map(crate::Facet::id)
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
        self.deactivate_routing(consumer.instance_id);
        self.runtimes.remove(&consumer.instance_id);
        if let Some(instance) = self.instances.get_mut(&consumer.instance_id) {
            instance.deactivate();
        }
    }
}
