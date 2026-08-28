//! Structural visibility, ownership, and lifecycle scopes.

use crate::{AddonId, ComponentInstanceId, ContextId};

/// Entity whose lifecycle owns a Context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextOwner {
    /// A System Addon owns the structural scope.
    Addon(AddonId),
    /// A living Component Instance owns the structural scope.
    Component(ComponentInstanceId),
}

/// Visibility boundary declared by a Context.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContextVisibility {
    /// Only the owning entity can observe scoped contributions.
    Owner,
    /// The owner and nested Contexts can observe scoped contributions.
    Descendants,
    /// Every entity in the same Kernel Runtime can observe scoped contributions.
    Runtime,
}

/// A structural scope for visibility, ownership, and lifecycle.
///
/// A Context contains no dependency lookup or resolution behavior.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Context {
    /// Stable identity of this structural scope.
    id: ContextId,
    /// Entity whose lifecycle governs the scope.
    owner: ContextOwner,
    /// Explicit visibility boundary.
    visibility: ContextVisibility,
    /// Optional enclosing structural scope.
    parent: Option<ContextId>,
}

impl Context {
    /// Creates a root Context with explicit ownership and visibility.
    #[must_use]
    pub const fn new(id: ContextId, owner: ContextOwner, visibility: ContextVisibility) -> Self {
        Self {
            id,
            owner,
            visibility,
            parent: None,
        }
    }

    /// Nests this Context under an existing structural scope.
    #[must_use = "builder methods return the updated Context"]
    pub const fn within(mut self, parent: ContextId) -> Self {
        self.parent = Some(parent);
        self
    }

    /// Returns the stable Context identity.
    #[must_use]
    pub const fn id(&self) -> ContextId {
        self.id
    }

    /// Returns the entity governing this Context lifecycle.
    #[must_use]
    pub const fn owner(&self) -> ContextOwner {
        self.owner
    }

    /// Returns the explicit visibility boundary.
    #[must_use]
    pub const fn visibility(&self) -> ContextVisibility {
        self.visibility
    }

    /// Returns the enclosing Context, when this scope is nested.
    #[must_use]
    pub const fn parent(&self) -> Option<ContextId> {
        self.parent
    }
}
