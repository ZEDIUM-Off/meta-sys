//! Private resolution plan derived from the inspectable System Graph.

use crate::{
    Binding, CapabilityContractId, ComponentInstanceId, ComponentRuntime, ComponentRuntimeId,
    ResolutionState, graph::GraphState,
};

/// Complete graph mutation required to activate one ready Component Instance.
#[derive(Debug)]
pub struct ActivationPlan {
    /// Pending Component Instance that can now activate.
    pub(super) instance_id: ComponentInstanceId,
    /// Explicit Bindings satisfying all of the Instance's Requirements.
    pub(super) bindings: Vec<Binding>,
}

impl GraphState {
    /// Returns the next deterministic activation made possible by current providers.
    pub(super) fn next_activation_plan(&self) -> Option<ActivationPlan> {
        self.instances
            .values()
            .find_map(|instance| self.activation_plan(instance.id()))
    }

    /// Applies a successfully started activation to the inspectable graph.
    pub(super) fn apply_activation(
        &mut self,
        plan: &ActivationPlan,
        runtime_id: ComponentRuntimeId,
    ) {
        for binding in &plan.bindings {
            self.bindings
                .insert(binding.requirement_id(), binding.clone());
        }
        if let Some(instance) = self.instances.get_mut(&plan.instance_id) {
            instance.activate();
            self.runtimes.insert(
                plan.instance_id,
                ComponentRuntime::new(runtime_id, plan.instance_id),
            );
        }
    }

    /// Builds an activation only when every necessary Requirement has a provider.
    fn activation_plan(&self, instance_id: ComponentInstanceId) -> Option<ActivationPlan> {
        let instance = self.instances.get(&instance_id)?;
        if instance.resolution() != ResolutionState::Pending {
            return None;
        }
        let definition = self.definitions.get(&instance.definition_id())?;
        let bindings = definition
            .requirements()
            .iter()
            .map(|requirement| {
                self.select_provider(requirement.contract())
                    .map(|(capability_id, provider_id)| {
                        Binding::new(requirement.id(), capability_id, provider_id)
                    })
            })
            .collect::<Option<Vec<_>>>()?;
        Some(ActivationPlan {
            instance_id,
            bindings,
        })
    }

    /// Selects the lowest-identity Capability published by an Active provider.
    fn select_provider(
        &self,
        contract: CapabilityContractId,
    ) -> Option<(crate::CapabilityId, ComponentInstanceId)> {
        self.capabilities
            .iter()
            .find_map(|(capability_id, placement)| {
                let provider = self.instances.get(&placement.provider_id)?;
                (placement.capability.contract() == contract
                    && provider.resolution() == ResolutionState::Active)
                    .then_some((*capability_id, placement.provider_id))
            })
    }
}
