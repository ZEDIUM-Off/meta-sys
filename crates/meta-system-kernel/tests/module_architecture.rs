//! Public module paths that make the Kernel architecture navigable.

use meta_system_kernel::execution::{
    DriverError, DriverProgress, EventLoopDriver, ExecutionFront, ExecutionPlan, ExecutionWork,
    RuntimeStart, SequentialExecutor,
};
use meta_system_kernel::loader::native::{NativeComponentDescriptor, NativeMaterializer};
use meta_system_kernel::loader::{
    ComponentMaterializer, ComponentSource, DeterministicMaterializer, LoadId, LoadPhase,
    LoadRecord, LoadRequest, Loader, LoaderDecision, LoaderError, LoaderEvent, LoaderHook,
    LoaderOutcome, MaterializerError,
};
use meta_system_kernel::resolution::{
    Binding, BindingCandidate, BindingDecision, BindingHook, BindingProposal, Capability,
    Requirement,
};
use meta_system_kernel::routing::{
    BroadcastReceipt, Delivery, Event, Mailbox, MailboxPolicy, Room, RoomDeclaration,
    RoutingContract, SendReceipt, Subscription,
};
use meta_system_kernel::runtime::{KernelError, KernelEvent, KernelRuntime, TransitionOutcome};
use meta_system_kernel::system::{
    ComponentDefinition, ComponentInstance, ComponentRuntime, Context, Effect, Facet, SystemGraph,
};

/// Proves that the Kernel transition seam is grouped under the Runtime domain.
#[test]
fn runtime_module_exposes_kernel_transition_seam() {
    let _runtime = KernelRuntime::new();
    let _event_type = std::any::type_name::<KernelEvent>();
    let _outcome_type = std::any::type_name::<TransitionOutcome>();
    let _error_type = std::any::type_name::<KernelError>();
}

/// Proves that the living graph model is grouped under the System domain.
#[test]
fn system_module_exposes_graph_observation_and_entities() {
    let runtime = KernelRuntime::new();
    let _graph: SystemGraph<'_> = runtime.graph();
    let _definition_type = std::any::type_name::<ComponentDefinition>();
    let _instance_type = std::any::type_name::<ComponentInstance>();
    let _runtime_type = std::any::type_name::<ComponentRuntime>();
    let _context_type = std::any::type_name::<Context>();
    let _facet_type = std::any::type_name::<Facet>();
    let _effect_type = std::any::type_name::<Effect>();
}

/// Proves that dependency declarations and policy decisions share one Resolution domain.
#[test]
fn resolution_module_exposes_binding_contract() {
    let _requirement_type = std::any::type_name::<Requirement>();
    let _capability_type = std::any::type_name::<Capability>();
    let _binding_type = std::any::type_name::<Binding>();
    let _candidate_type = std::any::type_name::<BindingCandidate>();
    let _proposal_type = std::any::type_name::<BindingProposal>();
    let _decision_type = std::any::type_name::<BindingDecision>();
    let _hook_type = std::any::type_name::<&dyn BindingHook>();
}

/// Proves that execution plans and replaceable Drivers share one Execution domain.
#[test]
fn execution_module_exposes_driver_seam_and_plans() {
    fn accepts_driver(_driver: &impl EventLoopDriver) {}

    let driver = SequentialExecutor::default();
    accepts_driver(&driver);
    let _error_type = std::any::type_name::<DriverError>();
    let _progress_type = std::any::type_name::<DriverProgress>();
    let _front_type = std::any::type_name::<ExecutionFront>();
    let _plan_type = std::any::type_name::<ExecutionPlan>();
    let _work_type = std::any::type_name::<ExecutionWork>();
    let _start_type = std::any::type_name::<RuntimeStart>();
}

/// Proves that declarations, living queues, and observations share one Routing domain.
#[test]
fn routing_module_exposes_event_distribution_contract() {
    let _event_type = std::any::type_name::<Event>();
    let _contract_type = std::any::type_name::<RoutingContract>();
    let _room_declaration_type = std::any::type_name::<RoomDeclaration>();
    let _room_type = std::any::type_name::<Room>();
    let _subscription_type = std::any::type_name::<Subscription>();
    let _mailbox_policy_type = std::any::type_name::<MailboxPolicy>();
    let _mailbox_type = std::any::type_name::<Mailbox>();
    let _delivery_type = std::any::type_name::<Delivery>();
    let _send_receipt_type = std::any::type_name::<SendReceipt>();
    let _broadcast_receipt_type = std::any::type_name::<BroadcastReceipt>();
}

/// Proves that the Loader machine, its adapter seam, and native support are navigable together.
#[test]
fn loader_module_exposes_lifecycle_and_materializer_seams() {
    let _loader_type = std::any::type_name::<Loader<DeterministicMaterializer>>();
    let _event_type = std::any::type_name::<LoaderEvent>();
    let _outcome_type = std::any::type_name::<LoaderOutcome>();
    let _error_type = std::any::type_name::<LoaderError>();
    let _load_id_type = std::any::type_name::<LoadId>();
    let _phase_type = std::any::type_name::<LoadPhase>();
    let _record_type = std::any::type_name::<LoadRecord>();
    let _source_type = std::any::type_name::<ComponentSource>();
    let _request_type = std::any::type_name::<LoadRequest>();
    let _decision_type = std::any::type_name::<LoaderDecision>();
    let _hook_type = std::any::type_name::<&dyn LoaderHook>();
    let _materializer_type = std::any::type_name::<&dyn ComponentMaterializer>();
    let _materializer_error_type = std::any::type_name::<MaterializerError>();
    let _native_adapter_type = std::any::type_name::<NativeMaterializer>();
    let _native_descriptor_type = std::any::type_name::<NativeComponentDescriptor>();
}
