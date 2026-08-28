//! Typed Events accepted by the Kernel Runtime machine.

use crate::{ComponentDefinition, ComponentInstanceId, Context, Effect, Facet, FacetSchema};

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
    /// Introduces one structural Context into the System Graph.
    RegisterContext {
        /// Complete ownership and visibility scope.
        context: Context,
    },
    /// Introduces one Addon-owned typed Facet Schema.
    RegisterFacetSchema {
        /// Complete schema contract made inspectable by this Event.
        schema: FacetSchema,
    },
    /// Attaches one typed Facet to an eligible graph entity.
    AttachFacet {
        /// Complete Facet to validate and make inspectable.
        facet: Facet,
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

    /// Builds an Event that registers one structural Context.
    #[must_use]
    pub const fn register_context(context: Context) -> Self {
        Self::RegisterContext { context }
    }

    /// Builds an Event that registers one Addon-owned Facet Schema.
    #[must_use]
    pub const fn register_facet_schema(schema: FacetSchema) -> Self {
        Self::RegisterFacetSchema { schema }
    }

    /// Builds an Event that attaches one typed Facet.
    #[must_use]
    pub const fn attach_facet(facet: Facet) -> Self {
        Self::AttachFacet { facet }
    }
}
