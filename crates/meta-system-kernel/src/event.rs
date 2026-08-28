//! Typed Events accepted by the Kernel Runtime machine.

use crate::{ComponentDefinition, ComponentInstanceId};

/// A typed stimulus that can evolve one Kernel Runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelEvent {
    /// Introduces one complete Component Definition and one living occurrence.
    RegisterComponent {
        /// Complete declaration made inspectable by this Event.
        definition: ComponentDefinition,
        /// Stable identity assigned to the new Component Instance.
        instance_id: ComponentInstanceId,
    },
}

impl KernelEvent {
    /// Builds the registration Event for one complete declaration and occurrence.
    #[must_use]
    pub const fn register_component(
        definition: ComponentDefinition,
        instance_id: ComponentInstanceId,
    ) -> Self {
        Self::RegisterComponent {
            definition,
            instance_id,
        }
    }
}
