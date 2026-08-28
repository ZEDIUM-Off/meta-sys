//! Matchable failures produced while evolving a Kernel Runtime.

use crate::{
    execution::DriverError,
    routing::{EventTypeId, RoomAddress},
    system::{
        AddonId, CapabilityId, ComponentDefinitionId, ComponentInstanceId, ContextId, EffectId,
        FacetId, FacetSchemaId, FacetValueKind, GraphEntityKind, RequirementId,
    },
};
use thiserror::Error;

/// A violation that prevents an Event from changing the System Graph.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum KernelError {
    /// The Event attempted to insert an existing Component Definition identity.
    #[error("Component Definition {0:?} already exists")]
    DuplicateComponentDefinition(ComponentDefinitionId),
    /// The Event attempted to insert an existing Component Instance identity.
    #[error("Component Instance {0:?} already exists")]
    DuplicateComponentInstance(ComponentInstanceId),
    /// The Event attempted to insert a Requirement identity already in the graph.
    #[error("Requirement {0:?} already exists")]
    DuplicateRequirement(RequirementId),
    /// The Event attempted to insert a Capability identity already in the graph.
    #[error("Capability {0:?} already exists")]
    DuplicateCapability(CapabilityId),
    /// The configured Driver could not start execution for an activating Instance.
    #[error("could not start Component Instance {instance_id:?}")]
    DriverStart {
        /// Component Instance whose execution failed to start.
        instance_id: ComponentInstanceId,
        /// Failure reported by the configured Driver.
        #[source]
        error: DriverError,
    },
    /// The Event referenced a Component Instance absent from this Runtime.
    #[error("Component Instance {0:?} does not exist")]
    UnknownComponentInstance(ComponentInstanceId),
    /// The Event attempted to insert an Effect identity already in the graph.
    #[error("Effect {0:?} already exists")]
    DuplicateEffect(EffectId),
    /// An Effect can only be introduced by an Active lifecycle owner.
    #[error("Component Instance {0:?} is not Active and cannot own a living Effect")]
    InactiveEffectOwner(ComponentInstanceId),
    /// The configured Driver could not stop execution for a deactivating Instance.
    #[error("could not stop Component Instance {instance_id:?}")]
    DriverStop {
        /// Component Instance whose execution failed to stop.
        instance_id: ComponentInstanceId,
        /// Failure reported by the configured Driver.
        #[source]
        error: DriverError,
    },
    /// The Event attempted to insert an existing Context identity.
    #[error("Context {0:?} already exists")]
    DuplicateContext(ContextId),
    /// The Event referenced a Context absent from this Runtime.
    #[error("Context {0:?} does not exist")]
    UnknownContext(ContextId),
    /// The Event attempted to insert an existing Facet Schema identity.
    #[error("Facet Schema {0:?} already exists")]
    DuplicateFacetSchema(FacetSchemaId),
    /// The Event referenced a Facet Schema absent from this Runtime.
    #[error("Facet Schema {0:?} does not exist")]
    UnknownFacetSchema(FacetSchemaId),
    /// The Event attempted to insert an existing Facet identity.
    #[error("Facet {0:?} already exists")]
    DuplicateFacet(FacetId),
    /// Facet data does not match the kind declared by its Schema.
    #[error("Facet value kind mismatch: expected {expected:?}, received {actual:?}")]
    FacetValueMismatch {
        /// Data kind required by the Facet Schema.
        expected: FacetValueKind,
        /// Data kind carried by the rejected Facet.
        actual: FacetValueKind,
    },
    /// Facet target does not match the graph category declared by its Schema.
    #[error("Facet target kind mismatch: expected {expected:?}, received {actual:?}")]
    FacetTargetMismatch {
        /// Graph category accepted by the Facet Schema.
        expected: GraphEntityKind,
        /// Graph category carried by the rejected Facet.
        actual: GraphEntityKind,
    },
    /// A Facet targeted an entity not represented in the current System Graph.
    #[error("Facet target does not exist in the current System Graph")]
    UnknownFacetTarget,
    /// The Runtime exhausted its local Component Runtime identity space.
    #[error("Component Runtime identity space is exhausted")]
    RuntimeIdentityExhausted,
    /// An active Addon rejected a proposed Binding with an inspectable reason.
    #[error("Addon {addon:?} rejected Binding for Requirement {requirement:?}: {reason}")]
    BindingRejected {
        /// Addon that rejected the Binding proposal.
        addon: AddonId,
        /// Requirement whose Binding was rejected.
        requirement: RequirementId,
        /// Addon-owned rejection reason.
        reason: String,
    },
    /// An active Addon selected a Capability absent from the compatible proposal.
    #[error(
        "Addon {addon:?} selected incompatible Capability {capability:?} for Requirement {requirement:?}"
    )]
    InvalidBindingSelection {
        /// Addon that returned the invalid influence.
        addon: AddonId,
        /// Requirement being resolved.
        requirement: RequirementId,
        /// Capability absent from the compatible candidates.
        capability: CapabilityId,
    },
    /// No Active concrete Room currently carries the logical address.
    #[error("logical Room {0:?} is unavailable")]
    UnavailableRoom(RoomAddress),
    /// A concrete Room could not accept beyond its declared queue bound.
    #[error("Room {0:?} distribution queue is full")]
    RoomQueueFull(RoomAddress),
    /// A concrete Room exhausted its local FIFO sequence space.
    #[error("Room {0:?} exhausted its FIFO sequence space")]
    RoomSequenceExhausted(RoomAddress),
    /// A Driver failed while processing one accepted Delivery.
    #[error("Driver could not process Delivery for {recipient:?}")]
    DriverDelivery {
        /// Component Runtime whose Mailbox accepted the Delivery.
        recipient: ComponentInstanceId,
        /// Failure reported by the configured Driver.
        #[source]
        error: DriverError,
    },
    /// A Component Definition redeclared an existing logical Room address.
    #[error("logical Room {0:?} is already declared")]
    DuplicateRoomAddress(RoomAddress),
    /// A Component Definition repeated the same Room Subscription.
    #[error("Subscription to logical Room {0:?} is duplicated in one Definition")]
    DuplicateSubscription(RoomAddress),
    /// An emit source has no Active Component Runtime.
    #[error("Component Instance {0:?} cannot emit while inactive")]
    InactiveEventSource(ComponentInstanceId),
    /// An Active Component contract does not declare this emitted Event type.
    #[error("Component Instance {emitter:?} does not declare emitted Event type {event_type:?}")]
    UndeclaredEmission {
        /// Active Component Instance attempting to emit.
        emitter: ComponentInstanceId,
        /// Event payload contract absent from its Routing Contract.
        event_type: EventTypeId,
    },
    /// A Driver returned a processing vector inconsistent with the frontier.
    #[error("Driver returned {actual} Delivery results for a frontier of {expected}")]
    InvalidDeliveryFrontSize {
        /// Number of independent Deliveries submitted to the Driver.
        expected: usize,
        /// Number of processing observations returned by the Driver.
        actual: usize,
    },
    /// An emit route references a Room absent from its owning Routing Contract.
    #[error("emit route references undeclared logical Room {0:?}")]
    UndeclaredEmissionRoom(RoomAddress),
    /// A Routing Contract repeats one broadcast Event listener declaration.
    #[error("broadcast listener for Event type {0:?} is duplicated")]
    DuplicateBroadcastSubscription(EventTypeId),
}
