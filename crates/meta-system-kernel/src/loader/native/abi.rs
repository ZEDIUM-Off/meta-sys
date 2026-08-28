//! Versioned C-compatible transfer descriptors for trusted native Components.

/// Native Component ABI version implemented by this host.
pub const NATIVE_COMPONENT_ABI_VERSION: u32 = 1;

/// Null-terminated native entry-point symbol for ABI v1.
pub const NATIVE_COMPONENT_ENTRY_POINT: &[u8] = b"meta_system_component_v1\0";

/// C-compatible Requirement contribution returned by a native Component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NativeRequirementDescriptor {
    /// Stable Requirement identity.
    pub id: u64,
    /// Capability Contract requested by the Requirement.
    pub contract: u64,
}

/// C-compatible Capability contribution returned by a native Component.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct NativeCapabilityDescriptor {
    /// Stable Capability identity.
    pub id: u64,
    /// Capability Contract implemented by the Capability.
    pub contract: u64,
}

/// Complete C-compatible transfer descriptor returned by ABI v1.
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct NativeComponentDescriptor {
    /// ABI version, required to equal [`NATIVE_COMPONENT_ABI_VERSION`].
    pub abi_version: u32,
    /// Reserved field required to be zero in ABI v1.
    pub reserved: u32,
    /// Stable Component Definition identity.
    pub definition_id: u64,
    /// Immutable Requirement array owned by the loaded library.
    pub requirements: *const NativeRequirementDescriptor,
    /// Number of readable Requirement entries.
    pub requirements_len: usize,
    /// Immutable Capability array owned by the loaded library.
    pub capabilities: *const NativeCapabilityDescriptor,
    /// Number of readable Capability entries.
    pub capabilities_len: usize,
}
