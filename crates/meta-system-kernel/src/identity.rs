//! Strongly typed identities used by the System Graph.

/// Identifies one declarative Component Definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentDefinitionId(u64);

impl ComponentDefinitionId {
    /// Creates an opaque Component Definition identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one living Component Instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentInstanceId(u64);

impl ComponentInstanceId {
    /// Creates an opaque Component Instance identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one Requirement declared by a Component Definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequirementId(u64);

impl RequirementId {
    /// Creates an opaque Requirement identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies the contract shared by compatible Requirements and Capabilities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityContractId(u64);

impl CapabilityContractId {
    /// Creates an opaque Capability Contract identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one Capability published in the System Graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(u64);

impl CapabilityId {
    /// Creates an opaque Capability identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one concrete Component Runtime lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComponentRuntimeId(u64);

impl ComponentRuntimeId {
    /// Creates an opaque Component Runtime identity from a Runtime-owned value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Identifies one living Effect owned by a Component Instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectId(u64);

impl EffectId {
    /// Creates an opaque Effect identity from a host-provided value.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }
}
