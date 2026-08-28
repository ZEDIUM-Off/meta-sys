//! Living consequences owned by Component Instance lifecycles.

use super::{ComponentInstanceId, EffectId};

/// An inspectable living consequence owned by one Component Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// Stable identity of this living consequence.
    id: EffectId,
    /// Component Instance whose lifecycle governs this Effect.
    owner: ComponentInstanceId,
}

impl Effect {
    /// Creates an Effect explicitly owned by one Component Instance.
    #[must_use]
    pub const fn new(id: EffectId, owner: ComponentInstanceId) -> Self {
        Self { id, owner }
    }

    /// Returns the stable Effect identity.
    #[must_use]
    pub const fn id(&self) -> EffectId {
        self.id
    }

    /// Returns the Component Instance that owns this Effect.
    #[must_use]
    pub const fn owner(&self) -> ComponentInstanceId {
        self.owner
    }
}
