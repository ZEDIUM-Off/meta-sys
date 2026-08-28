//! Adapter boundary for locating, materializing, and inspecting support.

use crate::{ComponentDefinition, ComponentSource, LoadId};
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

/// Deterministic in-memory adapter for Loader contract tests and reference use.
#[derive(Debug)]
pub struct DeterministicMaterializer {
    /// Complete Definition returned after ordered adapter calls.
    definition: ComponentDefinition,
}

impl DeterministicMaterializer {
    /// Creates an adapter that deterministically yields one complete Definition.
    #[must_use]
    pub const fn new(definition: ComponentDefinition) -> Self {
        Self { definition }
    }
}

impl ComponentMaterializer for DeterministicMaterializer {
    fn locate(&mut self, _id: LoadId, _source: &ComponentSource) -> Result<(), MaterializerError> {
        Ok(())
    }

    fn materialize(&mut self, _id: LoadId) -> Result<(), MaterializerError> {
        Ok(())
    }

    fn inspect(&mut self, _id: LoadId) -> Result<ComponentDefinition, MaterializerError> {
        Ok(self.definition.clone())
    }
}
