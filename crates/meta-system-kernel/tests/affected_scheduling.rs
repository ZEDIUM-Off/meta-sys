//! Public-contract tests for affected work and dependency frontiers.

use std::{
    sync::{
        Arc, Barrier,
        atomic::{AtomicUsize, Ordering},
    },
    thread,
};

use meta_system_kernel::{
    Capability, CapabilityContractId, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstanceId, ComponentRuntimeId, DriverError, DriverProgress, EventLoopDriver,
    ExecutionWork, KernelError, KernelEvent, KernelRuntime, Requirement, RequirementId,
    RuntimeStart, TransitionOutcome,
};

#[test]
fn provider_mutation_places_independent_consumers_in_same_front() -> Result<(), KernelError> {
    // Arrange and act
    let outcome = run_independent_consumers(KernelRuntime::new())?;

    // Assert
    let provider = ComponentInstanceId::new(30);
    let fronts = outcome.execution_plan().fronts();
    assert_eq!(fronts.len(), 2);
    assert_eq!(fronts[0].work().len(), 1);
    assert_eq!(fronts[0].work()[0].instance(), provider);
    assert_eq!(
        fronts[1]
            .work()
            .iter()
            .map(ExecutionWork::instance)
            .collect::<Vec<_>>(),
        vec![ComponentInstanceId::new(10), ComponentInstanceId::new(20)]
    );
    assert!(
        fronts[1]
            .work()
            .iter()
            .all(|work| work.dependencies() == [provider])
    );
    Ok(())
}

#[test]
fn concurrent_driver_overlaps_independent_front_without_changing_outcome() -> Result<(), KernelError>
{
    // Arrange
    let sequential_outcome = run_independent_consumers(KernelRuntime::new())?;
    let (driver, maximum_concurrency) = ConcurrentProbe::new();
    let concurrent_runtime = KernelRuntime::with_event_loop_driver(driver);

    // Act
    let concurrent_outcome = run_independent_consumers(concurrent_runtime)?;

    // Assert
    assert_eq!(concurrent_outcome, sequential_outcome);
    assert_eq!(maximum_concurrency.load(Ordering::SeqCst), 2);
    Ok(())
}

#[test]
fn transitive_dependencies_produce_deterministic_ordered_fronts() -> Result<(), KernelError> {
    // Arrange
    let root_contract = CapabilityContractId::new(2_000);
    let leaf_contract = CapabilityContractId::new(3_000);
    let root = ComponentInstanceId::new(50);
    let middle = ComponentInstanceId::new(60);
    let leaf = ComponentInstanceId::new(70);
    let leaf_definition = ComponentDefinition::new(ComponentDefinitionId::new(7)).with_requirement(
        Requirement::necessary(RequirementId::new(700), leaf_contract),
    );
    let middle_definition = ComponentDefinition::new(ComponentDefinitionId::new(6))
        .with_requirement(Requirement::necessary(
            RequirementId::new(600),
            root_contract,
        ))
        .with_capability(Capability::new(CapabilityId::new(600), leaf_contract));
    let root_definition = ComponentDefinition::new(ComponentDefinitionId::new(5))
        .with_capability(Capability::new(CapabilityId::new(500), root_contract));
    let mut runtime = KernelRuntime::new();
    let _leaf = runtime.handle(KernelEvent::register_component(leaf_definition, leaf))?;
    let _middle = runtime.handle(KernelEvent::register_component(middle_definition, middle))?;

    // Act
    let outcome = runtime.handle(KernelEvent::register_component(root_definition, root))?;

    // Assert
    let fronts = outcome.execution_plan().fronts();
    assert_eq!(fronts.len(), 3);
    assert_eq!(fronts[0].work()[0].instance(), root);
    assert_eq!(fronts[1].work()[0].instance(), middle);
    assert_eq!(fronts[1].work()[0].dependencies(), &[root]);
    assert_eq!(fronts[2].work()[0].instance(), leaf);
    assert_eq!(fronts[2].work()[0].dependencies(), &[middle]);
    Ok(())
}

/// Runs one provider mutation with two affected and one unrelated consumer.
fn run_independent_consumers<Driver: EventLoopDriver>(
    mut runtime: KernelRuntime<Driver>,
) -> Result<TransitionOutcome, KernelError> {
    let contract = CapabilityContractId::new(1_000);
    for (definition, instance, requirement) in [(1, 10, 100), (2, 20, 200)] {
        let component =
            ComponentDefinition::new(ComponentDefinitionId::new(definition)).with_requirement(
                Requirement::necessary(RequirementId::new(requirement), contract),
            );
        let _consumer = runtime.handle(KernelEvent::register_component(
            component,
            ComponentInstanceId::new(instance),
        ))?;
    }
    let unrelated = ComponentDefinition::new(ComponentDefinitionId::new(4)).with_requirement(
        Requirement::necessary(RequirementId::new(400), CapabilityContractId::new(4_000)),
    );
    let _unrelated = runtime.handle(KernelEvent::register_component(
        unrelated,
        ComponentInstanceId::new(40),
    ))?;
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(3))
        .with_capability(Capability::new(CapabilityId::new(300), contract));
    runtime.handle(KernelEvent::register_component(
        provider,
        ComponentInstanceId::new(30),
    ))
}

/// Test Driver that overlaps every Runtime start within one executable front.
#[derive(Debug)]
struct ConcurrentProbe {
    /// Number of startup tasks currently inside the local barrier.
    active: Arc<AtomicUsize>,
    /// Highest overlap observed across all fronts.
    maximum: Arc<AtomicUsize>,
}

impl ConcurrentProbe {
    /// Creates a Driver and a retained observation of its maximum overlap.
    fn new() -> (Self, Arc<AtomicUsize>) {
        let maximum = Arc::new(AtomicUsize::new(0));
        (
            Self {
                active: Arc::new(AtomicUsize::new(0)),
                maximum: Arc::clone(&maximum),
            },
            maximum,
        )
    }
}

impl EventLoopDriver for ConcurrentProbe {
    fn start_front(&mut self, starts: &[RuntimeStart]) -> Result<(), DriverError> {
        if starts.is_empty() {
            return Ok(());
        }
        let barrier = Arc::new(Barrier::new(starts.len()));
        thread::scope(|scope| {
            for _start in starts {
                let active = Arc::clone(&self.active);
                let maximum = Arc::clone(&self.maximum);
                let barrier = Arc::clone(&barrier);
                scope.spawn(move || observe_overlap(&active, &maximum, &barrier));
            }
        });
        Ok(())
    }

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
        Ok(())
    }
}

/// Records one startup task while every entry in its frontier is active.
fn observe_overlap(active: &AtomicUsize, maximum: &AtomicUsize, barrier: &Barrier) {
    let concurrency = active.fetch_add(1, Ordering::SeqCst) + 1;
    maximum.fetch_max(concurrency, Ordering::SeqCst);
    barrier.wait();
    active.fetch_sub(1, Ordering::SeqCst);
}
