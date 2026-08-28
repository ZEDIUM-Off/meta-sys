//! Addon-owned definitions of semantic graph dimensions.

use crate::{AddonId, FacetSchemaId, FacetValueKind, GraphEntityKind};

/// An Addon-owned typed dimension that may enrich eligible graph entities.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetSchema {
    /// Stable identity of this extension contract.
    id: FacetSchemaId,
    /// System Addon that owns the schema semantics.
    owner: AddonId,
    /// Data kind accepted by Facets of this schema.
    value_kind: FacetValueKind,
    /// Neutral graph category eligible for attachment.
    target_kind: GraphEntityKind,
}

impl FacetSchema {
    /// Defines one typed dimension without giving its semantics to the Kernel.
    #[must_use]
    pub const fn new(
        id: FacetSchemaId,
        owner: AddonId,
        value_kind: FacetValueKind,
        target_kind: GraphEntityKind,
    ) -> Self {
        Self {
            id,
            owner,
            value_kind,
            target_kind,
        }
    }

    /// Returns the stable Facet Schema identity.
    #[must_use]
    pub const fn id(&self) -> FacetSchemaId {
        self.id
    }

    /// Returns the Addon that owns this schema's semantics.
    #[must_use]
    pub const fn owner(&self) -> AddonId {
        self.owner
    }

    /// Returns the Facet data kind accepted by this schema.
    #[must_use]
    pub const fn value_kind(&self) -> FacetValueKind {
        self.value_kind
    }

    /// Returns the neutral graph category eligible for attachment.
    #[must_use]
    pub const fn target_kind(&self) -> GraphEntityKind {
        self.target_kind
    }
}
