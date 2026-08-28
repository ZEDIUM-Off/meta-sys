//! Active Addon decisions at the ordered Loader admission seam.

use super::LoadId;
use crate::{
    resolution::HookOrder,
    system::{AddonId, ComponentDefinition},
};

/// Complete inspected load proposed for admission to the Kernel Runtime.
#[derive(Debug, Clone, Copy)]
pub struct LoaderProposal<'definition> {
    /// Loader lifecycle being admitted.
    load: LoadId,
    /// Sole complete Definition obtained during inspection.
    definition: &'definition ComponentDefinition,
}

impl<'definition> LoaderProposal<'definition> {
    /// Creates the immutable proposal observed by every active hook.
    #[must_use]
    pub(crate) const fn new(load: LoadId, definition: &'definition ComponentDefinition) -> Self {
        Self { load, definition }
    }

    /// Returns the Loader lifecycle under policy evaluation.
    #[must_use]
    pub const fn load(self) -> LoadId {
        self.load
    }

    /// Returns the complete inspected Component Definition.
    #[must_use]
    pub const fn definition(self) -> &'definition ComponentDefinition {
        self.definition
    }
}

/// Admission decision returned by one active Loader Addon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoaderDecision {
    /// Allows the complete Definition to continue toward admission.
    Allow,
    /// Rejects the load with an inspectable Addon-owned reason.
    Reject(String),
}

/// Active Loader Addon hook participating in deterministic admission policy.
pub trait LoaderHook: std::fmt::Debug {
    /// Returns the Addon that owns this policy decision.
    fn addon(&self) -> AddonId;

    /// Returns this hook's declared deterministic order.
    fn order(&self) -> HookOrder;

    /// Observes and optionally rejects one complete inspected Definition.
    fn evaluate(&self, proposal: LoaderProposal<'_>) -> LoaderDecision;
}
