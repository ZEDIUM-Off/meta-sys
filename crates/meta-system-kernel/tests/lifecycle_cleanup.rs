//! Public-contract tests for lifecycle-owned cleanup and reactivation.

use meta_system_kernel::{
    Binding, Capability, CapabilityContractId, CapabilityId, ComponentDefinition,
    ComponentDefinitionId, ComponentInstance, ComponentInstanceId, ComponentRuntime,
    ComponentRuntimeId, DriverError, DriverProgress, Effect, EffectId, EventLoopDriver,
    KernelError, KernelEvent, KernelRuntime, Requirement, RequirementId, ResolutionState,
    TransitionOutcome,
};

#[test]
fn active_component_can_record_owned_effect() -> Result<(), KernelError> {
    // Arrange
    let instance_id = ComponentInstanceId::new(10);
    let effect_id = EffectId::new(100);
    let definition = ComponentDefinition::new(ComponentDefinitionId::new(1));
    let mut runtime = KernelRuntime::new();
    let _registration = runtime.handle(KernelEvent::register_component(definition, instance_id))?;

    // Act
    let _effect_outcome = runtime.handle(KernelEvent::record_effect(Effect::new(
        effect_id,
        instance_id,
    )))?;

    // Assert
    assert_eq!(
        runtime.graph().effect(effect_id).map(Effect::owner),
        Some(instance_id)
    );
    Ok(())
}

#[test]
fn provider_removal_cleans_consumer_and_replacement_reactivates() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = bound_consumer_with_effect()?;

    // Act
    let removal = fixture
        .runtime
        .handle(KernelEvent::unregister_component(fixture.ids.provider))?;

    // Assert
    assert_consumer_cleaned(&fixture, &removal);

    // Act — introduce a replacement provider for the same Capability Contract.
    let replacement = ComponentDefinition::new(ComponentDefinitionId::new(3)).with_capability(
        Capability::new(CapabilityId::new(300), fixture.ids.contract),
    );
    let replacement_id = ComponentInstanceId::new(30);
    let reactivation = fixture
        .runtime
        .handle(KernelEvent::register_component(replacement, replacement_id))?;

    // Assert
    assert_consumer_reactivated(&fixture, &reactivation, replacement_id);
    Ok(())
}

#[test]
fn provider_removal_does_not_disturb_unrelated_binding() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = bound_consumer_with_effect()?;
    let contract_id = CapabilityContractId::new(9_000);
    let requirement_id = RequirementId::new(900);
    let consumer_id = ComponentInstanceId::new(90);
    let provider_id = ComponentInstanceId::new(91);
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(9))
        .with_requirement(Requirement::necessary(requirement_id, contract_id));
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(10))
        .with_capability(Capability::new(CapabilityId::new(900), contract_id));
    let _consumer = fixture
        .runtime
        .handle(KernelEvent::register_component(consumer, consumer_id))?;
    let _provider = fixture
        .runtime
        .handle(KernelEvent::register_component(provider, provider_id))?;
    let original_runtime = fixture
        .runtime
        .graph()
        .component_runtime(consumer_id)
        .map(ComponentRuntime::id);

    // Act
    let _removal = fixture
        .runtime
        .handle(KernelEvent::unregister_component(fixture.ids.provider))?;

    // Assert
    assert_eq!(
        fixture
            .runtime
            .graph()
            .binding(requirement_id)
            .map(Binding::provider_id),
        Some(provider_id)
    );
    assert_eq!(
        fixture
            .runtime
            .graph()
            .component_runtime(consumer_id)
            .map(ComponentRuntime::id),
        original_runtime
    );
    Ok(())
}

#[test]
fn driver_stop_failure_is_not_a_resolution_state() -> Result<(), KernelError> {
    // Arrange
    let instance_id = ComponentInstanceId::new(80);
    let definition = ComponentDefinition::new(ComponentDefinitionId::new(8));
    let mut runtime = KernelRuntime::with_event_loop_driver(StopRejectingDriver);
    let _registration = runtime.handle(KernelEvent::register_component(definition, instance_id))?;

    // Act
    let result = runtime.handle(KernelEvent::unregister_component(instance_id));

    // Assert
    assert!(matches!(
        result,
        Err(KernelError::DriverStop {
            instance_id: failed_id,
            ..
        }) if failed_id == instance_id
    ));
    assert_eq!(
        runtime
            .graph()
            .instance(instance_id)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Active)
    );
    assert!(runtime.graph().component_runtime(instance_id).is_some());
    Ok(())
}

#[test]
fn provider_removal_deactivates_transitive_consumers() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = active_dependency_chain()?;

    // Act
    let _outcome = fixture
        .runtime
        .handle(KernelEvent::unregister_component(fixture.root_provider))?;

    // Assert
    for instance_id in [fixture.middle_consumer, fixture.leaf_consumer] {
        assert_eq!(
            fixture
                .runtime
                .graph()
                .instance(instance_id)
                .map(ComponentInstance::resolution),
            Some(ResolutionState::Pending)
        );
        assert!(
            fixture
                .runtime
                .graph()
                .component_runtime(instance_id)
                .is_none()
        );
    }
    assert!(
        fixture
            .runtime
            .graph()
            .binding(fixture.middle_requirement)
            .is_none()
    );
    assert!(
        fixture
            .runtime
            .graph()
            .binding(fixture.leaf_requirement)
            .is_none()
    );
    Ok(())
}

/// Execution-boundary test adapter that accepts starts and rejects stops.
#[derive(Debug)]
struct StopRejectingDriver;

impl EventLoopDriver for StopRejectingDriver {
    fn start(
        &mut self,
        _instance_id: ComponentInstanceId,
        _runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError> {
        Ok(())
    }

    fn advance(&mut self) -> Result<DriverProgress, DriverError> {
        Ok(DriverProgress::Idle)
    }

    fn stop(&mut self, _instance_id: ComponentInstanceId) -> Result<(), DriverError> {
        Err(DriverError::new("stop rejected by test adapter"))
    }
}

/// Stable identities needed to inspect one cleanup and reactivation scenario.
struct CleanupIds {
    /// Capability Contract shared by consumer and providers.
    contract: CapabilityContractId,
    /// Consumer that survives provider replacement.
    consumer: ComponentInstanceId,
    /// Initial provider removed by the test.
    provider: ComponentInstanceId,
    /// Initial provider Definition removed with its Instance.
    provider_definition: ComponentDefinitionId,
    /// Consumer Requirement whose Binding is replaced.
    requirement: RequirementId,
    /// Initial provider Capability removed with its Instance.
    capability: CapabilityId,
    /// Consumer-owned Effect that must be cleaned.
    effect: EffectId,
    /// Concrete Runtime identity that must not survive reactivation.
    original_runtime: ComponentRuntimeId,
}

/// Runtime and identities prepared for a cleanup assertion.
struct CleanupFixture {
    /// Isolated Kernel Runtime under test.
    runtime: KernelRuntime,
    /// Scenario identities used only through graph observations.
    ids: CleanupIds,
}

/// Active three-Instance dependency chain used to verify transitive cleanup.
struct ChainFixture {
    /// Isolated Kernel Runtime containing the chain.
    runtime: KernelRuntime,
    /// Root provider removed by the scenario.
    root_provider: ComponentInstanceId,
    /// Middle consumer that also provides the leaf contract.
    middle_consumer: ComponentInstanceId,
    /// Leaf consumer transitively affected by root removal.
    leaf_consumer: ComponentInstanceId,
    /// Requirement binding the middle consumer to the root.
    middle_requirement: RequirementId,
    /// Requirement binding the leaf consumer to the middle.
    leaf_requirement: RequirementId,
}

/// Creates an Active root-provider to middle-provider to leaf-consumer chain.
fn active_dependency_chain() -> Result<ChainFixture, KernelError> {
    let root_provider = ComponentInstanceId::new(100);
    let middle_consumer = ComponentInstanceId::new(110);
    let leaf_consumer = ComponentInstanceId::new(120);
    let middle_requirement = RequirementId::new(1_100);
    let leaf_requirement = RequirementId::new(1_200);
    let root_contract = CapabilityContractId::new(10_000);
    let leaf_contract = CapabilityContractId::new(11_000);
    let root = ComponentDefinition::new(ComponentDefinitionId::new(100))
        .with_capability(Capability::new(CapabilityId::new(1_000), root_contract));
    let middle = ComponentDefinition::new(ComponentDefinitionId::new(110))
        .with_requirement(Requirement::necessary(middle_requirement, root_contract))
        .with_capability(Capability::new(CapabilityId::new(1_100), leaf_contract));
    let leaf = ComponentDefinition::new(ComponentDefinitionId::new(120))
        .with_requirement(Requirement::necessary(leaf_requirement, leaf_contract));
    let mut runtime = KernelRuntime::new();
    let _leaf = runtime.handle(KernelEvent::register_component(leaf, leaf_consumer))?;
    let _middle = runtime.handle(KernelEvent::register_component(middle, middle_consumer))?;
    let _root = runtime.handle(KernelEvent::register_component(root, root_provider))?;
    Ok(ChainFixture {
        runtime,
        root_provider,
        middle_consumer,
        leaf_consumer,
        middle_requirement,
        leaf_requirement,
    })
}

/// Creates an Active consumer bound to a provider and owning one Effect.
fn bound_consumer_with_effect() -> Result<CleanupFixture, KernelError> {
    let contract_id = CapabilityContractId::new(2_000);
    let consumer_id = ComponentInstanceId::new(10);
    let provider_id = ComponentInstanceId::new(20);
    let requirement_id = RequirementId::new(100);
    let capability_id = CapabilityId::new(200);
    let effect_id = EffectId::new(100);
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(1))
        .with_requirement(Requirement::necessary(requirement_id, contract_id));
    let provider_definition_id = ComponentDefinitionId::new(2);
    let provider = ComponentDefinition::new(provider_definition_id)
        .with_capability(Capability::new(capability_id, contract_id));
    let mut runtime = KernelRuntime::new();
    let _consumer = runtime.handle(KernelEvent::register_component(consumer, consumer_id))?;
    let _provider = runtime.handle(KernelEvent::register_component(provider, provider_id))?;
    let _effect = runtime.handle(KernelEvent::record_effect(Effect::new(
        effect_id,
        consumer_id,
    )))?;
    let original_runtime_id = runtime
        .graph()
        .component_runtime(consumer_id)
        .map(ComponentRuntime::id)
        .ok_or(KernelError::UnknownComponentInstance(consumer_id))?;
    Ok(CleanupFixture {
        runtime,
        ids: CleanupIds {
            contract: contract_id,
            consumer: consumer_id,
            provider: provider_id,
            provider_definition: provider_definition_id,
            requirement: requirement_id,
            capability: capability_id,
            effect: effect_id,
            original_runtime: original_runtime_id,
        },
    })
}

/// Verifies complete consumer cleanup and provider disappearance.
fn assert_consumer_cleaned(fixture: &CleanupFixture, outcome: &TransitionOutcome) {
    let graph = fixture.runtime.graph();
    let ids = &fixture.ids;
    assert!(graph.instance(ids.provider).is_none());
    assert!(graph.definition(ids.provider_definition).is_none());
    assert!(graph.capability(ids.capability).is_none());
    assert!(graph.binding(ids.requirement).is_none());
    assert_eq!(
        graph
            .instance(ids.consumer)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Pending)
    );
    assert!(graph.component_runtime(ids.consumer).is_none());
    assert!(graph.effect(ids.effect).is_none());
    assert_eq!(outcome.removed_bindings().len(), 1);
    assert_eq!(outcome.removed_effects(), &[ids.effect]);
}

/// Verifies that replacement creates a fresh consumer Runtime and Binding.
fn assert_consumer_reactivated(
    fixture: &CleanupFixture,
    outcome: &TransitionOutcome,
    replacement_id: ComponentInstanceId,
) {
    let graph = fixture.runtime.graph();
    let ids = &fixture.ids;
    assert_eq!(
        graph.binding(ids.requirement).map(Binding::provider_id),
        Some(replacement_id)
    );
    assert_eq!(
        graph
            .instance(ids.consumer)
            .map(ComponentInstance::resolution),
        Some(ResolutionState::Active)
    );
    let new_runtime_id = graph
        .component_runtime(ids.consumer)
        .map(ComponentRuntime::id);
    assert!(new_runtime_id.is_some());
    assert_ne!(new_runtime_id, Some(ids.original_runtime));
    assert_eq!(outcome.created_bindings().len(), 1);
}
