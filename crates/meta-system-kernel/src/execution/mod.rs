//! Dependency-aware execution plans and the replaceable Driver seam.
//!
//! Resolution produces an inspectable [`ExecutionPlan`] made of ordered [`ExecutionFront`]
//! values. Work inside one front is independent and may overlap; fronts themselves retain the
//! local dependency order. An [`EventLoopDriver`] decides how Component Runtimes start, advance,
//! process observable Deliveries, and stop.
//!
//! ```text
//! affected graph mutation
//!          │
//!          ▼
//!   Execution Plan
//!   ┌──────────────┐
//!   │ Front 0: A B │  A and B may overlap
//!   ├──────────────┤
//!   │ Front 1: C   │  C waits for its dependencies
//!   └──────┬───────┘
//!          ▼
//!   Event Loop Driver
//! ```
//!
//! # Interface
//!
//! [`EventLoopDriver`] is the real strategy seam. [`SequentialExecutor`] is the deterministic
//! reference adapter; tests may supply a concurrent adapter without changing Kernel outcomes.
//! Plans are observations of dependency structure, not a mutable scheduler interface.
//!
//! # Invariants
//!
//! Dependency order is local and deterministic. The interface permits independent work to run
//! concurrently and imposes no global mutex, global serial queue, or thread affinity. Driver
//! failures remain lifecycle errors rather than stable Component resolution states.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::execution::{EventLoopDriver, SequentialExecutor};
//!
//! fn accepts_driver(_driver: &impl EventLoopDriver) {}
//!
//! let driver = SequentialExecutor::default();
//! accepts_driver(&driver);
//! ```

mod contract;
mod plan;
mod sequential;

pub use contract::{DriverError, DriverProgress, EventLoopDriver};
pub use plan::{ExecutionFront, ExecutionPlan, ExecutionWork, RuntimeStart};
pub use sequential::SequentialExecutor;
