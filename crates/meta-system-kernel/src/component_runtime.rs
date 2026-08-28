//! Execution state attached only to Active Component Instances.

use crate::{ComponentInstanceId, ComponentRuntimeId};

/// The living execution attached to one Active Component Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRuntime {
    /// Identity unique to this concrete execution lifecycle.
    id: ComponentRuntimeId,
    /// Component Instance whose execution this object represents.
    instance_id: ComponentInstanceId,
}

impl ComponentRuntime {
    /// Creates execution state after its Driver has started successfully.
    #[must_use]
    pub(crate) const fn new(id: ComponentRuntimeId, instance_id: ComponentInstanceId) -> Self {
        Self { id, instance_id }
    }

    /// Returns the identity of this concrete execution lifecycle.
    #[must_use]
    pub const fn id(&self) -> ComponentRuntimeId {
        self.id
    }

    /// Returns the Component Instance executed by this Runtime.
    #[must_use]
    pub const fn instance_id(&self) -> ComponentInstanceId {
        self.instance_id
    }
}
