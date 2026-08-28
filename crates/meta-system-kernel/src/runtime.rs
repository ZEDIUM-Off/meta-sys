//! Public event-processing seam of one isolated Kernel Runtime.

use crate::{KernelError, KernelEvent, SystemGraph, graph::GraphState};

/// The isolated evaluator and owner of exactly one System Graph.
#[derive(Debug, Default)]
pub struct KernelRuntime {
    /// Mutable graph state owned exclusively by this Runtime.
    graph: GraphState,
}

impl KernelRuntime {
    /// Creates an empty Kernel Runtime with no shared graph state.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Evolves this Runtime by interpreting one typed Event.
    ///
    /// A registration with an unsatisfied necessary Requirement must leave its
    /// Component Instance `Pending`, without a Binding or Component Runtime.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when the Event would duplicate a graph identity.
    pub fn handle(&mut self, event: KernelEvent) -> Result<(), KernelError> {
        let KernelEvent::RegisterComponent {
            definition,
            instance_id,
        } = event;
        self.graph.register_pending(definition, instance_id)
    }

    /// Returns a read-only view of this Runtime's current System Graph.
    pub const fn graph(&self) -> SystemGraph<'_> {
        SystemGraph::new(&self.graph)
    }
}
