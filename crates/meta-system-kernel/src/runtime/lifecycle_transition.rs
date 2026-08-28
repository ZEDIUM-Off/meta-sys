//! Observable stable resolution-state changes.

use crate::system::{ComponentInstanceId, ResolutionState};

/// One observable resolution-state change caused by an Event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LifecycleTransition {
    /// Component Instance whose state changed.
    instance_id: ComponentInstanceId,
    /// Stable state before the transition, absent for a new Instance.
    previous: Option<ResolutionState>,
    /// Stable state after the transition, absent when the Instance was removed.
    current: Option<ResolutionState>,
}

impl LifecycleTransition {
    /// Describes one completed stable-state transition.
    #[must_use]
    pub(crate) const fn new(
        instance_id: ComponentInstanceId,
        previous: Option<ResolutionState>,
        current: Option<ResolutionState>,
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

    /// Returns the reached stable state, or `None` after Instance removal.
    #[must_use]
    pub const fn current(&self) -> Option<ResolutionState> {
        self.current
    }
}
