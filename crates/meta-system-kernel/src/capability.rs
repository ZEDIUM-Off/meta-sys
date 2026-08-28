//! Inspectable offers contributed by Component Definitions.

use crate::{CapabilityContractId, CapabilityId};

/// An inspectable offer of one Capability Contract.
///
/// A Capability participates in composition and never represents permission.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    /// Stable identity of this offer.
    id: CapabilityId,
    /// Contract implemented by this offer.
    contract: CapabilityContractId,
}

impl Capability {
    /// Declares an offer implementing the given Capability Contract.
    #[must_use]
    pub const fn new(id: CapabilityId, contract: CapabilityContractId) -> Self {
        Self { id, contract }
    }

    /// Returns the stable Capability identity.
    #[must_use]
    pub const fn id(&self) -> CapabilityId {
        self.id
    }

    /// Returns the Capability Contract implemented by this offer.
    #[must_use]
    pub const fn contract(&self) -> CapabilityContractId {
        self.contract
    }
}
