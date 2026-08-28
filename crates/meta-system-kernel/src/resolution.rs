//! Affected activation planning derived from one graph mutation.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Binding, CapabilityContractId, ComponentInstanceId, ComponentRuntime, ComponentRuntimeId,
    ExecutionFront, ExecutionPlan, ExecutionWork, ResolutionState, graph::GraphState,
};

/// Complete graph mutation required to activate one ready Component Instance.
#[derive(Debug)]
pub struct ActivationPlan {
    /// Pending Component Instance that can now activate.
    pub(super) instance_id: ComponentInstanceId,
    /// Explicit Bindings satisfying all of the Instance's Requirements.
    pub(super) bindings: Vec<Binding>,
    /// Affected provider activations that must complete first.
    dependencies: Vec<ComponentInstanceId>,
}

/// Ordered dependency fronts affected by one registration mutation.
#[derive(Debug)]
pub struct ActivationExecutionPlan {
    /// Deterministic fronts whose entries are mutually independent.
    pub(super) fronts: Vec<Vec<ActivationPlan>>,
}

impl ActivationExecutionPlan {
    /// Converts private Binding plans into the inspectable scheduling contract.
    pub(super) fn inspectable(&self) -> ExecutionPlan {
        let fronts = self
            .fronts
            .iter()
            .map(|front| {
                ExecutionFront::new(
                    front
                        .iter()
                        .map(|plan| ExecutionWork::new(plan.instance_id, plan.dependencies.clone()))
                        .collect(),
                )
            })
            .collect();
        ExecutionPlan::new(fronts)
    }
}

impl GraphState {
    /// Plans only pending work reachable from the registered Component Instance.
    pub(super) fn affected_activation_plan(
        &self,
        seed: ComponentInstanceId,
    ) -> ActivationExecutionPlan {
        let candidates = self.affected_candidates(seed);
        let plans = candidates
            .iter()
            .filter_map(|instance_id| {
                self.activation_plan(*instance_id, &candidates)
                    .map(|plan| (*instance_id, plan))
            })
            .collect::<BTreeMap<_, _>>();
        ActivationExecutionPlan {
            fronts: Self::ordered_fronts(plans),
        }
    }

    /// Applies one successfully started activation to the inspectable graph.
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

    /// Computes the pending consumer closure reachable from changed Capabilities.
    fn affected_candidates(&self, seed: ComponentInstanceId) -> BTreeSet<ComponentInstanceId> {
        let mut candidates = BTreeSet::from([seed]);
        loop {
            let contracts = self
                .capabilities
                .values()
                .filter(|placement| candidates.contains(&placement.provider_id))
                .map(|placement| placement.capability.contract())
                .collect::<BTreeSet<_>>();
            let next = self.instances.values().find_map(|instance| {
                let definition = self.definitions.get(&instance.definition_id())?;
                (instance.resolution() == ResolutionState::Pending
                    && !candidates.contains(&instance.id())
                    && definition
                        .requirements()
                        .iter()
                        .any(|requirement| contracts.contains(&requirement.contract())))
                .then_some(instance.id())
            });
            let Some(instance_id) = next else {
                break;
            };
            candidates.insert(instance_id);
        }
        candidates
    }

    /// Builds one activation when every Requirement has an eligible provider.
    fn activation_plan(
        &self,
        instance_id: ComponentInstanceId,
        candidates: &BTreeSet<ComponentInstanceId>,
    ) -> Option<ActivationPlan> {
        let instance = self.instances.get(&instance_id)?;
        let definition = self.definitions.get(&instance.definition_id())?;
        let bindings = definition
            .requirements()
            .iter()
            .map(|requirement| {
                self.select_provider(requirement.contract(), candidates)
                    .map(|(capability_id, provider_id)| {
                        Binding::new(requirement.id(), capability_id, provider_id)
                    })
            })
            .collect::<Option<Vec<_>>>()?;
        let dependencies = bindings
            .iter()
            .map(Binding::provider_id)
            .filter(|provider_id| candidates.contains(provider_id))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Some(ActivationPlan {
            instance_id,
            bindings,
            dependencies,
        })
    }

    /// Selects an Active provider first, then an affected provider candidate.
    fn select_provider(
        &self,
        contract: CapabilityContractId,
        candidates: &BTreeSet<ComponentInstanceId>,
    ) -> Option<(crate::CapabilityId, ComponentInstanceId)> {
        self.select_active_provider(contract)
            .or_else(|| self.select_candidate_provider(contract, candidates))
    }

    /// Selects the lowest-identity matching Capability from an Active provider.
    fn select_active_provider(
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

    /// Selects the lowest-identity matching Capability from affected work.
    fn select_candidate_provider(
        &self,
        contract: CapabilityContractId,
        candidates: &BTreeSet<ComponentInstanceId>,
    ) -> Option<(crate::CapabilityId, ComponentInstanceId)> {
        self.capabilities
            .iter()
            .find(|(_, placement)| {
                placement.capability.contract() == contract
                    && candidates.contains(&placement.provider_id)
            })
            .map(|(capability_id, placement)| (*capability_id, placement.provider_id))
    }

    /// Topologically groups ready work into deterministic independent fronts.
    fn ordered_fronts(
        plans: BTreeMap<ComponentInstanceId, ActivationPlan>,
    ) -> Vec<Vec<ActivationPlan>> {
        let mut remaining = plans;
        let mut completed = BTreeSet::new();
        let mut fronts = Vec::new();
        loop {
            let ready = remaining
                .iter()
                .filter(|(_, plan)| plan.dependencies.iter().all(|id| completed.contains(id)))
                .map(|(instance_id, _)| *instance_id)
                .collect::<Vec<_>>();
            if ready.is_empty() {
                break;
            }
            fronts.push(Self::take_ready(&mut remaining, &mut completed, ready));
        }
        fronts
    }

    /// Removes one ready frontier and marks its work complete for later fronts.
    fn take_ready(
        remaining: &mut BTreeMap<ComponentInstanceId, ActivationPlan>,
        completed: &mut BTreeSet<ComponentInstanceId>,
        ready: Vec<ComponentInstanceId>,
    ) -> Vec<ActivationPlan> {
        ready
            .into_iter()
            .filter_map(|instance_id| {
                let plan = remaining.remove(&instance_id)?;
                completed.insert(instance_id);
                Some(plan)
            })
            .collect()
    }
}
