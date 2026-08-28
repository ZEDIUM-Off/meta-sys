//! Runtime primitives for the Meta-System Kernel.
//!
//! This crate will expose the small public seams through which callers submit
//! typed events and observe the resulting system graph. Prototype behaviour is
//! introduced only by independently tested vertical slices.

mod binding;
mod capability;
mod component_definition;
mod component_instance;
mod component_runtime;
mod context;
mod driver;
mod effect;
mod effect_lifecycle;
mod error;
mod event;
mod facet;
mod facet_lifecycle;
mod facet_schema;
mod facet_value;
mod graph;
mod graph_entity;
mod graph_view;
mod identity;
mod lifecycle_transition;
mod outcome;
mod removal;
mod requirement;
mod resolution;
mod runtime;

pub use binding::Binding;
pub use capability::Capability;
pub use component_definition::ComponentDefinition;
pub use component_instance::{ComponentInstance, ResolutionState};
pub use component_runtime::ComponentRuntime;
pub use context::{Context, ContextOwner, ContextVisibility};
pub use driver::{DriverError, DriverProgress, EventLoopDriver, SequentialExecutor};
pub use effect::Effect;
pub use error::KernelError;
pub use event::KernelEvent;
pub use facet::Facet;
pub use facet_schema::FacetSchema;
pub use facet_value::{FacetValue, FacetValueKind};
pub use graph_entity::{FacetTarget, GraphEntityKind};
pub use graph_view::SystemGraph;
pub use identity::{
    AddonId, CapabilityContractId, CapabilityId, ComponentDefinitionId, ComponentInstanceId,
    ComponentRuntimeId, ContextId, EffectId, FacetId, FacetSchemaId, RequirementId,
};
pub use lifecycle_transition::LifecycleTransition;
pub use outcome::TransitionOutcome;
pub use requirement::Requirement;
pub use runtime::KernelRuntime;
