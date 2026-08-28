//! Public-contract tests for emit, broadcast, and independent Room progress.

use std::{
    collections::BTreeSet,
    sync::{
        Arc, Barrier, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use meta_system_kernel::{
    BroadcastSubscription, ComponentDefinition, ComponentDefinitionId, ComponentInstanceId,
    ComponentRuntimeId, Delivery, DeliveryOrigin, DeliveryProgress, DriverError, DriverProgress,
    EmissionDeclaration, Event, EventId, EventLoopDriver, EventTypeId, KernelError, KernelEvent,
    KernelRuntime, QueueCapacity, RoomAddress, RoomDeclaration, RoutingContract, SendReceipt,
    SubscriptionDeclaration,
};

/// Test Driver that overlaps every independent Delivery frontier.
#[derive(Debug, Default)]
struct ConcurrentDeliveryDriver {
    active_instances: BTreeSet<ComponentInstanceId>,
    front_sizes: Arc<Mutex<Vec<usize>>>,
    maximum_overlap: Arc<AtomicUsize>,
}

/// Synchronizes one test worker so independent Deliveries visibly overlap.
fn overlap_worker(
    barrier: &Barrier,
    active: &AtomicUsize,
    maximum: &AtomicUsize,
) -> DeliveryProgress {
    let current = active.fetch_add(1, Ordering::SeqCst) + 1;
    maximum.fetch_max(current, Ordering::SeqCst);
    barrier.wait();
    active.fetch_sub(1, Ordering::SeqCst);
    DeliveryProgress::Processed
}

impl EventLoopDriver for ConcurrentDeliveryDriver {
    fn start(
        &mut self,
        instance_id: ComponentInstanceId,
        _runtime_id: ComponentRuntimeId,
    ) -> Result<(), DriverError> {
        self.active_instances.insert(instance_id);
        Ok(())
    }

    fn advance(&mut self) -> Result<DriverProgress, DriverError> {
        Ok(DriverProgress::Idle)
    }

    fn process_delivery_front(
        &mut self,
        deliveries: &[Delivery],
    ) -> Result<Vec<DeliveryProgress>, DriverError> {
        self.front_sizes
            .lock()
            .expect("front log remains available")
            .push(deliveries.len());
        let barrier = Arc::new(Barrier::new(deliveries.len()));
        let active = Arc::new(AtomicUsize::new(0));
        let maximum = Arc::clone(&self.maximum_overlap);
        Ok(thread::scope(|scope| {
            deliveries
                .iter()
                .map(|_| {
                    let barrier = Arc::clone(&barrier);
                    let active = Arc::clone(&active);
                    let maximum = Arc::clone(&maximum);
                    scope.spawn(move || overlap_worker(&barrier, &active, &maximum))
                })
                .collect::<Vec<_>>()
                .into_iter()
                .map(|worker| worker.join().expect("front worker completes"))
                .collect()
        }))
    }

    fn stop(&mut self, instance_id: ComponentInstanceId) -> Result<(), DriverError> {
        self.active_instances.remove(&instance_id);
        Ok(())
    }
}

/// Creates one typed Event using the common routing concept.
fn event(id: u64, event_type: u64) -> Event {
    Event::new(
        EventId::new(id),
        EventTypeId::new(event_type),
        id.to_le_bytes(),
    )
}

/// Registers one fixture Component and requires successful activation.
fn register<Driver: EventLoopDriver>(
    runtime: &mut KernelRuntime<Driver>,
    definition: ComponentDefinition,
    instance: u64,
) {
    let _ = runtime
        .handle(KernelEvent::register_component(
            definition,
            ComponentInstanceId::new(instance),
        ))
        .expect("publishing fixture Component activates");
}

/// Declares one emitter, two targeted listeners, and one unrelated Component.
fn configure<Driver: EventLoopDriver>(runtime: &mut KernelRuntime<Driver>) {
    let capacity = QueueCapacity::new(8).expect("fixture capacity is positive");
    let first = RoomAddress::new(10);
    let second = RoomAddress::new(20);
    let source = RoutingContract::new()
        .with_room(RoomDeclaration::new(first, capacity))
        .with_room(RoomDeclaration::new(second, capacity))
        .with_emission(EmissionDeclaration::new(EventTypeId::new(1), first))
        .with_emission(EmissionDeclaration::new(EventTypeId::new(1), second));
    let first_listener = RoutingContract::new()
        .with_subscription(SubscriptionDeclaration::new(first))
        .with_broadcast_subscription(BroadcastSubscription::new(EventTypeId::new(2)));
    let second_listener = RoutingContract::new()
        .with_subscription(SubscriptionDeclaration::new(second))
        .with_broadcast_subscription(BroadcastSubscription::new(EventTypeId::new(2)));
    register(
        runtime,
        ComponentDefinition::new(ComponentDefinitionId::new(1)).with_routing(source),
        1,
    );
    register(
        runtime,
        ComponentDefinition::new(ComponentDefinitionId::new(2)).with_routing(first_listener),
        2,
    );
    register(
        runtime,
        ComponentDefinition::new(ComponentDefinitionId::new(3)).with_routing(second_listener),
        3,
    );
    register(
        runtime,
        ComponentDefinition::new(ComponentDefinitionId::new(4)),
        4,
    );
}

#[test]
fn emit_reaches_only_subscribers_declared_by_the_source_contract() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = KernelRuntime::new();
    configure(&mut runtime);

    // Act
    let receipts = runtime.emit(ComponentInstanceId::new(1), &event(1, 1))?;

    // Assert
    assert_eq!(
        receipts.iter().map(SendReceipt::room).collect::<Vec<_>>(),
        vec![RoomAddress::new(10), RoomAddress::new(20)]
    );
    assert_eq!(
        receipts[0].deliveries()[0].recipient(),
        ComponentInstanceId::new(2)
    );
    assert_eq!(
        receipts[1].deliveries()[0].recipient(),
        ComponentInstanceId::new(3)
    );
    assert!(
        runtime
            .graph()
            .component_runtime(ComponentInstanceId::new(4))
            .expect("unrelated Runtime is Active")
            .mailbox()
            .is_empty()
    );
    Ok(())
}

#[test]
fn broadcast_reaches_only_registered_active_listeners() -> Result<(), KernelError> {
    // Arrange
    let mut runtime = KernelRuntime::new();
    configure(&mut runtime);

    // Act
    let receipt = runtime.broadcast(&event(2, 2))?;

    // Assert
    assert_eq!(
        receipt
            .deliveries()
            .iter()
            .map(|delivery| delivery.recipient())
            .collect::<Vec<_>>(),
        vec![ComponentInstanceId::new(2), ComponentInstanceId::new(3)]
    );
    for recipient in [2, 3] {
        let graph = runtime.graph();
        let delivery = graph
            .component_runtime(ComponentInstanceId::new(recipient))
            .expect("listener Runtime is Active")
            .mailbox()
            .deliveries()
            .next()
            .expect("broadcast Delivery remains pending");
        assert_eq!(delivery.origin(), DeliveryOrigin::Broadcast);
        assert_eq!(delivery.event().id(), EventId::new(2));
    }
    Ok(())
}

#[test]
fn independent_emit_rooms_form_one_concurrent_driver_front() -> Result<(), KernelError> {
    // Arrange
    let driver = ConcurrentDeliveryDriver::default();
    let front_sizes = Arc::clone(&driver.front_sizes);
    let maximum_overlap = Arc::clone(&driver.maximum_overlap);
    let mut runtime = KernelRuntime::with_event_loop_driver(driver);
    configure(&mut runtime);

    // Act
    let receipts = runtime.emit(ComponentInstanceId::new(1), &event(1, 1))?;

    // Assert
    assert!(
        receipts
            .iter()
            .all(|receipt| receipt.deliveries()[0].processed())
    );
    assert_eq!(
        front_sizes
            .lock()
            .expect("front log remains available")
            .as_slice(),
        &[2]
    );
    assert_eq!(maximum_overlap.load(Ordering::SeqCst), 2);
    Ok(())
}

#[test]
fn emit_rejects_event_type_absent_from_source_contract() {
    // Arrange
    let mut runtime = KernelRuntime::new();
    configure(&mut runtime);
    let source = ComponentInstanceId::new(1);
    let event_type = EventTypeId::new(99);

    // Act
    let result = runtime.emit(source, &event(1, 99));

    // Assert
    assert_eq!(
        result,
        Err(KernelError::UndeclaredEmission {
            emitter: source,
            event_type,
        })
    );
}
