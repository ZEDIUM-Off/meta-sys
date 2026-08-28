//! Execution state attached only to Active Component Instances.

use crate::ComponentInstanceId;

/// The living execution attached to one Active Component Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRuntime {
    /// Component Instance whose execution this object represents.
    instance_id: ComponentInstanceId,
}

impl ComponentRuntime {
    /// Returns the Component Instance executed by this Runtime.
    #[must_use]
    pub const fn instance_id(&self) -> ComponentInstanceId {
        self.instance_id
    }
}
