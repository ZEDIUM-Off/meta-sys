//! Inspectable terminal rejection produced by Loader admission policy.

use crate::system::AddonId;

/// Reason and optional Addon owner of one rejected Loader lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRejection {
    /// Active Loader Addon that rejected admission, absent for host Events.
    addon: Option<AddonId>,
    /// Human-readable policy or host reason.
    reason: String,
}

impl LoadRejection {
    /// Creates the complete rejection observation stored by the Loader.
    #[must_use]
    pub(crate) const fn new(addon: Option<AddonId>, reason: String) -> Self {
        Self { addon, reason }
    }

    /// Returns the active Loader Addon that rejected admission, when any.
    #[must_use]
    pub const fn addon(&self) -> Option<AddonId> {
        self.addon
    }

    /// Returns the inspectable Addon- or host-owned rejection reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}
