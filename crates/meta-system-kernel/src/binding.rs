//! Explicit resolution relations between Requirements and providers.

use crate::{CapabilityId, ComponentInstanceId, RequirementId};

/// An inspectable relation from one Requirement to its provider Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding {
    /// Requirement resolved by this relation.
    requirement: RequirementId,
    /// Compatible Capability selected for the Requirement.
    capability: CapabilityId,
    /// Component Instance providing the compatible Capability.
    provider: ComponentInstanceId,
}

impl Binding {
    /// Creates an explicit resolution relation selected by the Kernel.
    #[must_use]
    pub(crate) const fn new(
        requirement_id: RequirementId,
        capability_id: CapabilityId,
        provider_id: ComponentInstanceId,
    ) -> Self {
        Self {
            requirement: requirement_id,
            capability: capability_id,
            provider: provider_id,
        }
    }

    /// Returns the Requirement resolved by this Binding.
    #[must_use]
    pub const fn requirement_id(&self) -> RequirementId {
        self.requirement
    }

    /// Returns the compatible Capability selected by this Binding.
    #[must_use]
    pub const fn capability_id(&self) -> CapabilityId {
        self.capability
    }

    /// Returns the Component Instance selected as provider.
    #[must_use]
    pub const fn provider_id(&self) -> ComponentInstanceId {
        self.provider
    }
}
