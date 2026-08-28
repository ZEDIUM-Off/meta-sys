//! Deterministic in-memory adapter for Loader tests and reference use.

use super::{ComponentMaterializer, ComponentSource, LoadId, MaterializerError};
use crate::system::ComponentDefinition;

/// Materializer adapter that always yields one preconfigured complete Definition.
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
