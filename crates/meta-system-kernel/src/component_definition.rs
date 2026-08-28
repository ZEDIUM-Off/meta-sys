//! Static and complete declarations of Components.

use crate::{Capability, ComponentDefinitionId, Requirement};

/// The complete declarative identity and contributions of a Component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition {
    /// Stable identity of the declaration.
    id: ComponentDefinitionId,
    /// Capability needs contributed to the System Graph.
    requirements: Vec<Requirement>,
    /// Capability offers contributed to the System Graph.
    capabilities: Vec<Capability>,
}

impl ComponentDefinition {
    /// Creates a complete declaration with no contributions.
    #[must_use]
    pub const fn new(id: ComponentDefinitionId) -> Self {
        Self {
            id,
            requirements: Vec::new(),
            capabilities: Vec::new(),
        }
    }

    /// Adds one inspectable Capability to this complete declaration.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Adds one inspectable Requirement to this complete declaration.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_requirement(mut self, requirement: Requirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    /// Returns the stable Component Definition identity.
    #[must_use]
    pub const fn id(&self) -> ComponentDefinitionId {
        self.id
    }

    /// Returns every Requirement in declaration order.
    #[must_use]
    pub fn requirements(&self) -> &[Requirement] {
        &self.requirements
    }

    /// Returns every Capability in declaration order.
    #[must_use]
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }
}
