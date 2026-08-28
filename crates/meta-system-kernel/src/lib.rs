//! Runtime primitives for the Meta-System Kernel.
//!
//! This crate will expose the small public seams through which callers submit
//! typed events and observe the resulting system graph. Prototype behaviour is
//! introduced only by independently tested vertical slices.

mod binding;
mod component_definition;
mod component_instance;
mod component_runtime;
mod error;
mod event;
mod graph;
mod identity;
mod requirement;
mod runtime;

pub use binding::Binding;
pub use component_definition::ComponentDefinition;
pub use component_instance::{ComponentInstance, ResolutionState};
pub use component_runtime::ComponentRuntime;
pub use error::KernelError;
pub use event::KernelEvent;
pub use graph::SystemGraph;
pub use identity::{
    CapabilityContractId, ComponentDefinitionId, ComponentInstanceId, RequirementId,
};
pub use requirement::Requirement;
pub use runtime::KernelRuntime;
