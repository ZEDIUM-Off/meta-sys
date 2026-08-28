//! Inspectable identity and ordered state of one Loader cycle.

use crate::{
    ComponentDefinition, ComponentInstanceId, ComponentSource, LoadTransition, LoaderError,
};

/// Identifies one independent Loader lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoadId(u64);

impl LoadId {
    /// Creates an opaque Loader lifecycle identity.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Ordered observable phase of one Loader lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadPhase {
    /// A typed load request exists.
    Declared,
    /// The adapter resolved the source location.
    Located,
    /// Executable support was materialized.
    Materialized,
    /// One complete Component Definition was inspected.
    Inspected,
    /// Loading policy admitted the complete Definition.
    Admitted,
    /// Loading policy rejected the complete Definition.
    Rejected,
    /// The Kernel Runtime accepted Component registration.
    Registered,
    /// The complete load lifecycle finished successfully.
    Ready,
}

/// Inspectable state of one Loader lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRecord {
    /// Stable Loader lifecycle identity.
    id: LoadId,
    /// Source interpreted only by the configured materializer.
    source: ComponentSource,
    /// Component Instance identity reserved for Kernel registration.
    instance: ComponentInstanceId,
    /// Current ordered Loader phase.
    phase: LoadPhase,
    /// Complete inspected Definition, absent before `Inspected`.
    definition: Option<ComponentDefinition>,
    /// Inspectable rejection reason when the lifecycle ends Rejected.
    rejection: Option<String>,
}

impl LoadRecord {
    /// Creates the initial inspectable `Declared` state.
    #[must_use]
    pub(crate) fn declared(request: &crate::LoadRequest) -> Self {
        Self {
            id: request.id(),
            source: request.source().clone(),
            instance: request.instance(),
            phase: LoadPhase::Declared,
            definition: None,
            rejection: None,
        }
    }

    /// Validates the phase required before an Event may perform side effects.
    pub(crate) fn require(&self, expected: LoadPhase) -> Result<(), LoaderError> {
        if self.phase == expected {
            Ok(())
        } else {
            Err(LoaderError::InvalidTransition {
                expected,
                actual: self.phase,
            })
        }
    }

    /// Advances to one validated next phase and returns its observation.
    pub(crate) const fn advance(&mut self, next: LoadPhase) -> LoadTransition {
        let previous = self.phase;
        self.phase = next;
        LoadTransition::new(self.id, Some(previous), next)
    }

    /// Stores the sole complete Definition produced by inspection.
    pub(crate) fn set_definition(&mut self, definition: ComponentDefinition) {
        self.definition = Some(definition);
    }

    /// Stores an inspectable policy rejection reason.
    pub(crate) fn set_rejection(&mut self, reason: String) {
        self.rejection = Some(reason);
    }

    /// Returns the stable Loader lifecycle identity.
    #[must_use]
    pub const fn id(&self) -> LoadId {
        self.id
    }

    /// Returns the materializer-owned source declaration.
    #[must_use]
    pub const fn source(&self) -> &ComponentSource {
        &self.source
    }

    /// Returns the Component Instance reserved for registration.
    #[must_use]
    pub const fn instance(&self) -> ComponentInstanceId {
        self.instance
    }

    /// Returns the current ordered Loader phase.
    #[must_use]
    pub const fn phase(&self) -> LoadPhase {
        self.phase
    }

    /// Returns the sole complete Component Definition after inspection.
    #[must_use]
    pub const fn definition(&self) -> Option<&ComponentDefinition> {
        self.definition.as_ref()
    }

    /// Returns the inspectable rejection reason, when rejected.
    #[must_use]
    pub fn rejection(&self) -> Option<&str> {
        self.rejection.as_deref()
    }
}
