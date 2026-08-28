//! Private Room distribution and Component Mailbox mutation engine.

use std::collections::BTreeSet;

use crate::{
    ComponentDefinition, ComponentInstanceId, ComponentRuntimeId, Delivery, DeliveryProgress,
    DeliveryReceipt, DeliveryState, Event, EventLoopDriver, KernelError, Room, RoomAddress,
    RoomRuntimeId, SendReceipt, Subscription, graph::GraphState,
};

impl GraphState {
    /// Validates logical Room and Subscription declarations before graph mutation.
    pub(super) fn validate_routing_registration(
        &self,
        definition: &ComponentDefinition,
    ) -> Result<(), KernelError> {
        let existing = self
            .definitions
            .values()
            .flat_map(ComponentDefinition::rooms)
            .map(|room| room.address())
            .collect::<BTreeSet<_>>();
        let mut declared = BTreeSet::new();
        for room in definition.rooms() {
            if existing.contains(&room.address()) || !declared.insert(room.address()) {
                return Err(KernelError::DuplicateRoomAddress(room.address()));
            }
        }
        let mut subscriptions = BTreeSet::new();
        for subscription in definition.subscriptions() {
            if !subscriptions.insert(subscription.room()) {
                return Err(KernelError::DuplicateSubscription(subscription.room()));
            }
        }
        Ok(())
    }

    /// Materializes declared Rooms and Subscriptions for one Active Runtime.
    pub(super) fn activate_routing(
        &mut self,
        instance_id: ComponentInstanceId,
        runtime_id: ComponentRuntimeId,
    ) {
        let Some(definition) = self
            .instances
            .get(&instance_id)
            .and_then(|instance| self.definitions.get(&instance.definition_id()).cloned())
        else {
            return;
        };
        for (ordinal, declaration) in definition.rooms().iter().copied().enumerate() {
            self.rooms.insert(
                declaration.address(),
                Room::new(
                    RoomRuntimeId::new(runtime_id, ordinal),
                    declaration,
                    instance_id,
                ),
            );
        }
        self.subscriptions.extend(
            definition
                .subscriptions()
                .iter()
                .map(|declaration| Subscription::new(declaration.room(), instance_id)),
        );
        self.subscriptions
            .sort_by_key(|subscription| (subscription.room(), subscription.subscriber()));
    }

    /// Releases concrete routing resources owned by one deactivating Runtime.
    pub(super) fn deactivate_routing(&mut self, instance_id: ComponentInstanceId) {
        self.rooms.retain(|_, room| room.owner() != instance_id);
        self.subscriptions
            .retain(|subscription| subscription.subscriber() != instance_id);
    }

    /// Accepts and distributes one Event through exactly one FIFO Room step.
    pub(super) fn send<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        address: RoomAddress,
        event: Event,
    ) -> Result<SendReceipt, KernelError> {
        let accepted = self
            .rooms
            .get_mut(&address)
            .ok_or(KernelError::UnavailableRoom(address))?
            .accept(event)?;
        let recipients = self
            .subscriptions
            .iter()
            .filter(|subscription| subscription.room() == address)
            .map(|subscription| subscription.subscriber())
            .collect::<Vec<_>>();
        let mut receipts = Vec::with_capacity(recipients.len());
        for recipient in recipients {
            receipts.push(self.deliver(driver, address, recipient, &accepted)?);
        }
        Ok(SendReceipt::new(address, accepted.sequence, receipts))
    }

    /// Attempts one subscribed Mailbox placement and asks the Driver once.
    fn deliver<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        address: RoomAddress,
        recipient: ComponentInstanceId,
        accepted: &crate::room::AcceptedEvent,
    ) -> Result<DeliveryReceipt, KernelError> {
        let delivery = Delivery::new(
            address,
            accepted.sequence,
            recipient,
            accepted.event.clone(),
        );
        let Some(runtime) = self.runtimes.get_mut(&recipient) else {
            return Ok(DeliveryReceipt::new(recipient, DeliveryState::MailboxFull));
        };
        if !runtime.mailbox_mut().deliver(delivery.clone()) {
            return Ok(DeliveryReceipt::new(recipient, DeliveryState::MailboxFull));
        }
        let progress = driver
            .process_delivery(&delivery)
            .map_err(|error| KernelError::DriverDelivery { recipient, error })?;
        if progress == DeliveryProgress::Processed {
            runtime.mailbox_mut().mark_processed(accepted.sequence);
            Ok(DeliveryReceipt::new(recipient, DeliveryState::Processed))
        } else {
            Ok(DeliveryReceipt::new(recipient, DeliveryState::Delivered))
        }
    }
}
