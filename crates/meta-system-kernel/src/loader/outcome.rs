//! Inspectable Loader transitions returned after accepted Events.

use super::{LoadId, LoadPhase};

/// One completed Loader phase transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadTransition {
    /// Loader lifecycle that changed.
    id: LoadId,
    /// Prior phase, absent for declaration.
    previous: Option<LoadPhase>,
    /// Phase reached by the Event.
    current: LoadPhase,
}

impl LoadTransition {
    /// Describes one completed Loader transition.
    #[must_use]
    pub(crate) const fn new(id: LoadId, previous: Option<LoadPhase>, current: LoadPhase) -> Self {
        Self {
            id,
            previous,
            current,
        }
    }

    /// Returns the Loader lifecycle that changed.
    #[must_use]
    pub const fn id(self) -> LoadId {
        self.id
    }

    /// Returns the prior phase, absent for declaration.
    #[must_use]
    pub const fn previous(self) -> Option<LoadPhase> {
        self.previous
    }

    /// Returns the phase reached by the Event.
    #[must_use]
    pub const fn current(self) -> LoadPhase {
        self.current
    }
}

/// Observable result of one accepted Loader Event.
#[derive(Debug, Clone, PartialEq, Eq)]
#[must_use = "Loader outcomes expose the completed phase transition"]
pub struct LoaderOutcome {
    /// Sole transition completed by one Loader Event.
    transition: Option<LoadTransition>,
}

impl LoaderOutcome {
    /// Creates an outcome containing one completed phase transition.
    pub(crate) const fn transitioned(transition: LoadTransition) -> Self {
        Self {
            transition: Some(transition),
        }
    }

    /// Returns the completed transition, when the Event changed state.
    #[must_use]
    pub const fn transition(&self) -> Option<LoadTransition> {
        self.transition
    }
}
