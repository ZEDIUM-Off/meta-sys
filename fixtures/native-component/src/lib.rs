//! Independent dynamic-library fixture implementing Native Component ABI v1.

#![allow(
    unsafe_code,
    reason = "exporting the documented C ABI requires an unmangled symbol"
)]

/// C-compatible Requirement contribution copied by the host.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RequirementDescriptor {
    /// Stable Requirement identity.
    pub id: u64,
    /// Requested Capability Contract identity.
    pub contract: u64,
}

/// C-compatible Capability contribution copied by the host.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CapabilityDescriptor {
    /// Stable Capability identity.
    pub id: u64,
    /// Implemented Capability Contract identity.
    pub contract: u64,
}

/// Complete C-compatible Native Component ABI v1 descriptor.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ComponentDescriptor {
    /// ABI version implemented by this fixture.
    pub abi_version: u32,
    /// Reserved v1 field fixed to zero.
    pub reserved: u32,
    /// Stable Component Definition identity.
    pub definition_id: u64,
    /// Immutable static Requirement contribution array.
    pub requirements: *const RequirementDescriptor,
    /// Number of Requirement entries.
    pub requirements_len: usize,
    /// Immutable static Capability contribution array.
    pub capabilities: *const CapabilityDescriptor,
    /// Number of Capability entries.
    pub capabilities_len: usize,
}

/// Fixture Requirements retained for the full library lifetime.
static REQUIREMENTS: [RequirementDescriptor; 1] = [RequirementDescriptor {
    id: 700,
    contract: 70,
}];

/// Fixture Capabilities retained for the full library lifetime.
static CAPABILITIES: [CapabilityDescriptor; 1] = [CapabilityDescriptor {
    id: 800,
    contract: 80,
}];

/// Exports one complete Native Component ABI v1 descriptor.
#[unsafe(no_mangle)]
pub extern "C" fn meta_system_component_v1() -> ComponentDescriptor {
    ComponentDescriptor {
        abi_version: 1,
        reserved: 0,
        definition_id: 42,
        requirements: REQUIREMENTS.as_ptr(),
        requirements_len: REQUIREMENTS.len(),
        capabilities: CAPABILITIES.as_ptr(),
        capabilities_len: CAPABILITIES.len(),
    }
}

/// Forces Cargo to link this fixture as a test dependency as well as a cdylib.
#[must_use]
pub const fn fixture_marker() -> u64 {
    42
}
