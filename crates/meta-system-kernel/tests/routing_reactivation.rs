//! Public contract for logical Room references across Runtime reactivation.

use meta_system_kernel::{
    Capability, CapabilityContractId, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstanceId, ComponentRuntime, Event, EventId, EventTypeId, KernelError, KernelEvent,
    KernelRuntime, QueueCapacity, Requirement, RequirementId, Room, RoomAddress, RoomDeclaration,
    RoomRuntimeId, RoutingContract, SubscriptionDeclaration,
};

/// Living routing fixture whose Room owner depends on a replaceable provider.
struct Fixture {
    runtime: KernelRuntime,
    room: RoomAddress,
    owner: ComponentInstanceId,
    provider: ComponentInstanceId,
    contract: CapabilityContractId,
}

/// Creates one typed Event for lifecycle-isolation assertions.
fn event(id: u64) -> Event {
    Event::new(EventId::new(id), EventTypeId::new(1), id.to_le_bytes())
}

/// Activates a self-subscribed Room owner through one provider Binding.
fn active_fixture() -> Result<Fixture, KernelError> {
    let room = RoomAddress::new(10);
    let owner = ComponentInstanceId::new(10);
    let provider = ComponentInstanceId::new(20);
    let contract = CapabilityContractId::new(100);
    let capacity = QueueCapacity::new(2).expect("fixture capacity is positive");
    let routing = RoutingContract::new()
        .with_room(RoomDeclaration::new(room, capacity))
        .with_subscription(SubscriptionDeclaration::new(room));
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(1))
        .with_requirement(Requirement::necessary(RequirementId::new(1), contract))
        .with_routing(routing);
    let provider_definition = ComponentDefinition::new(ComponentDefinitionId::new(2))
        .with_capability(Capability::new(CapabilityId::new(1), contract));
    let mut runtime = KernelRuntime::new();
    let _ = runtime.handle(KernelEvent::register_component(consumer, owner))?;
    let _ = runtime.handle(KernelEvent::register_component(
        provider_definition,
        provider,
    ))?;
    Ok(Fixture {
        runtime,
        room,
        owner,
        provider,
        contract,
    })
}

/// Returns concrete Room and Component Runtime identities for one active owner.
fn concrete_ids(fixture: &Fixture) -> (RoomRuntimeId, meta_system_kernel::ComponentRuntimeId) {
    let graph = fixture.runtime.graph();
    let room = graph.room(fixture.room).map(Room::id);
    let runtime = graph
        .component_runtime(fixture.owner)
        .map(ComponentRuntime::id);
    (
        room.expect("logical Room has one Active concrete lifecycle"),
        runtime.expect("owner has one Active Component Runtime"),
    )
}

/// Deactivates the owner and proves its concrete routing resources disappear.
fn deactivate_owner(
    fixture: &mut Fixture,
    logical_reference: RoomAddress,
) -> Result<(), KernelError> {
    let _ = fixture
        .runtime
        .handle(KernelEvent::unregister_component(fixture.provider))?;
    assert!(fixture.runtime.graph().room(logical_reference).is_none());
    assert!(
        fixture
            .runtime
            .graph()
            .component_runtime(fixture.owner)
            .is_none()
    );
    assert_eq!(
        fixture.runtime.send(logical_reference, event(2)),
        Err(KernelError::UnavailableRoom(logical_reference))
    );
    Ok(())
}

/// Introduces a compatible provider that reactivates the pending Room owner.
fn reactivate_owner(fixture: &mut Fixture) -> Result<(), KernelError> {
    let replacement = ComponentDefinition::new(ComponentDefinitionId::new(3))
        .with_capability(Capability::new(CapabilityId::new(2), fixture.contract));
    let _ = fixture.runtime.handle(KernelEvent::register_component(
        replacement,
        ComponentInstanceId::new(30),
    ))?;
    Ok(())
}

/// Proves concrete renewal and absence of queued work from the old lifecycle.
fn assert_fresh_lifecycle(
    fixture: &mut Fixture,
    logical_reference: RoomAddress,
    old_ids: (RoomRuntimeId, meta_system_kernel::ComponentRuntimeId),
) -> Result<(), KernelError> {
    assert_ne!(concrete_ids(fixture), old_ids);
    let graph = fixture.runtime.graph();
    assert!(
        graph
            .component_runtime(fixture.owner)
            .expect("owner reactivated")
            .mailbox()
            .is_empty()
    );
    let _ = fixture.runtime.send(logical_reference, event(3))?;
    let graph = fixture.runtime.graph();
    let event_ids = graph
        .component_runtime(fixture.owner)
        .expect("owner remains Active")
        .mailbox()
        .deliveries()
        .map(|delivery| delivery.event().id())
        .collect::<Vec<_>>();
    assert_eq!(event_ids, vec![EventId::new(3)]);
    Ok(())
}

#[test]
fn logical_room_reference_targets_new_empty_runtime_after_reactivation() -> Result<(), KernelError>
{
    // Arrange
    let mut fixture = active_fixture()?;
    let logical_reference = fixture.room;
    let _ = fixture.runtime.send(logical_reference, event(1))?;
    let old_ids = concrete_ids(&fixture);

    // Act and Assert
    deactivate_owner(&mut fixture, logical_reference)?;
    reactivate_owner(&mut fixture)?;
    assert_fresh_lifecycle(&mut fixture, logical_reference, old_ids)
}
