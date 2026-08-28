//! Typed Events accepted by the Kernel Runtime machine.

use crate::{ComponentDefinition, ComponentInstanceId, Effect};

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
    /// Introduces one living Effect owned by an Active Component Instance.
    RecordEffect {
        /// Complete Effect identity and lifecycle owner.
        effect: Effect,
    },
    /// Removes one Component Instance and its lifecycle-owned resources.
    UnregisterComponent {
        /// Component Instance that disappears from the System Graph.
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

    /// Builds an Event that records one lifecycle-owned Effect.
    #[must_use]
    pub const fn record_effect(effect: Effect) -> Self {
        Self::RecordEffect { effect }
    }

    /// Builds an Event that removes one Component Instance.
    #[must_use]
    pub const fn unregister_component(instance_id: ComponentInstanceId) -> Self {
        Self::UnregisterComponent { instance_id }
    }
}
