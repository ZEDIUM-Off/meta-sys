//! Validation and storage for lifecycle-owned Effects.

use super::graph::GraphState;
use super::{Effect, ResolutionState};
use crate::runtime::KernelError;

impl GraphState {
    /// Records an Effect only while its owning Component Instance is Active.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] for an unknown or inactive owner, or when the
    /// Effect identity already exists.
    pub(crate) fn record_effect(&mut self, effect: Effect) -> Result<(), KernelError> {
        let owner_id = effect.owner();
        let owner = self
            .instances
            .get(&owner_id)
            .ok_or(KernelError::UnknownComponentInstance(owner_id))?;
        if owner.resolution() != ResolutionState::Active {
            return Err(KernelError::InactiveEffectOwner(owner_id));
        }
        if self.effects.contains_key(&effect.id()) {
            return Err(KernelError::DuplicateEffect(effect.id()));
        }
        self.effects.insert(effect.id(), effect);
        Ok(())
    }
}
