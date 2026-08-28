//! Living occurrences of Component Definitions.

use crate::{ComponentDefinitionId, ComponentInstanceId};

/// Stable resolution state of a Component Instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionState {
    /// At least one necessary Requirement has no Binding.
    Pending,
    /// Every necessary Requirement has a Binding and a Component Runtime lives.
    Active,
}

/// A living occurrence of a Component Definition and its resolution state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentInstance {
    /// Stable identity of this occurrence.
    id: ComponentInstanceId,
    /// Declaration instantiated by this occurrence.
    definition_id: ComponentDefinitionId,
    /// Current stable resolution state.
    resolution: ResolutionState,
}

impl ComponentInstance {
    /// Creates an unresolved occurrence for insertion into the graph.
    #[must_use]
    pub(crate) const fn pending(
        id: ComponentInstanceId,
        definition_id: ComponentDefinitionId,
    ) -> Self {
        Self {
            id,
            definition_id,
            resolution: ResolutionState::Pending,
        }
    }

    /// Marks this occurrence Active after its Bindings and Runtime exist.
    pub(crate) const fn activate(&mut self) {
        self.resolution = ResolutionState::Active;
    }

    /// Returns this occurrence to Pending after lifecycle cleanup.
    pub(crate) const fn deactivate(&mut self) {
        self.resolution = ResolutionState::Pending;
    }

    /// Returns the stable Component Instance identity.
    #[must_use]
    pub const fn id(&self) -> ComponentInstanceId {
        self.id
    }

    /// Returns the Component Definition instantiated by this occurrence.
    #[must_use]
    pub const fn definition_id(&self) -> ComponentDefinitionId {
        self.definition_id
    }

    /// Returns the current stable resolution state.
    #[must_use]
    pub const fn resolution(&self) -> ResolutionState {
        self.resolution
    }
}
