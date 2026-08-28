//! Public-contract tests for Capability resolution and activation.

use meta_system_kernel::{
    Binding, Capability, CapabilityContractId, CapabilityId, ComponentDefinition,
    ComponentDefinitionId, ComponentInstance, ComponentInstanceId, ComponentRuntimeId, DriverError,
    DriverProgress, EventLoopDriver, KernelError, KernelEvent, KernelRuntime, Requirement,
    RequirementId, ResolutionState, TransitionOutcome,
};

#[test]
fn compatible_capability_binds_and_activates_pending_consumer() -> Result<(), KernelError> {
    // Arrange
    let contract_id = CapabilityContractId::new(1_000);
    let requirement_id = RequirementId::new(100);
    let consumer_id = ComponentInstanceId::new(10);
    let provider_id = ComponentInstanceId::new(20);
    let capability_id = CapabilityId::new(200);
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(1))
        .with_requirement(Requirement::necessary(requirement_id, contract_id));
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(2))
        .with_capability(Capability::new(capability_id, contract_id));
    let mut runtime = KernelRuntime::new();
    let _consumer_outcome =
        runtime.handle(KernelEvent::register_component(consumer, consumer_id))?;
    assert_eq!(
        runtime
            .graph()
            .instance(consumer_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Pending)
    );

    // Act
    let outcome = runtime.handle(KernelEvent::register_component(provider, provider_id))?;

    // Assert
    let graph = runtime.graph();
    assert!(graph.capability(capability_id).is_some());
    assert_eq!(
        graph.binding(requirement_id).map(Binding::capability_id),
        Some(capability_id)
    );
    assert_eq!(
        graph.binding(requirement_id).map(Binding::provider_id),
        Some(provider_id)
    );
    assert_eq!(
        graph
            .instance(consumer_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Active)
    );
    assert!(graph.component_runtime(consumer_id).is_some());
    assert_activation_outcome(&outcome, consumer_id);
    Ok(())
}

/// Verifies that an outcome explains one Pending-to-Active resolution.
fn assert_activation_outcome(outcome: &TransitionOutcome, consumer_id: ComponentInstanceId) {
    assert_eq!(outcome.created_bindings().len(), 1);
    assert!(outcome.transitions().iter().any(|transition| {
        transition.instance_id() == consumer_id
            && transition.previous() == Some(ResolutionState::Pending)
            && transition.current() == ResolutionState::Active
    }));
}

#[test]
fn incompatible_capability_leaves_consumer_pending() -> Result<(), KernelError> {
    // Arrange
    let requirement_id = RequirementId::new(300);
    let consumer_id = ComponentInstanceId::new(30);
    let provider_id = ComponentInstanceId::new(40);
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(3)).with_requirement(
        Requirement::necessary(requirement_id, CapabilityContractId::new(3_000)),
    );
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(4)).with_capability(
        Capability::new(CapabilityId::new(400), CapabilityContractId::new(4_000)),
    );
    let mut runtime = KernelRuntime::new();
    let _consumer_outcome =
        runtime.handle(KernelEvent::register_component(consumer, consumer_id))?;

    // Act
    let outcome = runtime.handle(KernelEvent::register_component(provider, provider_id))?;

    // Assert
    assert!(runtime.graph().binding(requirement_id).is_none());
    assert_eq!(
        runtime
            .graph()
            .instance(consumer_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Pending)
    );
    assert!(runtime.graph().component_runtime(consumer_id).is_none());
    assert_eq!(
        runtime
            .graph()
            .instance(provider_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Active)
    );
    assert!(outcome.created_bindings().is_empty());
    Ok(())
}

#[test]
fn active_provider_resolves_consumer_registered_later() -> Result<(), KernelError> {
    // Arrange
    let contract_id = CapabilityContractId::new(5_000);
    let capability_id = CapabilityId::new(500);
    let requirement_id = RequirementId::new(500);
    let provider_id = ComponentInstanceId::new(50);
    let consumer_id = ComponentInstanceId::new(60);
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(5))
        .with_capability(Capability::new(capability_id, contract_id));
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(6))
        .with_requirement(Requirement::necessary(requirement_id, contract_id));
    let mut runtime = KernelRuntime::new();
    let _provider_outcome =
        runtime.handle(KernelEvent::register_component(provider, provider_id))?;

    // Act
    let outcome = runtime.handle(KernelEvent::register_component(consumer, consumer_id))?;

    // Assert
    assert_eq!(
        runtime
            .graph()
            .binding(requirement_id)
            .map(Binding::provider_id),
        Some(provider_id)
    );
    assert!(runtime.graph().component_runtime(consumer_id).is_some());
    assert_eq!(outcome.created_bindings().len(), 1);
    Ok(())
}

#[test]
fn driver_start_failure_does_not_publish_active_runtime() {
    // Arrange
    let instance_id = ComponentInstanceId::new(70);
    let definition = ComponentDefinition::new(ComponentDefinitionId::new(7));
    let mut runtime = KernelRuntime::with_event_loop_driver(RejectingDriver);

    // Act
    let result = runtime.handle(KernelEvent::register_component(definition, instance_id));

    // Assert
    assert!(matches!(
        result,
        Err(KernelError::DriverStart {
            instance_id: failed_id,
            ..
        }) if failed_id == instance_id
    ));
    assert_eq!(
        runtime
            .graph()
            .instance(instance_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Pending)
    );
    assert!(runtime.graph().component_runtime(instance_id).is_none());
}

/// Execution-boundary test adapter that rejects every startup.
#[derive(Debug)]
struct RejectingDriver;

impl EventLoopDriver for RejectingDriver {
    fn start(
        &mut self,
        _instance_id: ComponentInstanceId,
        _runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError> {
        Err(DriverError::new("startup rejected by test adapter"))
    }

    fn advance(&mut self) -> Result<DriverProgress, DriverError> {
        Ok(DriverProgress::Idle)
    }

    fn stop(&mut self, _instance_id: ComponentInstanceId) -> Result<(), DriverError> {
        Ok(())
    }
}
