//! Public-contract tests for FIFO send, Subscriptions, Mailboxes, and Receipts.

use std::collections::BTreeSet;

use meta_system_kernel::{
    ComponentDefinition, ComponentDefinitionId, ComponentInstanceId, ComponentRuntimeId, Delivery,
    DeliveryProgress, DeliveryState, DriverError, DriverProgress, Event, EventId, EventLoopDriver,
    EventTypeId, KernelError, KernelEvent, KernelRuntime, MailboxOverflowStrategy, MailboxPolicy,
    QueueCapacity, RoomAddress, RoomDeclaration, RoutingContract, SendReceipt, SequentialExecutor,
    SubscriptionDeclaration,
};

/// Driver fixture that confirms every accepted Delivery as processed.
#[derive(Debug, Default)]
struct ProcessingDriver {
    active: BTreeSet<ComponentInstanceId>,
}

impl EventLoopDriver for ProcessingDriver {
    fn start(
        &mut self,
        instance_id: ComponentInstanceId,
        _runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError> {
        self.active.insert(instance_id);
        Ok(())
    }

    fn advance(&mut self) -> Result<DriverProgress, DriverError> {
        Ok(DriverProgress::Idle)
    }

    fn process_delivery(&mut self, delivery: &Delivery) -> Result<DeliveryProgress, DriverError> {
        assert!(self.active.contains(&delivery.recipient()));
        Ok(DeliveryProgress::Processed)
    }

    fn stop(&mut self, instance_id: ComponentInstanceId) -> Result<(), DriverError> {
        self.active.remove(&instance_id);
        Ok(())
    }
}

/// Creates one small typed Event for routing assertions.
fn event(id: u64) -> Event {
    Event::new(EventId::new(id), EventTypeId::new(1), id.to_le_bytes())
}

/// Registers one complete Definition and asserts activation succeeded.
fn register<Driver: EventLoopDriver>(
    runtime: &mut KernelRuntime<Driver>,
    definition: ComponentDefinition,
    instance: ComponentInstanceId,
) {
    let _ = runtime
        .handle(KernelEvent::register_component(definition, instance))
        .expect("routing fixture Component activates");
}

/// Builds one owner and two potential recipients around a logical Room.
fn configure<Driver: EventLoopDriver>(runtime: &mut KernelRuntime<Driver>, mailbox: QueueCapacity) {
    let room = RoomAddress::new(10);
    let room_capacity = QueueCapacity::new(4).expect("fixture capacity is positive");
    let owner = ComponentDefinition::new(ComponentDefinitionId::new(1))
        .with_routing(RoutingContract::new().with_room(RoomDeclaration::new(room, room_capacity)));
    let subscriber = ComponentDefinition::new(ComponentDefinitionId::new(2)).with_routing(
        RoutingContract::new()
            .with_subscription(SubscriptionDeclaration::new(room))
            .with_mailbox_policy(MailboxPolicy::new(
                mailbox,
                MailboxOverflowStrategy::RejectNew,
            )),
    );
    let outsider = ComponentDefinition::new(ComponentDefinitionId::new(3)).with_routing(
        RoutingContract::new().with_mailbox_policy(MailboxPolicy::new(
            mailbox,
            MailboxOverflowStrategy::RejectNew,
        )),
    );
    register(runtime, owner, ComponentInstanceId::new(1));
    register(runtime, subscriber, ComponentInstanceId::new(2));
    register(runtime, outsider, ComponentInstanceId::new(3));
}

/// Asserts FIFO receipts and Mailbox ownership after three sends.
fn assert_fifo_distribution(runtime: &KernelRuntime, receipts: &[SendReceipt]) {
    assert!(receipts.iter().all(SendReceipt::accepted));
    assert_eq!(
        receipts
            .iter()
            .map(|receipt| receipt.sequence().value())
            .collect::<Vec<_>>(),
        vec![1, 2, 3]
    );
    assert!(receipts.iter().all(|receipt| {
        receipt.deliveries().len() == 1
            && receipt.deliveries()[0].recipient() == ComponentInstanceId::new(2)
            && receipt.deliveries()[0].delivered()
            && !receipt.deliveries()[0].processed()
    }));
    let graph = runtime.graph();
    let subscriber = graph
        .component_runtime(ComponentInstanceId::new(2))
        .expect("subscriber Runtime is Active");
    assert_eq!(
        subscriber
            .mailbox()
            .deliveries()
            .map(|delivery| delivery.event().id())
            .collect::<Vec<_>>(),
        vec![EventId::new(1), EventId::new(2), EventId::new(3)]
    );
    assert!(
        graph
            .component_runtime(ComponentInstanceId::new(3))
            .expect("outsider Runtime is Active")
            .mailbox()
            .is_empty()
    );
}

#[test]
fn send_distributes_fifo_only_to_subscribed_mailboxes() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = KernelRuntime::new();
    configure(
        &mut runtime,
        QueueCapacity::new(3).expect("fixture capacity is positive"),
    );
    let room = RoomAddress::new(10);

    // Act
    let receipts = [event(1), event(2), event(3)]
        .into_iter()
        .map(|event| runtime.send(room, event))
        .collect::<Result<Vec<_>, _>>()?;

    // Assert
    assert_fifo_distribution(&runtime, &receipts);
    Ok(())
}

#[test]
fn bounded_mailbox_reports_full_without_retry() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = KernelRuntime::new();
    configure(
        &mut runtime,
        QueueCapacity::new(1).expect("fixture capacity is positive"),
    );
    let room = RoomAddress::new(10);
    let _ = runtime.send(room, event(1))?;

    // Act
    let receipt = runtime.send(room, event(2))?;

    // Assert
    assert_eq!(receipt.deliveries()[0].state(), DeliveryState::RejectedFull);
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
        vec![EventId::new(1)]
    );
    Ok(())
}

#[test]
fn receipt_reports_processing_only_when_driver_observes_it() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = KernelRuntime::with_event_loop_driver(ProcessingDriver::default());
    configure(
        &mut runtime,
        QueueCapacity::new(1).expect("fixture capacity is positive"),
    );

    // Act
    let receipt = runtime.send(RoomAddress::new(10), event(1))?;

    // Assert
    assert!(receipt.deliveries()[0].processed());
    assert!(
        runtime
            .graph()
            .component_runtime(ComponentInstanceId::new(2))
            .expect("subscriber Runtime is Active")
            .mailbox()
            .is_empty()
    );
    Ok(())
}

#[test]
fn send_to_unavailable_logical_room_fails_explicitly() {
    // Arrange
    let mut runtime = KernelRuntime::<SequentialExecutor>::new();
    let room = RoomAddress::new(99);

    // Act
    let result = runtime.send(room, event(1));

    // Assert
    assert_eq!(result, Err(KernelError::UnavailableRoom(room)));
}
