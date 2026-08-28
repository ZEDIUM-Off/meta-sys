//! Closed neutral value representation for typed Facets.

/// Runtime-checkable data kind declared by a Facet Schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FacetValueKind {
    /// Boolean extension data.
    Boolean,
    /// Signed integer extension data.
    Integer,
    /// UTF-8 text extension data.
    Text,
    /// Opaque binary extension data.
    Bytes,
}

/// Typed extension data whose semantics remain owned by an Addon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacetValue {
    /// Boolean extension data.
    Boolean(bool),
    /// Signed integer extension data.
    Integer(i64),
    /// UTF-8 text extension data.
    Text(String),
    /// Opaque binary extension data.
    Bytes(Vec<u8>),
}

impl FacetValue {
    /// Returns the runtime-checkable kind of this value.
    #[must_use]
    pub const fn kind(&self) -> FacetValueKind {
        match self {
            Self::Boolean(_) => FacetValueKind::Boolean,
            Self::Integer(_) => FacetValueKind::Integer,
            Self::Text(_) => FacetValueKind::Text,
            Self::Bytes(_) => FacetValueKind::Bytes,
        }
    }
}
