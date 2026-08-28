//! Observable explanations returned after Kernel state transitions.

use super::LifecycleTransition;
use crate::{
    execution::ExecutionPlan,
    resolution::Binding,
    system::{ComponentInstanceId, EffectId, ResolutionState},
};

/// Observable graph and lifecycle changes produced by one accepted Event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "transition outcomes explain the graph changes caused by an Event"]
pub struct TransitionOutcome {
    /// Stable lifecycle transitions in execution order.
    transitions: Vec<LifecycleTransition>,
    /// Bindings created while resolving affected Instances.
    created_bindings: Vec<Binding>,
    /// Bindings removed while deactivating affected Instances.
    removed_bindings: Vec<Binding>,
    /// Effects removed with their owning lifecycle.
    removed_effects: Vec<EffectId>,
    /// Dependency fronts executed while processing the Event.
    execution_plan: ExecutionPlan,
}

impl TransitionOutcome {
    /// Creates an accepted outcome with no lifecycle or Binding change.
    pub(crate) const fn empty() -> Self {
        Self {
            transitions: Vec::new(),
            created_bindings: Vec::new(),
            removed_bindings: Vec::new(),
            removed_effects: Vec::new(),
            execution_plan: ExecutionPlan::new(Vec::new()),
        }
    }

    /// Creates the initial Pending transition for a registered Instance.
    pub(crate) fn registered_pending(instance_id: ComponentInstanceId) -> Self {
        Self {
            transitions: vec![LifecycleTransition::new(
                instance_id,
                None,
                Some(ResolutionState::Pending),
            )],
            created_bindings: Vec::new(),
            removed_bindings: Vec::new(),
            removed_effects: Vec::new(),
            execution_plan: ExecutionPlan::default(),
        }
    }

    /// Records one completed activation and the Bindings that enabled it.
    pub(crate) fn record_activation(
        &mut self,
        instance_id: ComponentInstanceId,
        bindings: &[Binding],
    ) {
        self.transitions.push(LifecycleTransition::new(
            instance_id,
            Some(ResolutionState::Pending),
            Some(ResolutionState::Active),
        ));
        self.created_bindings.extend_from_slice(bindings);
    }

    /// Records cleanup that returned one consumer from Active to Pending.
    pub(crate) fn record_deactivation(
        &mut self,
        instance_id: ComponentInstanceId,
        bindings: &[Binding],
        effects: &[EffectId],
    ) {
        self.transitions.push(LifecycleTransition::new(
            instance_id,
            Some(ResolutionState::Active),
            Some(ResolutionState::Pending),
        ));
        self.removed_bindings.extend_from_slice(bindings);
        self.removed_effects.extend_from_slice(effects);
    }

    /// Records disappearance of one Component Instance after cleanup.
    pub(crate) fn record_removal(
        &mut self,
        instance_id: ComponentInstanceId,
        previous: ResolutionState,
        bindings: &[Binding],
        effects: &[EffectId],
    ) {
        self.transitions
            .push(LifecycleTransition::new(instance_id, Some(previous), None));
        self.removed_bindings.extend_from_slice(bindings);
        self.removed_effects.extend_from_slice(effects);
    }

    /// Records the affected dependency plan executed for this Event.
    pub(crate) fn set_execution_plan(&mut self, execution_plan: ExecutionPlan) {
        self.execution_plan = execution_plan;
    }

    /// Returns stable lifecycle transitions in their execution order.
    #[must_use]
    pub fn transitions(&self) -> &[LifecycleTransition] {
        &self.transitions
    }

    /// Returns Bindings created while processing the Event.
    #[must_use]
    pub fn created_bindings(&self) -> &[Binding] {
        &self.created_bindings
    }

    /// Returns Bindings removed while processing the Event.
    #[must_use]
    pub fn removed_bindings(&self) -> &[Binding] {
        &self.removed_bindings
    }

    /// Returns Effect identities removed with their owning lifecycle.
    #[must_use]
    pub fn removed_effects(&self) -> &[EffectId] {
        &self.removed_effects
    }

    /// Returns ordered dependency fronts executed for this Event.
    #[must_use]
    pub const fn execution_plan(&self) -> &ExecutionPlan {
        &self.execution_plan
    }
}
