//! Typed instances of Addon-owned Facet Schemas.

use crate::{ContextId, FacetId, FacetSchemaId, FacetTarget, FacetValue};

/// One typed Facet attached to an eligible graph entity inside a Context.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet {
    /// Stable identity of this schema instance.
    id: FacetId,
    /// Addon-owned schema governing type and target eligibility.
    schema: FacetSchemaId,
    /// Structural scope governing visibility, ownership, and lifecycle.
    context: ContextId,
    /// Existing graph entity enriched by this Facet.
    target: FacetTarget,
    /// Typed data interpreted only by the schema-owning Addon.
    value: FacetValue,
}

impl Facet {
    /// Creates a typed schema instance for validation by the Kernel Runtime.
    #[must_use]
    pub const fn new(
        id: FacetId,
        schema: FacetSchemaId,
        context: ContextId,
        target: FacetTarget,
        value: FacetValue,
    ) -> Self {
        Self {
            id,
            schema,
            context,
            target,
            value,
        }
    }

    /// Returns the stable Facet identity.
    #[must_use]
    pub const fn id(&self) -> FacetId {
        self.id
    }

    /// Returns the Addon-owned Facet Schema identity.
    #[must_use]
    pub const fn schema(&self) -> FacetSchemaId {
        self.schema
    }

    /// Returns the structural Context containing this Facet.
    #[must_use]
    pub const fn context(&self) -> ContextId {
        self.context
    }

    /// Returns the eligible graph entity enriched by this Facet.
    #[must_use]
    pub const fn target(&self) -> FacetTarget {
        self.target
    }

    /// Returns typed extension data interpreted by the schema-owning Addon.
    #[must_use]
    pub const fn value(&self) -> &FacetValue {
        &self.value
    }
}
