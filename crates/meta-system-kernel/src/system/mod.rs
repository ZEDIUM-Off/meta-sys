//! The single living System Graph and the entities represented within it.
//!
//! The System domain separates complete static [`ComponentDefinition`] values from living
//! [`ComponentInstance`] occurrences and their active [`ComponentRuntime`] execution state. It
//! also contains structural [`Context`] scopes, Addon-owned [`Facet`] values, lifecycle-owned
//! [`Effect`] values, and the read-only [`SystemGraph`] observation seam.
//!
//! ```text
//! Component Definition ──creates──▶ Component Instance
//!          │                            │
//!          │ contributes                │ owns while Active
//!          ▼                            ▼
//!  graph relations                Component Runtime
//!          │
//!          ├── Context ──contains──▶ Facet
//!          └── Component Instance ─owns──▶ Effect
//! ```
//!
//! # Interface
//!
//! [`SystemGraph`] is the only public graph observation interface. Mutation remains hidden behind
//! [`KernelRuntime`](crate::runtime::KernelRuntime), so callers cannot bypass Kernel Events or
//! construct a second desired-state graph.
//!
//! # Invariants
//!
//! Definition, Instance, and Runtime identities are distinct. Exactly one living graph contains
//! Definitions, Instances, Requirements, Capabilities, Bindings, Contexts, Facets, Effects, and
//! routing lifecycles. This module stores those relations but does not select Bindings, schedule
//! execution, or interpret routing policy; those responsibilities belong to their dedicated
//! modules.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::{KernelRuntime, system::SystemGraph};
//!
//! let runtime = KernelRuntime::new();
//! let _graph: SystemGraph<'_> = runtime.graph();
//! ```

mod component_definition;
mod component_instance;
mod component_runtime;
mod context;
mod effect;
mod effect_lifecycle;
mod facet;
mod facet_lifecycle;
mod facet_schema;
mod facet_value;
mod graph;
mod graph_entity;
mod identity;
mod view;

pub use component_definition::ComponentDefinition;
pub use component_instance::{ComponentInstance, ResolutionState};
pub use component_runtime::ComponentRuntime;
pub use context::{Context, ContextOwner, ContextVisibility};
pub use effect::Effect;
pub use facet::Facet;
pub use facet_schema::FacetSchema;
pub use facet_value::{FacetValue, FacetValueKind};
pub use graph_entity::{FacetTarget, GraphEntityKind};
pub use identity::{
    AddonId, CapabilityContractId, CapabilityId, ComponentDefinitionId, ComponentInstanceId,
    ComponentRuntimeId, ContextId, EffectId, FacetId, FacetSchemaId, RequirementId,
};
pub use view::SystemGraph;

pub(crate) use graph::GraphState;
