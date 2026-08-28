//! Public-contract tests for Component-owned Mailbox overflow strategies.

use meta_system_kernel::{
    ComponentDefinition, ComponentDefinitionId, ComponentInstanceId, DeliveryState, Event, EventId,
    EventTypeId, KernelError, KernelEvent, KernelRuntime, MailboxOverflowStrategy, MailboxPolicy,
    QueueCapacity, RoomAddress, RoomDeclaration, RoutingContract, SubscriptionDeclaration,
};

/// Creates one typed Event for overflow observations.
fn event(id: u64) -> Event {
    Event::new(EventId::new(id), EventTypeId::new(1), id.to_le_bytes())
}

/// Builds one Room and one subscriber using the selected overflow strategy.
fn runtime(strategy: MailboxOverflowStrategy) -> KernelRuntime {
    let mut runtime = KernelRuntime::new();
    let room = RoomAddress::new(10);
    let capacity = QueueCapacity::new(1).expect("fixture capacity is positive");
    let owner = ComponentDefinition::new(ComponentDefinitionId::new(1))
        .with_routing(RoutingContract::new().with_room(RoomDeclaration::new(room, capacity)));
    let subscriber = ComponentDefinition::new(ComponentDefinitionId::new(2)).with_routing(
        RoutingContract::new()
            .with_subscription(SubscriptionDeclaration::new(room))
            .with_mailbox_policy(MailboxPolicy::new(capacity, strategy)),
    );
    for (definition, instance) in [(owner, 1), (subscriber, 2)] {
        let _ = runtime
            .handle(KernelEvent::register_component(
                definition,
                ComponentInstanceId::new(instance),
            ))
            .expect("overflow fixture Component activates");
    }
    runtime
}

#[test]
fn reject_new_preserves_oldest_delivery_without_retry() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = runtime(MailboxOverflowStrategy::RejectNew);
    let room = RoomAddress::new(10);
    let _ = runtime.send(room, event(1))?;

    // Act
    let receipt = runtime.send(room, event(2))?;

    // Assert
    assert_eq!(receipt.deliveries()[0].state(), DeliveryState::RejectedFull);
    let graph = runtime.graph();
    let event_ids = graph
        .component_runtime(ComponentInstanceId::new(2))
        .expect("subscriber Runtime is Active")
        .mailbox()
        .deliveries()
        .map(|delivery| delivery.event().id())
        .collect::<Vec<_>>();
    assert_eq!(event_ids, vec![EventId::new(1)]);
    Ok(())
}

#[test]
fn drop_oldest_accepts_new_delivery_once_without_retry() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = runtime(MailboxOverflowStrategy::DropOldest);
    let room = RoomAddress::new(10);
    let _ = runtime.send(room, event(1))?;

    // Act
    let receipt = runtime.send(room, event(2))?;

    // Assert
    assert_eq!(
        receipt.deliveries()[0].state(),
        DeliveryState::DeliveredAfterDroppingOldest
    );
    let graph = runtime.graph();
    let mailbox = graph
        .component_runtime(ComponentInstanceId::new(2))
        .expect("subscriber Runtime is Active")
        .mailbox();
    assert_eq!(mailbox.len(), 1);
    assert_eq!(
        mailbox
            .deliveries()
            .map(|delivery| delivery.event().id())
            .collect::<Vec<_>>(),
        vec![EventId::new(2)]
    );
    Ok(())
}
