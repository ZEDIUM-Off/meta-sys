//! Public Kernel Runtime routing operations.

use crate::{Event, EventLoopDriver, KernelError, KernelRuntime, RoomAddress, SendReceipt};

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
}
