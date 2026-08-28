//! Runtime primitives for the Meta-System Kernel.
//!
//! This crate will expose the small public seams through which callers submit
//! typed events and observe the resulting system graph. Prototype behaviour is
//! introduced only by independently tested vertical slices.

mod binding;
mod binding_hook;
mod binding_policy;
mod capability;
mod component_definition;
mod component_instance;
mod component_runtime;
mod context;
mod delivery;
mod driver;
mod effect;
mod effect_lifecycle;
mod error;
mod event;
mod event_message;
mod execution_plan;
mod facet;
mod facet_lifecycle;
mod facet_schema;
mod facet_value;
mod graph;
mod graph_entity;
mod graph_view;
mod identity;
mod lifecycle_transition;
mod loader;
mod loader_error;
mod loader_event;
mod loader_hook;
mod loader_outcome;
mod loader_rejection;
mod loader_state;
mod mailbox;
mod materializer;
mod outcome;
mod queue_capacity;
mod removal;
mod requirement;
mod resolution;
mod room;
mod routing;
mod routing_identity;
mod routing_runtime;
mod runtime;
mod runtime_hooks;
mod send_receipt;
mod subscription;

pub use binding::Binding;
pub use binding_hook::{BindingDecision, BindingHook, HookOrder};
pub use binding_policy::{BindingCandidate, BindingProposal};
pub use capability::Capability;
pub use component_definition::ComponentDefinition;
pub use component_instance::{ComponentInstance, ResolutionState};
pub use component_runtime::ComponentRuntime;
pub use context::{Context, ContextOwner, ContextVisibility};
pub use delivery::{Delivery, DeliveryProgress};
pub use driver::{DriverError, DriverProgress, EventLoopDriver, SequentialExecutor};
pub use effect::Effect;
pub use error::KernelError;
pub use event::KernelEvent;
pub use event_message::Event;
pub use execution_plan::{ExecutionFront, ExecutionPlan, ExecutionWork, RuntimeStart};
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
pub use loader::Loader;
pub use loader_error::LoaderError;
pub use loader_event::{ComponentSource, LoadRequest, LoaderEvent};
pub use loader_hook::{LoaderDecision, LoaderHook, LoaderProposal};
pub use loader_outcome::{LoadTransition, LoaderOutcome};
pub use loader_rejection::LoadRejection;
pub use loader_state::{LoadId, LoadPhase, LoadRecord};
pub use mailbox::Mailbox;
pub use materializer::{ComponentMaterializer, DeterministicMaterializer, MaterializerError};
pub use outcome::TransitionOutcome;
pub use queue_capacity::QueueCapacity;
pub use requirement::Requirement;
pub use room::{Room, RoomDeclaration};
pub use routing_identity::{EventId, EventTypeId, RoomAddress, RoomRuntimeId, RoomSequence};
pub use runtime::KernelRuntime;
pub use send_receipt::{DeliveryReceipt, DeliveryState, SendReceipt};
pub use subscription::{Subscription, SubscriptionDeclaration};
