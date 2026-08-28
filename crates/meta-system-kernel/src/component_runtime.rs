//! Execution state attached only to Active Component Instances.

use crate::{ComponentInstanceId, ComponentRuntimeId, Mailbox, MailboxPolicy};

/// The living execution attached to one Active Component Instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentRuntime {
    /// Identity unique to this concrete execution lifecycle.
    id: ComponentRuntimeId,
    /// Component Instance whose execution this object represents.
    instance_id: ComponentInstanceId,
    /// Bounded Delivery queue owned by this concrete Runtime lifecycle.
    mailbox: Mailbox,
}

impl ComponentRuntime {
    /// Creates execution state with its Component-declared Mailbox bound.
    #[must_use]
    pub(crate) const fn with_mailbox(
        id: ComponentRuntimeId,
        instance_id: ComponentInstanceId,
        policy: MailboxPolicy,
    ) -> Self {
        Self {
            id,
            instance_id,
            mailbox: Mailbox::new(policy),
        }
    }

    /// Returns the identity of this concrete execution lifecycle.
    #[must_use]
    pub const fn id(&self) -> ComponentRuntimeId {
        self.id
    }

    /// Returns the Component Instance executed by this Runtime.
    #[must_use]
    pub const fn instance_id(&self) -> ComponentInstanceId {
        self.instance_id
    }

    /// Returns this Runtime's bounded aggregate Delivery queue.
    #[must_use]
    pub const fn mailbox(&self) -> &Mailbox {
        &self.mailbox
    }

    /// Returns mutable Mailbox state to the private routing engine.
    pub(crate) const fn mailbox_mut(&mut self) -> &mut Mailbox {
        &mut self.mailbox
    }
}
