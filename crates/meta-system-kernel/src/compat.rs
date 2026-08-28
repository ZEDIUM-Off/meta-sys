//! Temporary flat-path compatibility aliases hidden from generated documentation.
//!
//! New code should import through the documented domain modules. These aliases preserve the
//! prototype's existing callers while its public paths migrate without obscuring rustdoc.

pub use crate::execution::{
    DriverError, DriverProgress, EventLoopDriver, ExecutionFront, ExecutionPlan, ExecutionWork,
    RuntimeStart, SequentialExecutor,
};
pub use crate::loader::native::{
    NATIVE_COMPONENT_ABI_VERSION, NATIVE_COMPONENT_ENTRY_POINT, NativeCapabilityDescriptor,
    NativeComponentDescriptor, NativeMaterializer, NativeRequirementDescriptor,
};
pub use crate::loader::{
    ComponentMaterializer, ComponentSource, DeterministicMaterializer, LoadId, LoadPhase,
    LoadRecord, LoadRejection, LoadRequest, LoadTransition, LoaderDecision, LoaderError,
    LoaderEvent, LoaderHook, LoaderOutcome, LoaderProposal, MaterializerError,
};
pub use crate::resolution::{
    Binding, BindingCandidate, BindingDecision, BindingHook, BindingProposal, Capability,
    HookOrder, Requirement,
};
pub use crate::routing::{
    BroadcastReceipt, BroadcastSubscription, Delivery, DeliveryOrigin, DeliveryProgress,
    DeliveryReceipt, DeliveryState, EmissionDeclaration, Event, EventId, EventTypeId, Mailbox,
    MailboxOverflowStrategy, MailboxPolicy, QueueCapacity, Room, RoomAddress, RoomDeclaration,
    RoomRuntimeId, RoomSequence, RoutingContract, SendReceipt, Subscription,
    SubscriptionDeclaration,
};
pub use crate::runtime::{KernelError, KernelEvent, LifecycleTransition, TransitionOutcome};
pub use crate::system::{
    AddonId, CapabilityContractId, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstance, ComponentInstanceId, ComponentRuntime, ComponentRuntimeId, Context,
    ContextId, ContextOwner, ContextVisibility, Effect, EffectId, Facet, FacetId, FacetSchema,
    FacetSchemaId, FacetTarget, FacetValue, FacetValueKind, GraphEntityKind, RequirementId,
    ResolutionState, SystemGraph,
};
