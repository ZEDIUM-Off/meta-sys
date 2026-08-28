//! Typed Event routing through bounded Rooms and Component Mailboxes.
//!
//! A [`RoutingContract`] declares Rooms, Subscriptions, emissions, broadcast listeners, and the
//! Component-owned [`MailboxPolicy`]. At runtime, the Kernel accepts one [`Event`] into a [`Room`],
//! creates observable [`Delivery`] attempts, and places them in subscribed [`Mailbox`] queues.
//!
//! ```text
//! send / emit / broadcast
//!           │
//!           ▼
//!      Room (local FIFO)
//!           │ one distribution step
//!           ▼
//!      Subscription
//!           │
//!           ▼
//!   Delivery ──attempt──▶ Mailbox ──Driver──▶ processing observation
//!           │
//!           └──────────────────────────────▶ Receipt
//! ```
//!
//! # Interface
//!
//! This module exposes declarations and observations. Mutation remains on
//! [`KernelRuntime`](crate::runtime::KernelRuntime) through
//! [`send`](crate::runtime::KernelRuntime::send), [`emit`](crate::runtime::KernelRuntime::emit),
//! and [`broadcast`](crate::runtime::KernelRuntime::broadcast); the distribution engine and graph
//! mutations remain private.
//!
//! # Invariants
//!
//! FIFO order is local to one concrete Room and no causal order exists between Rooms. Room and
//! Mailbox queues are bounded. Overflow follows the Component policy and never creates an implicit
//! retry, transaction, or global backpressure mechanism. A logical [`RoomAddress`] may survive
//! deactivation while its concrete [`RoomRuntimeId`] does not.
//!
//! # Example
//!
//! ```
//! use meta_system_kernel::routing::{Event, EventId, EventTypeId};
//!
//! let event = Event::new(EventId::new(1), EventTypeId::new(7), b"ready".to_vec());
//! assert_eq!(event.payload(), b"ready");
//! ```

mod broadcast_receipt;
mod contract;
mod delivery;
mod emission;
mod engine;
mod event;
mod identity;
mod mailbox;
mod mailbox_policy;
mod queue_capacity;
mod room;
mod send_receipt;
mod subscription;

pub use broadcast_receipt::BroadcastReceipt;
pub use contract::RoutingContract;
pub use delivery::{Delivery, DeliveryOrigin, DeliveryProgress};
pub use emission::{BroadcastSubscription, EmissionDeclaration};
pub use event::Event;
pub use identity::{EventId, EventTypeId, RoomAddress, RoomRuntimeId, RoomSequence};
pub use mailbox::Mailbox;
pub use mailbox_policy::{MailboxOverflowStrategy, MailboxPolicy};
pub use queue_capacity::QueueCapacity;
pub use room::{Room, RoomDeclaration};
pub use send_receipt::{DeliveryReceipt, DeliveryState, SendReceipt};
pub use subscription::{Subscription, SubscriptionDeclaration};
