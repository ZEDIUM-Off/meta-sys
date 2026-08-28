//! Ordered Addon policy seam for proposed Binding selection.

use super::{BindingDecision, BindingHook};
use crate::{
    runtime::KernelError,
    system::{CapabilityContractId, CapabilityId, ComponentInstanceId, RequirementId},
};

/// One compatible Capability and its publishing provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BindingCandidate {
    /// Compatible Capability considered by the Resolver.
    capability: CapabilityId,
    /// Component Instance publishing the Capability.
    provider: ComponentInstanceId,
}

impl BindingCandidate {
    /// Creates one compatible provider candidate.
    #[must_use]
    pub(crate) const fn new(capability: CapabilityId, provider: ComponentInstanceId) -> Self {
        Self {
            capability,
            provider,
        }
    }

    /// Returns the compatible Capability identity.
    #[must_use]
    pub const fn capability(&self) -> CapabilityId {
        self.capability
    }

    /// Returns the publishing Component Instance.
    #[must_use]
    pub const fn provider(&self) -> ComponentInstanceId {
        self.provider
    }
}

/// Inspectable proposed Binding passed through ordered active Addon hooks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BindingProposal {
    /// Requirement being resolved.
    requirement: RequirementId,
    /// Capability Contract required by the consumer.
    contract: CapabilityContractId,
    /// Every compatible provider eligible for this mutation.
    candidates: Vec<BindingCandidate>,
    /// Capability selected by prior hooks or the allow-all default.
    selected: CapabilityId,
}

impl BindingProposal {
    /// Creates a proposal with the deterministic allow-all selection.
    #[must_use]
    pub(crate) const fn new(
        requirement: RequirementId,
        contract: CapabilityContractId,
        candidates: Vec<BindingCandidate>,
        selected: CapabilityId,
    ) -> Self {
        Self {
            requirement,
            contract,
            candidates,
            selected,
        }
    }

    /// Replaces the selected compatible Capability for the next hook.
    pub(crate) const fn select(&mut self, capability: CapabilityId) {
        self.selected = capability;
    }

    /// Returns the Requirement being resolved.
    #[must_use]
    pub const fn requirement(&self) -> RequirementId {
        self.requirement
    }

    /// Returns the Capability Contract required by the consumer.
    #[must_use]
    pub const fn contract(&self) -> CapabilityContractId {
        self.contract
    }

    /// Returns every compatible provider candidate.
    #[must_use]
    pub fn candidates(&self) -> &[BindingCandidate] {
        &self.candidates
    }

    /// Returns the Capability selected by prior hooks or allow-all.
    #[must_use]
    pub const fn selected(&self) -> CapabilityId {
        self.selected
    }
}

/// Applies active hooks in pre-sorted order and returns the selected candidate.
pub fn evaluate_binding_hooks(
    hooks: &[Box<dyn BindingHook>],
    mut proposal: BindingProposal,
    mut selected: BindingCandidate,
) -> Result<BindingCandidate, KernelError> {
    for hook in hooks {
        match hook.evaluate(&proposal) {
            BindingDecision::Allow => {}
            BindingDecision::Reject(reason) => {
                return Err(KernelError::BindingRejected {
                    addon: hook.addon(),
                    requirement: proposal.requirement(),
                    reason,
                });
            }
            BindingDecision::SelectCapability(capability) => {
                selected = proposal
                    .candidates()
                    .iter()
                    .find(|candidate| candidate.capability() == capability)
                    .copied()
                    .ok_or_else(|| KernelError::InvalidBindingSelection {
                        addon: hook.addon(),
                        requirement: proposal.requirement(),
                        capability,
                    })?;
                proposal.select(capability);
            }
        }
    }
    Ok(selected)
}
