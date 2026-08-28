//! Public dependency plan derived from one affected graph mutation.

use crate::{ComponentInstanceId, ComponentRuntimeId};

/// One Component activation with explicit work dependencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionWork {
    /// Component Instance activated by this work item.
    instance: ComponentInstanceId,
    /// Activations that must complete before this work can execute.
    dependencies: Vec<ComponentInstanceId>,
}

impl ExecutionWork {
    /// Creates one inspectable activation work item.
    #[must_use]
    pub(crate) const fn new(
        instance: ComponentInstanceId,
        dependencies: Vec<ComponentInstanceId>,
    ) -> Self {
        Self {
            instance,
            dependencies,
        }
    }

    /// Returns the Component Instance activated by this work.
    #[must_use]
    pub const fn instance(&self) -> ComponentInstanceId {
        self.instance
    }

    /// Returns activations that must complete before this work executes.
    #[must_use]
    pub fn dependencies(&self) -> &[ComponentInstanceId] {
        &self.dependencies
    }
}

/// Deterministic set of independent work that may execute concurrently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionFront {
    /// Independent activation work ordered by Component Instance identity.
    work: Vec<ExecutionWork>,
}

impl ExecutionFront {
    /// Creates one deterministic executable frontier.
    #[must_use]
    pub(crate) const fn new(work: Vec<ExecutionWork>) -> Self {
        Self { work }
    }

    /// Returns independent work in deterministic reference order.
    #[must_use]
    pub fn work(&self) -> &[ExecutionWork] {
        &self.work
    }
}

/// Ordered fronts containing only activation work affected by one Event.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExecutionPlan {
    /// Fronts ordered by their explicit dependency constraints.
    fronts: Vec<ExecutionFront>,
}

impl ExecutionPlan {
    /// Creates an ordered dependency plan from deterministic fronts.
    #[must_use]
    pub(crate) const fn new(fronts: Vec<ExecutionFront>) -> Self {
        Self { fronts }
    }

    /// Returns ordered executable fronts.
    #[must_use]
    pub fn fronts(&self) -> &[ExecutionFront] {
        &self.fronts
    }
}

/// Concrete Runtime identity assigned immediately before Driver startup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RuntimeStart {
    /// Component Instance whose execution resources are created.
    instance: ComponentInstanceId,
    /// Identity of the concrete Component Runtime lifecycle.
    runtime: ComponentRuntimeId,
}

impl RuntimeStart {
    /// Creates one concrete startup request for an execution front.
    #[must_use]
    pub(crate) const fn new(instance: ComponentInstanceId, runtime: ComponentRuntimeId) -> Self {
        Self { instance, runtime }
    }

    /// Returns the Component Instance being started.
    #[must_use]
    pub const fn instance(&self) -> ComponentInstanceId {
        self.instance
    }

    /// Returns the concrete Component Runtime identity being started.
    #[must_use]
    pub const fn runtime(&self) -> ComponentRuntimeId {
        self.runtime
    }
}
