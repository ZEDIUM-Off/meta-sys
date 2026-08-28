//! Affected activation planning derived from one graph mutation.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Binding, BindingCandidate, BindingHook, BindingProposal, CapabilityContractId,
    ComponentInstanceId, ComponentRuntime, ComponentRuntimeId, ExecutionFront, ExecutionPlan,
    ExecutionWork, KernelError, Requirement, ResolutionState,
    binding_policy::evaluate_binding_hooks, graph::GraphState,
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
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when an active Binding hook rejects or returns an
    /// incompatible provider selection.
    pub(super) fn affected_activation_plan(
        &self,
        seed: ComponentInstanceId,
        hooks: &[Box<dyn BindingHook>],
    ) -> Result<ActivationExecutionPlan, KernelError> {
        let candidates = self.affected_candidates(seed);
        let mut plans = BTreeMap::new();
        for instance_id in &candidates {
            if let Some(plan) = self.activation_plan(*instance_id, &candidates, hooks)? {
                plans.insert(*instance_id, plan);
            }
        }
        Ok(ActivationExecutionPlan {
            fronts: Self::ordered_fronts(plans),
        })
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
        let mailbox_policy = self
            .instances
            .get(&plan.instance_id)
            .and_then(|instance| self.definitions.get(&instance.definition_id()))
            .map_or_else(crate::MailboxPolicy::default, |definition| {
                definition.routing().mailbox_policy()
            });
        if let Some(instance) = self.instances.get_mut(&plan.instance_id) {
            instance.activate();
            self.runtimes.insert(
                plan.instance_id,
                ComponentRuntime::with_mailbox(runtime_id, plan.instance_id, mailbox_policy),
            );
            self.activate_routing(plan.instance_id, runtime_id);
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

    /// Builds one activation when every Requirement has an admitted provider.
    fn activation_plan(
        &self,
        instance_id: ComponentInstanceId,
        candidates: &BTreeSet<ComponentInstanceId>,
        hooks: &[Box<dyn BindingHook>],
    ) -> Result<Option<ActivationPlan>, KernelError> {
        let Some(instance) = self.instances.get(&instance_id) else {
            return Ok(None);
        };
        let Some(definition) = self.definitions.get(&instance.definition_id()) else {
            return Ok(None);
        };
        let mut bindings = Vec::with_capacity(definition.requirements().len());
        for requirement in definition.requirements() {
            let Some(binding) = self.binding_for_requirement(requirement, candidates, hooks)?
            else {
                return Ok(None);
            };
            bindings.push(binding);
        }
        let dependencies = bindings
            .iter()
            .map(Binding::provider_id)
            .filter(|provider_id| candidates.contains(provider_id))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        Ok(Some(ActivationPlan {
            instance_id,
            bindings,
            dependencies,
        }))
    }

    /// Passes compatible providers through the ordered Addon Binding seam.
    fn binding_for_requirement(
        &self,
        requirement: &Requirement,
        candidates: &BTreeSet<ComponentInstanceId>,
        hooks: &[Box<dyn BindingHook>],
    ) -> Result<Option<Binding>, KernelError> {
        let compatible = self.compatible_providers(requirement.contract(), candidates);
        let active_default = compatible.iter().find(|candidate| {
            self.instances
                .get(&candidate.provider())
                .is_some_and(|provider| provider.resolution() == ResolutionState::Active)
        });
        let Some(default) = active_default
            .copied()
            .or_else(|| compatible.first().copied())
        else {
            return Ok(None);
        };
        let proposal = BindingProposal::new(
            requirement.id(),
            requirement.contract(),
            compatible,
            default.capability(),
        );
        let selected = evaluate_binding_hooks(hooks, proposal, default)?;
        Ok(Some(Binding::new(
            requirement.id(),
            selected.capability(),
            selected.provider(),
        )))
    }

    /// Collects all matching Active or affected provider candidates.
    fn compatible_providers(
        &self,
        contract: CapabilityContractId,
        candidates: &BTreeSet<ComponentInstanceId>,
    ) -> Vec<BindingCandidate> {
        self.capabilities
            .iter()
            .filter_map(|(capability_id, placement)| {
                let provider = self.instances.get(&placement.provider_id)?;
                (placement.capability.contract() == contract
                    && (provider.resolution() == ResolutionState::Active
                        || candidates.contains(&provider.id())))
                .then_some(BindingCandidate::new(*capability_id, placement.provider_id))
            })
            .collect()
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
