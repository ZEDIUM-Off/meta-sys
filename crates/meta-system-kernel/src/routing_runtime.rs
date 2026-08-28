//! Public Kernel Runtime routing operations.

use crate::{
    BroadcastReceipt, ComponentInstanceId, Event, EventLoopDriver, KernelError, KernelRuntime,
    RoomAddress, SendReceipt,
};

impl<Driver: EventLoopDriver> KernelRuntime<Driver> {
    /// Sends one typed Event to an available logical Room.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when no Active Room carries the address or one
    /// routing or Driver invariant prevents the distribution step.
    pub fn send(&mut self, address: RoomAddress, event: Event) -> Result<SendReceipt, KernelError> {
        self.graph.send(&mut self.driver, address, event)
    }

    /// Emits one Event through Rooms declared by the source Component contract.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when the source is inactive, the Event type is
    /// undeclared, or any declared Room distribution fails.
    pub fn emit(
        &mut self,
        source: ComponentInstanceId,
        event: &Event,
    ) -> Result<Vec<SendReceipt>, KernelError> {
        self.graph.emit(&mut self.driver, source, event)
    }

    /// Broadcasts one Event to every Active registered listener.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when a Mailbox or Driver invariant prevents the
    /// distribution frontier from completing.
    pub fn broadcast(&mut self, event: &Event) -> Result<BroadcastReceipt, KernelError> {
        self.graph.broadcast(&mut self.driver, event)
    }
}
