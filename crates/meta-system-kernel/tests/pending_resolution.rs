//! Public-contract tests for unresolved Component registration.

use meta_system_kernel::{
    CapabilityContractId, ComponentDefinition, ComponentDefinitionId, ComponentInstance,
    ComponentInstanceId, KernelError, KernelEvent, KernelRuntime, Requirement, RequirementId,
    ResolutionState,
};

#[test]
fn necessary_requirement_without_provider_keeps_instance_pending() {
    // Arrange
    let definition_id = ComponentDefinitionId::new(1);
    let instance_id = ComponentInstanceId::new(10);
    let requirement_id = RequirementId::new(100);
    let requirement = Requirement::necessary(requirement_id, CapabilityContractId::new(1_000));
    let definition = ComponentDefinition::new(definition_id).with_requirement(requirement);
    let mut runtime = KernelRuntime::new();

    // Act
    let result = runtime.handle(KernelEvent::register_component(definition, instance_id));

    // Assert
    assert!(result.is_ok());
    let graph = runtime.graph();
    assert!(graph.definition(definition_id).is_some());
    assert!(graph.requirement(requirement_id).is_some());
    assert_eq!(
        graph
            .instance(instance_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Pending)
    );
    assert!(graph.binding(requirement_id).is_none());
    assert!(graph.component_runtime(instance_id).is_none());
}

#[test]
fn kernel_runtimes_keep_graph_state_isolated() {
    // Arrange
    let definition_id = ComponentDefinitionId::new(2);
    let instance_id = ComponentInstanceId::new(20);
    let mut populated_runtime = KernelRuntime::new();
    let isolated_runtime = KernelRuntime::new();
    let event = pending_registration(definition_id, instance_id, RequirementId::new(200));

    // Act
    let result = populated_runtime.handle(event);

    // Assert
    assert!(result.is_ok());
    assert!(populated_runtime.graph().instance(instance_id).is_some());
    assert!(isolated_runtime.graph().definition(definition_id).is_none());
    assert!(isolated_runtime.graph().instance(instance_id).is_none());
}

#[test]
fn duplicate_definition_rejects_whole_registration() {
    // Arrange
    let definition_id = ComponentDefinitionId::new(3);
    let first_instance_id = ComponentInstanceId::new(30);
    let duplicate_instance_id = ComponentInstanceId::new(31);
    let mut runtime = KernelRuntime::new();
    let first = pending_registration(definition_id, first_instance_id, RequirementId::new(300));
    let duplicate = pending_registration(
        definition_id,
        duplicate_instance_id,
        RequirementId::new(301),
    );
    assert!(runtime.handle(first).is_ok());

    // Act
    let result = runtime.handle(duplicate);

    // Assert
    assert_eq!(
        result,
        Err(KernelError::DuplicateComponentDefinition(definition_id))
    );
    assert!(runtime.graph().instance(first_instance_id).is_some());
    assert!(runtime.graph().instance(duplicate_instance_id).is_none());
    assert!(
        runtime
            .graph()
            .requirement(RequirementId::new(301))
            .is_none()
    );
}

#[test]
fn duplicate_instance_rejects_whole_registration() {
    // Arrange
    let instance_id = ComponentInstanceId::new(40);
    let mut runtime = KernelRuntime::new();
    let first = pending_registration(
        ComponentDefinitionId::new(4),
        instance_id,
        RequirementId::new(400),
    );
    let duplicate = pending_registration(
        ComponentDefinitionId::new(5),
        instance_id,
        RequirementId::new(401),
    );
    assert!(runtime.handle(first).is_ok());

    // Act
    let result = runtime.handle(duplicate);

    // Assert
    assert_eq!(
        result,
        Err(KernelError::DuplicateComponentInstance(instance_id))
    );
    assert!(
        runtime
            .graph()
            .definition(ComponentDefinitionId::new(5))
            .is_none()
    );
    assert!(
        runtime
            .graph()
            .requirement(RequirementId::new(401))
            .is_none()
    );
}

#[test]
fn duplicate_requirement_rejects_whole_registration() {
    // Arrange
    let duplicate_requirement_id = RequirementId::new(500);
    let definition_id = ComponentDefinitionId::new(6);
    let instance_id = ComponentInstanceId::new(60);
    let first = pending_registration(
        ComponentDefinitionId::new(5),
        ComponentInstanceId::new(50),
        duplicate_requirement_id,
    );
    let duplicate = pending_registration(definition_id, instance_id, duplicate_requirement_id);
    let mut runtime = KernelRuntime::new();
    assert!(runtime.handle(first).is_ok());

    // Act
    let result = runtime.handle(duplicate);

    // Assert
    assert_eq!(
        result,
        Err(KernelError::DuplicateRequirement(duplicate_requirement_id))
    );
    assert!(runtime.graph().definition(definition_id).is_none());
    assert!(runtime.graph().instance(instance_id).is_none());
}

/// Builds the smallest complete registration containing one necessary Requirement.
fn pending_registration(
    definition_id: ComponentDefinitionId,
    instance_id: ComponentInstanceId,
    requirement_id: RequirementId,
) -> KernelEvent {
    let requirement = Requirement::necessary(requirement_id, CapabilityContractId::new(2_000));
    let definition = ComponentDefinition::new(definition_id).with_requirement(requirement);
    KernelEvent::register_component(definition, instance_id)
}
