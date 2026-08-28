//! Declarative Capability needs carried by Component Definitions.

use crate::{CapabilityContractId, RequirementId};

/// An inspectable need for a Capability, never an authorization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Requirement {
    /// Stable identity of this declared Requirement.
    id: RequirementId,
    /// Capability Contract that a provider must satisfy.
    contract: CapabilityContractId,
}

impl Requirement {
    /// Declares a Requirement that must be bound before its Instance can activate.
    #[must_use]
    pub const fn necessary(id: RequirementId, contract: CapabilityContractId) -> Self {
        Self { id, contract }
    }

    /// Returns the stable Requirement identity.
    #[must_use]
    pub const fn id(&self) -> RequirementId {
        self.id
    }

    /// Returns the Capability Contract requested by this Requirement.
    #[must_use]
    pub const fn contract(&self) -> CapabilityContractId {
        self.contract
    }
}
