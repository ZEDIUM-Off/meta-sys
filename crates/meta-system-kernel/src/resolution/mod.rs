//! Dependency resolution and ordered Binding policy.
//!
//! A [`Requirement`] names a needed Capability Contract and a [`Capability`] offers that same
//! contract. The private Resolver examines only the affected pending Instances, proposes explicit
//! [`Binding`] relations, and evaluates active [`BindingHook`] values in deterministic order.
//!
//! ```text
//! Requirement ──matches contract──▶ Capability candidates
//!                                      │
//!                                      ▼
//!                             ordered Binding hooks
//!                                      │
//!                         allow / reject / select candidate
//!                                      │
//!                                      ▼
//!                                   Binding
//! ```
//!
//! # Interface
//!
//! The public interface contains inspectable declarations, proposals, decisions, and the Addon
//! hook seam. A Resolver object is intentionally absent: affected-subgraph planning and removal
//! plans remain private implementation behind [`KernelRuntime`](crate::runtime::KernelRuntime).
//!
//! # Invariants
//!
//! A Requirement is a composition need, never a permission. A hook may select only a compatible
//! candidate present in its [`BindingProposal`]. Without an active hook, selection is `allow-all`.
//! Bindings remain explicit in the single [`SystemGraph`](crate::system::SystemGraph).
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::{
//!     resolution::{Capability, Requirement},
//!     system::{CapabilityContractId, CapabilityId, RequirementId},
//! };
//!
//! let contract = CapabilityContractId::new(7);
//! let requirement = Requirement::necessary(RequirementId::new(1), contract);
//! let capability = Capability::new(CapabilityId::new(2), contract);
//!
//! assert_eq!(requirement.contract(), capability.contract());
//! ```

mod binding;
mod capability;
mod hook;
mod planner;
mod policy;
mod removal;
mod requirement;

pub use binding::Binding;
pub use capability::Capability;
pub use hook::{BindingDecision, BindingHook, HookOrder};
pub use policy::{BindingCandidate, BindingProposal};
pub use requirement::Requirement;

pub(crate) use planner::ActivationPlan;
