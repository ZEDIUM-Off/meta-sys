//! Observable explanations returned after Kernel state transitions.

use crate::{Binding, ComponentInstanceId, ResolutionState};

/// One observable resolution-state change caused by an Event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleTransition {
    /// Component Instance whose state changed.
    instance_id: ComponentInstanceId,
    /// Stable state before the transition, absent for a new Instance.
    previous: Option<ResolutionState>,
    /// Stable state after the transition.
    current: ResolutionState,
}

impl LifecycleTransition {
    /// Describes one completed stable-state transition.
    #[must_use]
    pub(crate) const fn new(
        instance_id: ComponentInstanceId,
        previous: Option<ResolutionState>,
        current: ResolutionState,
    ) -> Self {
        Self {
            instance_id,
            previous,
            current,
        }
    }

    /// Returns the Component Instance affected by this transition.
    #[must_use]
    pub const fn instance_id(&self) -> ComponentInstanceId {
        self.instance_id
    }

    /// Returns the prior stable state, or `None` for a new Instance.
    #[must_use]
    pub const fn previous(&self) -> Option<ResolutionState> {
        self.previous
    }

    /// Returns the stable state reached by the transition.
    #[must_use]
    pub const fn current(&self) -> ResolutionState {
        self.current
    }
}

/// Observable graph and lifecycle changes produced by one accepted Event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "transition outcomes explain the graph changes caused by an Event"]
pub struct TransitionOutcome {
    /// Stable lifecycle transitions in execution order.
    transitions: Vec<LifecycleTransition>,
    /// Bindings created while resolving affected Instances.
    created_bindings: Vec<Binding>,
}

impl TransitionOutcome {
    /// Creates the initial Pending transition for a registered Instance.
    pub(crate) fn registered_pending(instance_id: ComponentInstanceId) -> Self {
        Self {
            transitions: vec![LifecycleTransition::new(
                instance_id,
                None,
                ResolutionState::Pending,
            )],
            created_bindings: Vec::new(),
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
            ResolutionState::Active,
        ));
        self.created_bindings.extend_from_slice(bindings);
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
}
