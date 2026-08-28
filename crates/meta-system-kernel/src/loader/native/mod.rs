//! Trusted native Component ABI and its dynamic-library materializer adapter.
//!
//! [`NativeMaterializer`](crate::loader::native::NativeMaterializer) opens a trusted dynamic
//! library during
//! [`LoadPhase::Materialized`](crate::loader::LoadPhase::Materialized), resolves the versioned
//! [`NATIVE_COMPONENT_ENTRY_POINT`](crate::loader::native::NATIVE_COMPONENT_ENTRY_POINT) during
//! [`LoadPhase::Inspected`](crate::loader::LoadPhase::Inspected), validates its C-compatible
//! [`NativeComponentDescriptor`](crate::loader::native::NativeComponentDescriptor), and copies the
//! result into the unique complete Rust
//! [`ComponentDefinition`](crate::system::ComponentDefinition).
//!
//! ```text
//! native library ──dlopen──▶ retained handle
//!       │
//!       └── ABI v1 descriptor ──validate and copy──▶ Component Definition
//! ```
//!
//! # Trust model
//!
//! Native code executes in-process and is trusted. Declared Requirements and Capabilities explain
//! composition; they are not permissions or a sandbox. Filesystem access used for bootstrap stays
//! private to the adapter and does not create a filesystem Capability.
//!
//! # Interface and boundary
//!
//! - [`NativeMaterializer`](crate::loader::native::NativeMaterializer) is the concrete adapter used
//!   through the
//!   [`ComponentMaterializer`](crate::loader::ComponentMaterializer) contract.
//! - [`NATIVE_COMPONENT_ABI_VERSION`](crate::loader::native::NATIVE_COMPONENT_ABI_VERSION) and
//!   [`NATIVE_COMPONENT_ENTRY_POINT`](crate::loader::native::NATIVE_COMPONENT_ENTRY_POINT)
//!   identify the supported entry point exactly.
//! - [`NativeComponentDescriptor`](crate::loader::native::NativeComponentDescriptor),
//!   [`NativeRequirementDescriptor`](crate::loader::native::NativeRequirementDescriptor), and
//!   [`NativeCapabilityDescriptor`](crate::loader::native::NativeCapabilityDescriptor) form the
//!   complete borrowed C-compatible transfer interface.
//!
//! Dynamic-library handles, symbol resolution, pointer validation, and conversion into domain
//! values stay hidden in the private adapter implementation. The descriptor is copied while its
//! library handle is retained; its ABI version and reserved field must be valid, and every
//! non-empty descriptor slice must be non-null, aligned, initialized, and readable for its
//! announced length.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::loader::native::NATIVE_COMPONENT_ABI_VERSION;
//!
//! assert_eq!(NATIVE_COMPONENT_ABI_VERSION, 1);
//! ```

mod abi;
#[allow(
    unsafe_code,
    reason = "trusted native bootstrap is isolated and documented by the ABI v1 contract"
)]
mod materializer;

pub use abi::{
    NATIVE_COMPONENT_ABI_VERSION, NATIVE_COMPONENT_ENTRY_POINT, NativeCapabilityDescriptor,
    NativeComponentDescriptor, NativeRequirementDescriptor,
};
pub use materializer::NativeMaterializer;
