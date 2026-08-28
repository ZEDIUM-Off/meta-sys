//! Static and complete declarations of Components.

use crate::{
    Capability, ComponentDefinitionId, QueueCapacity, Requirement, RoomDeclaration,
    SubscriptionDeclaration,
};

/// The complete declarative identity and contributions of a Component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentDefinition {
    /// Stable identity of the declaration.
    id: ComponentDefinitionId,
    /// Capability needs contributed to the System Graph.
    requirements: Vec<Requirement>,
    /// Capability offers contributed to the System Graph.
    capabilities: Vec<Capability>,
    /// Logical Rooms materialized only while an Instance is Active.
    rooms: Vec<RoomDeclaration>,
    /// Room relations materialized into the active Runtime Mailbox.
    subscriptions: Vec<SubscriptionDeclaration>,
    /// Explicit bound of each concrete Runtime Mailbox.
    mailbox_capacity: QueueCapacity,
}

impl ComponentDefinition {
    /// Creates a complete declaration with no contributions.
    #[must_use]
    pub const fn new(id: ComponentDefinitionId) -> Self {
        Self {
            id,
            requirements: Vec::new(),
            capabilities: Vec::new(),
            rooms: Vec::new(),
            subscriptions: Vec::new(),
            mailbox_capacity: QueueCapacity::DEFAULT,
        }
    }

    /// Adds one inspectable Capability to this complete declaration.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Adds one inspectable Requirement to this complete declaration.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_requirement(mut self, requirement: Requirement) -> Self {
        self.requirements.push(requirement);
        self
    }

    /// Adds one logical Room contribution to this complete declaration.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_room(mut self, room: RoomDeclaration) -> Self {
        self.rooms.push(room);
        self
    }

    /// Adds one declared routing relation to this Component's Mailbox.
    #[must_use = "builder methods return the updated Component Definition"]
    pub fn with_subscription(mut self, subscription: SubscriptionDeclaration) -> Self {
        self.subscriptions.push(subscription);
        self
    }

    /// Sets the explicit bound of each concrete Runtime Mailbox.
    #[must_use = "builder methods return the updated Component Definition"]
    pub const fn with_mailbox_capacity(mut self, capacity: QueueCapacity) -> Self {
        self.mailbox_capacity = capacity;
        self
    }

    /// Returns the stable Component Definition identity.
    #[must_use]
    pub const fn id(&self) -> ComponentDefinitionId {
        self.id
    }

    /// Returns every Requirement in declaration order.
    #[must_use]
    pub fn requirements(&self) -> &[Requirement] {
        &self.requirements
    }

    /// Returns every Capability in declaration order.
    #[must_use]
    pub fn capabilities(&self) -> &[Capability] {
        &self.capabilities
    }

    /// Returns logical Rooms in declaration order.
    #[must_use]
    pub fn rooms(&self) -> &[RoomDeclaration] {
        &self.rooms
    }

    /// Returns Room relations in declaration order.
    #[must_use]
    pub fn subscriptions(&self) -> &[SubscriptionDeclaration] {
        &self.subscriptions
    }

    /// Returns the explicit concrete Runtime Mailbox bound.
    #[must_use]
    pub const fn mailbox_capacity(&self) -> QueueCapacity {
        self.mailbox_capacity
    }
}
