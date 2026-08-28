//! Active Addon decisions at the ordered Binding policy seam.

use crate::{AddonId, BindingProposal, CapabilityId};

/// Deterministic total-order position of one active Binding hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HookOrder(u32);

impl HookOrder {
    /// Creates an explicit hook order value; lower values run first.
    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }
}

/// Influence returned by one active Addon at the Binding seam.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingDecision {
    /// Accepts the proposal without changing provider selection.
    Allow,
    /// Rejects Binding creation with an inspectable Addon-owned reason.
    Reject(String),
    /// Selects another compatible Capability from the proposal.
    SelectCapability(CapabilityId),
}

/// Active Addon hook participating in deterministic Binding policy.
pub trait BindingHook: std::fmt::Debug {
    /// Returns the Addon that owns this policy decision.
    fn addon(&self) -> AddonId;

    /// Returns this hook's declared deterministic order.
    fn order(&self) -> HookOrder;

    /// Observes and optionally rejects or influences the proposed Binding.
    fn evaluate(&self, proposal: &BindingProposal) -> BindingDecision;
}
