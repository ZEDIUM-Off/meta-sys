//! Ordered Loader lifecycle, Addon admission policy, and materializer adapters.
//!
//! The [`Loader`] is a typed-Event state machine that turns an opaque [`ComponentSource`] into one
//! complete [`ComponentDefinition`](crate::system::ComponentDefinition), admits or rejects it
//! through ordered [`LoaderHook`] values, and registers an admitted Definition with a
//! [`KernelRuntime`](crate::runtime::KernelRuntime).
//!
//! ```text
//! Declared → Located → Materialized → Inspected → Admitted ─→ Registered → Ready
//!                                          └────→ Rejected
//! ```
//!
//! # Interface
//!
//! - [`Loader`] owns independent, inspectable [`LoadRecord`] lifecycles.
//! - [`LoaderEvent`] is the only public transition stimulus.
//! - [`ComponentMaterializer`] is the bootstrap adapter seam.
//! - [`LoaderHook`] is the deterministic Addon admission seam.
//! - [`native`](crate::loader::native) contains the trusted native ABI and its concrete adapter.
//!
//! # Invariants
//!
//! Events cannot skip or reorder phases. Inspection yields the same complete Definition later
//! registered in the System Graph. Hooks govern only transitions that occur after their
//! activation; without hooks, admission is `allow-all`. Materialization mechanisms remain Loader
//! implementation and do not grant Components implicit Capabilities.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::loader::ComponentSource;
//!
//! let source = ComponentSource::new("component://editor");
//! assert_eq!(source.as_str(), "component://editor");
//! ```

use std::collections::BTreeMap;

mod deterministic;
mod error;
mod event;
mod machine;
mod materializer;
/// Trusted native Component ABI and dynamic-library adapter.
pub mod native;
mod outcome;
mod policy;
mod rejection;
mod state;

pub use deterministic::DeterministicMaterializer;
pub use error::LoaderError;
pub use event::{ComponentSource, LoadRequest, LoaderEvent};
pub use materializer::{ComponentMaterializer, MaterializerError};
pub use outcome::{LoadTransition, LoaderOutcome};
pub use policy::{LoaderDecision, LoaderHook, LoaderProposal};
pub use rejection::LoadRejection;
pub use state::{LoadId, LoadPhase, LoadRecord};

/// Ordered Loader machine backed by one materializer adapter.
#[derive(Debug)]
pub struct Loader<Materializer> {
    /// Bootstrap adapter kept outside Component Capabilities.
    materializer: Materializer,
    /// Independent inspectable Loader lifecycles.
    loads: BTreeMap<LoadId, LoadRecord>,
    /// Active Loader Addon hooks in their declared deterministic order.
    hooks: Vec<Box<dyn LoaderHook>>,
}
