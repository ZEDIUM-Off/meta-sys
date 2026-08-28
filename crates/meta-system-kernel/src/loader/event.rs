//! Typed Events that drive the ordered Loader machine.

use super::LoadId;
use crate::system::ComponentInstanceId;

/// Opaque source declaration interpreted only by a materializer adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentSource(String);

impl ComponentSource {
    /// Creates an opaque source declaration without granting a Capability.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the adapter-owned source text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Complete declaration required to start one Loader lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadRequest {
    /// Stable lifecycle identity.
    id: LoadId,
    /// Opaque source interpreted by the materializer.
    source: ComponentSource,
    /// Component Instance identity reserved for Kernel registration.
    instance: ComponentInstanceId,
}

impl LoadRequest {
    /// Creates one complete load declaration.
    #[must_use]
    pub const fn new(id: LoadId, source: ComponentSource, instance: ComponentInstanceId) -> Self {
        Self {
            id,
            source,
            instance,
        }
    }

    /// Returns the stable Loader lifecycle identity.
    #[must_use]
    pub const fn id(&self) -> LoadId {
        self.id
    }

    /// Returns the opaque component source.
    #[must_use]
    pub const fn source(&self) -> &ComponentSource {
        &self.source
    }

    /// Returns the Component Instance reserved for registration.
    #[must_use]
    pub const fn instance(&self) -> ComponentInstanceId {
        self.instance
    }
}

/// Typed stimulus that advances exactly one Loader transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoaderEvent {
    /// Starts a lifecycle in `Declared`.
    Declare(LoadRequest),
    /// Advances `Declared` to `Located`.
    Locate(LoadId),
    /// Advances `Located` to `Materialized`.
    Materialize(LoadId),
    /// Advances `Materialized` to `Inspected` with one complete Definition.
    Inspect(LoadId),
    /// Advances `Inspected` to `Admitted`.
    Admit(LoadId),
    /// Advances `Inspected` to terminal `Rejected` with a reason.
    Reject {
        /// Loader lifecycle being rejected.
        id: LoadId,
        /// Inspectable rejection reason.
        reason: String,
    },
    /// Registers the inspected Definition with the Kernel Runtime.
    Register(LoadId),
    /// Marks a successfully registered lifecycle `Ready`.
    MarkReady(LoadId),
}
