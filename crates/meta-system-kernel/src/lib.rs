//! # Meta-System Kernel
//!
//! The Kernel is the domain-neutral language and orchestrator of one living graph of dynamic
//! state machines. It accepts typed Events, resolves explicit dependencies, schedules independent
//! work, routes Events locally, and exposes every accepted transition for inspection. It owns no
//! filesystem, network, clock, storage, or other outward-facing Capability.
//!
//! ## Architecture
//!
//! ```text
//! Component Source
//!       │
//!       ▼
//!    Loader ───── complete Component Definition ─────┐
//!       ▲                                            ▼
//! Loader Addons                               Kernel Runtime
//!                                                    │ owns
//!                                                    ▼
//!                                               System Graph
//!                                      ┌─────────────┼─────────────┐
//!                                      ▼             ▼             ▼
//!                                 Resolution     Execution      Routing
//!                                      │             │             │
//!                               Binding hooks  EventLoopDriver  Room → Mailbox
//! ```
//!
//! The crate is organized by domain rather than by Rust item kind:
//!
//! - [`runtime`] — the [`KernelRuntime`] state machine, its Events, outcomes, and errors;
//! - [`system`] — the single System Graph, Components, Contexts, Facets, and Effects;
//! - [`resolution`] — Requirements, Capabilities, Bindings, and ordered policy hooks;
//! - [`execution`] — dependency fronts, execution plans, and the Driver seam;
//! - [`routing`] — Events, Rooms, Subscriptions, Mailboxes, Deliveries, and Receipts;
//! - [`loader`] — ordered loading, admission hooks, materializers, and trusted native support.
//!
//! ## Reading paths
//!
//! To understand Component activation, follow [`loader`] → [`runtime`] → [`system`] →
//! [`resolution`] → [`execution`]. To understand Event delivery, follow [`runtime`] → [`routing`]
//! → [`execution::EventLoopDriver`]. Each module page documents its interface, hidden
//! implementation, invariants, and relationship to the rest of the graph.
//!
//! ## Stable entry seams
//!
//! [`KernelRuntime`] is the main state-transition seam and exclusive owner of one System Graph.
//! [`Loader`] is the ordered bootstrap seam that supplies complete Component Definitions. Other
//! public items live under their domain modules so generated documentation preserves the
//! architecture instead of flattening it into a list of types.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::KernelRuntime;
//!
//! let runtime = KernelRuntime::new();
//! let _graph = runtime.graph();
//! ```

mod compat;

/// Dependency-aware execution plans and the replaceable Driver seam.
pub mod execution;
/// Ordered Loader lifecycle, Addon admission policy, and materializer adapters.
pub mod loader;
/// Dependency resolution and ordered Binding policy.
pub mod resolution;
/// Typed Event routing through bounded Rooms and Component Mailboxes.
pub mod routing;
/// Kernel Runtime state machine and its observable transition contract.
pub mod runtime;
/// The single living System Graph and the entities represented within it.
pub mod system;

#[doc(hidden)]
pub use compat::*;
pub use loader::Loader;
pub use runtime::KernelRuntime;
