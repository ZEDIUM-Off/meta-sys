//! Public event-processing seam of one isolated Kernel Runtime.

use crate::{
    ComponentRuntimeId, EventLoopDriver, KernelError, KernelEvent, SequentialExecutor, SystemGraph,
    TransitionOutcome, graph::GraphState,
};

/// The isolated evaluator and owner of exactly one System Graph.
#[derive(Debug)]
pub struct KernelRuntime<Driver = SequentialExecutor> {
    /// Mutable graph state owned exclusively by this Runtime.
    graph: GraphState,
    /// Interchangeable execution strategy selected for this Runtime.
    driver: Driver,
    /// Next identity reserved for a successfully started Component Runtime.
    next_runtime_id: u64,
}

impl KernelRuntime<SequentialExecutor> {
    /// Creates an empty Kernel Runtime with no shared graph state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for KernelRuntime<SequentialExecutor> {
    fn default() -> Self {
        Self {
            graph: GraphState::default(),
            driver: SequentialExecutor::default(),
            next_runtime_id: 1,
        }
    }
}

impl<Driver: EventLoopDriver> KernelRuntime<Driver> {
    /// Creates an empty Kernel Runtime using an interchangeable execution strategy.
    #[must_use]
    pub fn with_event_loop_driver(driver: Driver) -> Self {
        Self {
            graph: GraphState::default(),
            driver,
            next_runtime_id: 1,
        }
    }

    /// Evolves this Runtime by interpreting one typed Event.
    ///
    /// A registration with an unsatisfied necessary Requirement must leave its
    /// Component Instance `Pending`, without a Binding or Component Runtime.
    /// An Instance becomes `Active` only after compatible Bindings have been
    /// selected and the configured [`EventLoopDriver`] has started its Runtime.
    /// The returned [`TransitionOutcome`] explains every completed transition
    /// and provider selection caused by the Event.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when an Event violates a graph or lifecycle
    /// invariant, references an unknown Instance, or when the configured Driver
    /// rejects Component Runtime startup or shutdown. Rejected lifecycle work
    /// never introduces a third resolution state beyond `Pending` and `Active`.
    pub fn handle(&mut self, event: KernelEvent) -> Result<TransitionOutcome, KernelError> {
        match event {
            KernelEvent::RegisterComponent {
                definition,
                instance_id,
            } => self.handle_registration(definition, instance_id),
            KernelEvent::RecordEffect { effect } => self.handle_effect(effect),
            KernelEvent::UnregisterComponent { instance_id } => {
                self.handle_unregistration(instance_id)
            }
        }
    }

    /// Registers one complete declaration and resolves every ready Instance.
    fn handle_registration(
        &mut self,
        definition: crate::ComponentDefinition,
        instance_id: crate::ComponentInstanceId,
    ) -> Result<TransitionOutcome, KernelError> {
        self.graph.register_pending(definition, instance_id)?;
        let mut outcome = TransitionOutcome::registered_pending(instance_id);
        while let Some(plan) = self.graph.next_activation_plan() {
            let runtime_id = ComponentRuntimeId::new(self.next_runtime_id);
            self.driver
                .start(plan.instance_id, runtime_id)
                .map_err(|error| KernelError::DriverStart {
                    instance_id: plan.instance_id,
                    error,
                })?;
            self.next_runtime_id += 1;
            self.graph.apply_activation(&plan, runtime_id);
            outcome.record_activation(plan.instance_id, &plan.bindings);
        }
        Ok(outcome)
    }

    /// Records one Effect after validating its living lifecycle owner.
    fn handle_effect(&mut self, effect: crate::Effect) -> Result<TransitionOutcome, KernelError> {
        self.graph.record_effect(effect)?;
        Ok(TransitionOutcome::empty())
    }

    /// Stops affected Runtimes and applies complete lifecycle-owned cleanup.
    fn handle_unregistration(
        &mut self,
        instance_id: crate::ComponentInstanceId,
    ) -> Result<TransitionOutcome, KernelError> {
        let plan = self.graph.removal_plan(instance_id)?;
        for consumer in &plan.consumers {
            self.driver
                .stop(consumer.instance_id)
                .map_err(|error| KernelError::DriverStop {
                    instance_id: consumer.instance_id,
                    error,
                })?;
        }
        if plan.previous == crate::ResolutionState::Active {
            self.driver
                .stop(plan.instance_id)
                .map_err(|error| KernelError::DriverStop {
                    instance_id: plan.instance_id,
                    error,
                })?;
        }
        let mut outcome = TransitionOutcome::empty();
        for consumer in &plan.consumers {
            outcome.record_deactivation(
                consumer.instance_id,
                &consumer.bindings,
                &consumer.effects,
            );
        }
        outcome.record_removal(
            plan.instance_id,
            plan.previous,
            &plan.own_bindings,
            &plan.effects,
        );
        self.graph.apply_removal(&plan);
        Ok(outcome)
    }

    /// Returns a read-only view of this Runtime's current System Graph.
    pub const fn graph(&self) -> SystemGraph<'_> {
        SystemGraph::new(&self.graph)
    }
}
