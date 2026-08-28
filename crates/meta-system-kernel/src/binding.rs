//! Explicit resolution relations between Requirements and providers.

use crate::{ComponentInstanceId, RequirementId};

/// An inspectable relation from one Requirement to its provider Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    /// Requirement resolved by this relation.
    requirement_id: RequirementId,
    /// Component Instance providing the compatible Capability.
    provider_id: ComponentInstanceId,
}

impl Binding {
    /// Returns the Requirement resolved by this Binding.
    #[must_use]
    pub const fn requirement_id(&self) -> RequirementId {
        self.requirement_id
    }

    /// Returns the Component Instance selected as provider.
    #[must_use]
    pub const fn provider_id(&self) -> ComponentInstanceId {
        self.provider_id
    }
}
