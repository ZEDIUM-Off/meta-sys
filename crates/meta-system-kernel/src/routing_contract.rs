//! Complete declarative Event routing contribution of one Component.

use crate::{
    BroadcastSubscription, EmissionDeclaration, MailboxPolicy, RoomDeclaration,
    SubscriptionDeclaration,
};

/// Cohesive Room, Mailbox, emit, and broadcast contract of a Component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingContract {
    /// Explicit bound of each concrete Runtime Mailbox.
    mailbox_policy: MailboxPolicy,
    /// Logical Rooms materialized only while an Instance is Active.
    rooms: Vec<RoomDeclaration>,
    /// Room relations materialized into the active Runtime Mailbox.
    subscriptions: Vec<SubscriptionDeclaration>,
    /// Typed emit routes owned by this Component contract.
    emissions: Vec<EmissionDeclaration>,
    /// Broadcast Event contracts observed by an Active Instance.
    broadcast_subscriptions: Vec<BroadcastSubscription>,
}

impl RoutingContract {
    /// Creates an empty routing contract with the prototype Mailbox bound.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mailbox_policy: MailboxPolicy::new(
                crate::QueueCapacity::DEFAULT,
                crate::MailboxOverflowStrategy::RejectNew,
            ),
            rooms: Vec::new(),
            subscriptions: Vec::new(),
            emissions: Vec::new(),
            broadcast_subscriptions: Vec::new(),
        }
    }

    /// Sets the complete Component-owned Runtime Mailbox policy.
    #[must_use = "builder methods return the updated Routing Contract"]
    pub const fn with_mailbox_policy(mut self, policy: MailboxPolicy) -> Self {
        self.mailbox_policy = policy;
        self
    }

    /// Adds one logical Room contribution.
    #[must_use = "builder methods return the updated Routing Contract"]
    pub fn with_room(mut self, room: RoomDeclaration) -> Self {
        self.rooms.push(room);
        self
    }

    /// Adds one declared Room relation to the Component Mailbox.
    #[must_use = "builder methods return the updated Routing Contract"]
    pub fn with_subscription(mut self, subscription: SubscriptionDeclaration) -> Self {
        self.subscriptions.push(subscription);
        self
    }

    /// Adds one typed emit route owned by this Component.
    #[must_use = "builder methods return the updated Routing Contract"]
    pub fn with_emission(mut self, emission: EmissionDeclaration) -> Self {
        self.emissions.push(emission);
        self
    }

    /// Adds one broadcast Event contract observed by this Component.
    #[must_use = "builder methods return the updated Routing Contract"]
    pub fn with_broadcast_subscription(mut self, subscription: BroadcastSubscription) -> Self {
        self.broadcast_subscriptions.push(subscription);
        self
    }

    /// Returns the complete Component-owned Runtime Mailbox policy.
    #[must_use]
    pub const fn mailbox_policy(&self) -> MailboxPolicy {
        self.mailbox_policy
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

    /// Returns typed emit routes in declaration order.
    #[must_use]
    pub fn emissions(&self) -> &[EmissionDeclaration] {
        &self.emissions
    }

    /// Returns broadcast listener contracts in declaration order.
    #[must_use]
    pub fn broadcast_subscriptions(&self) -> &[BroadcastSubscription] {
        &self.broadcast_subscriptions
    }
}

impl Default for RoutingContract {
    fn default() -> Self {
        Self::new()
    }
}
