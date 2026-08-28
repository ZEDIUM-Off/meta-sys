//! Implementation of the Kernel Runtime transition machine.

use super::{KernelError, KernelEvent, KernelRuntime, TransitionOutcome};
use crate::{
    execution::{EventLoopDriver, RuntimeStart, SequentialExecutor},
    system::{
        ComponentDefinition, ComponentInstanceId, ComponentRuntimeId, Context, Effect, Facet,
        FacetSchema, GraphState, ResolutionState, SystemGraph,
    },
};

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
            binding_hooks: Vec::new(),
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
            binding_hooks: Vec::new(),
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
            KernelEvent::RegisterContext { context } => self.handle_context(context),
            KernelEvent::RegisterFacetSchema { schema } => self.handle_facet_schema(schema),
            KernelEvent::AttachFacet { facet } => self.handle_facet(facet),
        }
    }

    /// Registers one complete declaration and resolves every ready Instance.
    fn handle_registration(
        &mut self,
        definition: ComponentDefinition,
        instance_id: ComponentInstanceId,
    ) -> Result<TransitionOutcome, KernelError> {
        self.graph.register_pending(definition, instance_id)?;
        let mut outcome = TransitionOutcome::registered_pending(instance_id);
        let execution_plan = self
            .graph
            .affected_activation_plan(instance_id, &self.binding_hooks)?;
        let inspectable_plan = execution_plan.inspectable();
        for front in &execution_plan.fronts {
            self.execute_activation_front(front, &mut outcome)?;
        }
        outcome.set_execution_plan(inspectable_plan);
        Ok(outcome)
    }

    /// Starts and commits one dependency-free activation frontier.
    fn execute_activation_front(
        &mut self,
        front: &[crate::resolution::ActivationPlan],
        outcome: &mut TransitionOutcome,
    ) -> Result<(), KernelError> {
        let Some(first) = front.first() else {
            return Ok(());
        };
        let mut next_id = self.next_runtime_id;
        let mut starts = Vec::with_capacity(front.len());
        for plan in front {
            starts.push(RuntimeStart::new(
                plan.instance_id,
                ComponentRuntimeId::new(next_id),
            ));
            next_id = next_id
                .checked_add(1)
                .ok_or(KernelError::RuntimeIdentityExhausted)?;
        }
        self.driver
            .start_front(&starts)
            .map_err(|error| KernelError::DriverStart {
                instance_id: first.instance_id,
                error,
            })?;
        for (plan, start) in front.iter().zip(starts) {
            self.graph.apply_activation(plan, start.runtime());
            outcome.record_activation(plan.instance_id, &plan.bindings);
        }
        self.next_runtime_id = next_id;
        Ok(())
    }

    /// Records one Effect after validating its living lifecycle owner.
    fn handle_effect(&mut self, effect: Effect) -> Result<TransitionOutcome, KernelError> {
        self.graph.record_effect(effect)?;
        Ok(TransitionOutcome::empty())
    }

    /// Stops affected Runtimes and applies complete lifecycle-owned cleanup.
    fn handle_unregistration(
        &mut self,
        instance_id: ComponentInstanceId,
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
        if plan.previous == ResolutionState::Active {
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

    /// Registers one structural Context after validating explicit scope links.
    fn handle_context(&mut self, context: Context) -> Result<TransitionOutcome, KernelError> {
        self.graph.register_context(context)?;
        Ok(TransitionOutcome::empty())
    }

    /// Registers one Addon-owned Facet Schema.
    fn handle_facet_schema(
        &mut self,
        schema: FacetSchema,
    ) -> Result<TransitionOutcome, KernelError> {
        self.graph.register_facet_schema(schema)?;
        Ok(TransitionOutcome::empty())
    }

    /// Validates and attaches one typed Facet to the System Graph.
    fn handle_facet(&mut self, facet: Facet) -> Result<TransitionOutcome, KernelError> {
        self.graph.attach_facet(facet)?;
        Ok(TransitionOutcome::empty())
    }

    /// Returns a read-only view of this Runtime's current System Graph.
    pub const fn graph(&self) -> SystemGraph<'_> {
        SystemGraph::new(&self.graph)
    }
}
