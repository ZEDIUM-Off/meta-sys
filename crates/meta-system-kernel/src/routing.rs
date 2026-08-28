//! Private Room distribution and Component Mailbox mutation engine.

use std::collections::BTreeSet;

use crate::{
    BroadcastReceipt, ComponentDefinition, ComponentInstanceId, ComponentRuntimeId, Delivery,
    DeliveryProgress, DeliveryReceipt, Event, EventLoopDriver, EventTypeId, KernelError, Room,
    RoomAddress, RoomRuntimeId, RoomSequence, SendReceipt, Subscription, graph::GraphState,
    mailbox::MailboxPlacement,
};

/// Accepted work from one concrete Room before Mailbox distribution.
#[derive(Debug)]
struct RoomBatch {
    /// Logical Room that accepted this Event.
    address: RoomAddress,
    /// FIFO position local to the concrete Room lifecycle.
    sequence: RoomSequence,
    /// Independent subscriber Deliveries produced by this step.
    deliveries: Vec<Delivery>,
}

/// Mailbox-accepted Delivery awaiting one Driver-front observation.
#[derive(Debug)]
struct PendingDelivery {
    /// Position in the final recipient-local receipt vector.
    receipt: usize,
    /// Delivery retained for Driver observation and Mailbox completion.
    delivery: Delivery,
}

impl GraphState {
    /// Validates logical routing declarations before graph mutation.
    pub(super) fn validate_routing_registration(
        &self,
        definition: &ComponentDefinition,
    ) -> Result<(), KernelError> {
        let existing = self
            .definitions
            .values()
            .flat_map(|definition| definition.routing().rooms())
            .map(|room| room.address())
            .collect::<BTreeSet<_>>();
        let mut rooms = BTreeSet::new();
        for room in definition.routing().rooms() {
            if existing.contains(&room.address()) || !rooms.insert(room.address()) {
                return Err(KernelError::DuplicateRoomAddress(room.address()));
            }
        }
        let mut subscriptions = BTreeSet::new();
        for subscription in definition.routing().subscriptions() {
            if !subscriptions.insert(subscription.room()) {
                return Err(KernelError::DuplicateSubscription(subscription.room()));
            }
        }
        for emission in definition.routing().emissions() {
            if !rooms.contains(&emission.room()) {
                return Err(KernelError::UndeclaredEmissionRoom(emission.room()));
            }
        }
        let mut broadcasts = BTreeSet::new();
        for subscription in definition.routing().broadcast_subscriptions() {
            if !broadcasts.insert(subscription.event_type()) {
                return Err(KernelError::DuplicateBroadcastSubscription(
                    subscription.event_type(),
                ));
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
        for (ordinal, declaration) in definition.routing().rooms().iter().copied().enumerate() {
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
                .routing()
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
        let mut receipts = self.route_rooms(driver, vec![(address, event)])?;
        Ok(receipts.remove(0))
    }

    /// Publishes through every typed Room route declared by an Active source.
    pub(super) fn emit<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        source: ComponentInstanceId,
        event: &Event,
    ) -> Result<Vec<SendReceipt>, KernelError> {
        if !self.runtimes.contains_key(&source) {
            return Err(KernelError::InactiveEventSource(source));
        }
        let routes = self.emission_rooms(source, event.event_type())?;
        self.route_rooms(
            driver,
            routes
                .into_iter()
                .map(|room| (room, event.clone()))
                .collect(),
        )
    }

    /// Publishes directly to every Active listener of one Event type.
    pub(super) fn broadcast<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        event: &Event,
    ) -> Result<BroadcastReceipt, KernelError> {
        let deliveries = self
            .broadcast_recipients(event.event_type())
            .into_iter()
            .map(|recipient| Delivery::from_broadcast(recipient, event.clone()))
            .collect();
        Ok(BroadcastReceipt::new(
            self.deliver_front(driver, deliveries)?,
        ))
    }

    /// Resolves typed emit routes without granting access to undeclared Rooms.
    fn emission_rooms(
        &self,
        source: ComponentInstanceId,
        event_type: EventTypeId,
    ) -> Result<Vec<RoomAddress>, KernelError> {
        let definition = self
            .instances
            .get(&source)
            .and_then(|instance| self.definitions.get(&instance.definition_id()))
            .ok_or(KernelError::InactiveEventSource(source))?;
        let rooms = definition
            .routing()
            .emissions()
            .iter()
            .filter(|emission| emission.event_type() == event_type)
            .map(|emission| emission.room())
            .collect::<Vec<_>>();
        if rooms.is_empty() {
            Err(KernelError::UndeclaredEmission {
                emitter: source,
                event_type,
            })
        } else {
            Ok(rooms)
        }
    }

    /// Returns Active broadcast listeners in deterministic Instance order.
    fn broadcast_recipients(&self, event_type: EventTypeId) -> Vec<ComponentInstanceId> {
        self.runtimes
            .keys()
            .filter(|instance_id| {
                self.instances
                    .get(instance_id)
                    .and_then(|instance| self.definitions.get(&instance.definition_id()))
                    .is_some_and(|definition| {
                        definition
                            .routing()
                            .broadcast_subscriptions()
                            .iter()
                            .any(|subscription| subscription.event_type() == event_type)
                    })
            })
            .copied()
            .collect()
    }

    /// Accepts independent Room Events before one shared Driver frontier.
    fn route_rooms<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        routes: Vec<(RoomAddress, Event)>,
    ) -> Result<Vec<SendReceipt>, KernelError> {
        let mut batches = Vec::with_capacity(routes.len());
        for (address, event) in routes {
            let accepted = self
                .rooms
                .get_mut(&address)
                .ok_or(KernelError::UnavailableRoom(address))?
                .accept(event)?;
            let deliveries = self
                .subscriptions
                .iter()
                .filter(|subscription| subscription.room() == address)
                .map(|subscription| {
                    Delivery::from_room(
                        address,
                        accepted.sequence,
                        subscription.subscriber(),
                        accepted.event.clone(),
                    )
                })
                .collect();
            batches.push(RoomBatch {
                address,
                sequence: accepted.sequence,
                deliveries,
            });
        }
        self.distribute_batches(driver, batches)
    }

    /// Distributes all independent Room batches through one Driver frontier.
    fn distribute_batches<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        batches: Vec<RoomBatch>,
    ) -> Result<Vec<SendReceipt>, KernelError> {
        let counts = batches
            .iter()
            .map(|batch| batch.deliveries.len())
            .collect::<Vec<_>>();
        let deliveries = batches
            .iter()
            .flat_map(|batch| batch.deliveries.iter().cloned())
            .collect();
        let delivery_receipts = self.deliver_front(driver, deliveries)?;
        let mut offset = 0;
        Ok(batches
            .into_iter()
            .zip(counts)
            .map(|(batch, count)| {
                let end = offset + count;
                let receipts = delivery_receipts[offset..end].to_vec();
                offset = end;
                SendReceipt::new(batch.address, batch.sequence, receipts)
            })
            .collect())
    }

    /// Places a Delivery frontier in Mailboxes and observes processing once.
    fn deliver_front<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        deliveries: Vec<Delivery>,
    ) -> Result<Vec<DeliveryReceipt>, KernelError> {
        let mut receipts = Vec::with_capacity(deliveries.len());
        let mut pending = Vec::new();
        for delivery in deliveries {
            let recipient = delivery.recipient();
            let placement = self
                .runtimes
                .get_mut(&recipient)
                .map_or(MailboxPlacement::RejectedFull, |runtime| {
                    runtime.mailbox_mut().deliver(delivery.clone())
                });
            let state = placement.delivery_state();
            if placement != MailboxPlacement::RejectedFull {
                pending.push(PendingDelivery {
                    receipt: receipts.len(),
                    delivery,
                });
            }
            receipts.push(DeliveryReceipt::new(recipient, state));
        }
        self.complete_front(driver, &pending, &mut receipts)?;
        Ok(receipts)
    }

    /// Applies Driver observations without retrying any rejected Delivery.
    fn complete_front<Driver: EventLoopDriver>(
        &mut self,
        driver: &mut Driver,
        pending: &[PendingDelivery],
        receipts: &mut [DeliveryReceipt],
    ) -> Result<(), KernelError> {
        if pending.is_empty() {
            return Ok(());
        }
        let deliveries = pending
            .iter()
            .map(|entry| entry.delivery.clone())
            .collect::<Vec<_>>();
        let progress = driver
            .process_delivery_front(&deliveries)
            .map_err(|error| KernelError::DriverDelivery {
                recipient: deliveries[0].recipient(),
                error,
            })?;
        if progress.len() != pending.len() {
            return Err(KernelError::InvalidDeliveryFrontSize {
                expected: pending.len(),
                actual: progress.len(),
            });
        }
        for (entry, state) in pending.iter().zip(progress) {
            if state != DeliveryProgress::Processed {
                continue;
            }
            let Some(runtime) = self.runtimes.get_mut(&entry.delivery.recipient()) else {
                continue;
            };
            runtime.mailbox_mut().mark_processed(&entry.delivery);
            receipts[entry.receipt].mark_processed();
        }
        Ok(())
    }
}
