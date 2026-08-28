//! Configuration of active Addon hooks on a Kernel Runtime.

use crate::{BindingHook, KernelRuntime};

impl<Driver> KernelRuntime<Driver> {
    /// Activates one Addon hook at the global Binding policy seam.
    pub fn add_binding_hook(&mut self, hook: impl BindingHook + 'static) {
        self.binding_hooks.push(Box::new(hook));
        self.binding_hooks
            .sort_by_key(|hook| (hook.order(), hook.addon()));
    }
}
