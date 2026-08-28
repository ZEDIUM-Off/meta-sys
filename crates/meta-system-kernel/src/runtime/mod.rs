//! Kernel Runtime state machine and its observable transition contract.
//!
//! A [`KernelRuntime`] owns exactly one
//! [`SystemGraph`](crate::system::SystemGraph). It accepts a typed [`KernelEvent`], delegates
//! resolution, execution, and routing work to their respective domains, then returns either a
//! [`TransitionOutcome`] or a [`KernelError`].
//!
//! ```text
//! Current System Graph + Kernel Event
//!                 │
//!                 ▼
//!            Kernel Runtime
//!       ┌─────────┼──────────┐
//!       ▼         ▼          ▼
//!  resolution  execution   routing
//!       └─────────┼──────────┘
//!                 ▼
//!       Next System Graph + Outcome
//! ```
//!
//! # Interface
//!
//! - [`KernelRuntime`] is the event-processing seam and exclusive graph owner.
//! - [`KernelEvent`] is the closed set of Kernel state transitions.
//! - [`TransitionOutcome`] explains accepted graph and lifecycle changes.
//! - [`KernelError`] reports a rejected transition without introducing a new resolution state.
//!
//! # Invariants
//!
//! A Runtime never shares mutable graph state with another Runtime. Stable Component resolution
//! remains either [`ResolutionState::Pending`](crate::system::ResolutionState::Pending) or
//! [`ResolutionState::Active`](crate::system::ResolutionState::Active); lifecycle failures are
//! errors, not graph states. The
//! private implementation hides graph storage, affected-subgraph planning, cleanup, and routing
//! mutation behind the confirmed [`KernelRuntime`] seam.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::runtime::KernelRuntime;
//!
//! let runtime = KernelRuntime::new();
//! let _graph = runtime.graph();
//! ```

mod error;
mod event;
mod hooks;
mod lifecycle_transition;
mod machine;
mod outcome;
mod routing;

use crate::{execution::SequentialExecutor, resolution::BindingHook, system::GraphState};

pub use error::KernelError;
pub use event::KernelEvent;
pub use lifecycle_transition::LifecycleTransition;
pub use outcome::TransitionOutcome;

/// The isolated evaluator and owner of exactly one System Graph.
#[derive(Debug)]
pub struct KernelRuntime<Driver = SequentialExecutor> {
    /// Mutable graph state owned exclusively by this Runtime.
    graph: GraphState,
    /// Interchangeable execution strategy selected for this Runtime.
    driver: Driver,
    /// Next identity reserved for a successfully started Component Runtime.
    next_runtime_id: u64,
    /// Active Addon hooks sorted by their declared deterministic order.
    binding_hooks: Vec<Box<dyn BindingHook>>,
}
