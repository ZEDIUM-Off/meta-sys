//! Adapter seam for locating, materializing, and inspecting Component support.

use super::{ComponentSource, LoadId};
use crate::system::ComponentDefinition;
use thiserror::Error;

/// Failure reported by a component materializer adapter.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("{message}")]
pub struct MaterializerError {
    /// Adapter-owned diagnostic context.
    message: String,
}

impl MaterializerError {
    /// Creates an adapter-owned materialization failure.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Bootstrap adapter kept separate from Component Capabilities.
pub trait ComponentMaterializer: std::fmt::Debug {
    /// Resolves one opaque Component source.
    ///
    /// # Errors
    ///
    /// Returns [`MaterializerError`] when the source cannot be located.
    fn locate(&mut self, id: LoadId, source: &ComponentSource) -> Result<(), MaterializerError>;

    /// Materializes executable support for a located source.
    ///
    /// # Errors
    ///
    /// Returns [`MaterializerError`] when executable support cannot be prepared.
    fn materialize(&mut self, id: LoadId) -> Result<(), MaterializerError>;

    /// Inspects and returns one complete Component Definition.
    ///
    /// # Errors
    ///
    /// Returns [`MaterializerError`] when a complete Definition cannot be inspected.
    fn inspect(&mut self, id: LoadId) -> Result<ComponentDefinition, MaterializerError>;
}
