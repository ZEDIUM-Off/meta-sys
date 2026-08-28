//! Public-contract tests for ordered Addon hooks at the Binding seam.

use meta_system_kernel::{
    AddonId, Binding, BindingDecision, BindingHook, BindingProposal, Capability,
    CapabilityContractId, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentInstanceId, HookOrder, KernelError, KernelEvent, KernelRuntime, Requirement,
    RequirementId,
};

#[test]
fn policy_hook_selects_another_compatible_provider() -> Result<(), KernelError> {
    // Arrange
    let contract = CapabilityContractId::new(1_000);
    let first_capability = CapabilityId::new(100);
    let selected_capability = CapabilityId::new(200);
    let first_provider = ComponentInstanceId::new(10);
    let selected_provider = ComponentInstanceId::new(20);
    let consumer = ComponentInstanceId::new(30);
    let requirement = RequirementId::new(300);
    let mut runtime = KernelRuntime::new();
    for (definition, instance, capability) in [
        (1, first_provider, first_capability),
        (2, selected_provider, selected_capability),
    ] {
        let provider = ComponentDefinition::new(ComponentDefinitionId::new(definition))
            .with_capability(Capability::new(capability, contract));
        let _provider = runtime.handle(KernelEvent::register_component(provider, instance))?;
    }
    runtime.add_binding_hook(RequireSelectionHook {
        addon: AddonId::new(2),
        order: HookOrder::new(20),
        required: selected_capability,
    });
    runtime.add_binding_hook(SelectHook {
        addon: AddonId::new(1),
        order: HookOrder::new(10),
        capability: selected_capability,
    });
    let consumer_definition = ComponentDefinition::new(ComponentDefinitionId::new(3))
        .with_requirement(Requirement::necessary(requirement, contract));

    // Act
    let _outcome = runtime.handle(KernelEvent::register_component(
        consumer_definition,
        consumer,
    ))?;

    // Assert
    let graph = runtime.graph();
    let binding = graph.binding(requirement);
    assert_eq!(
        binding.map(Binding::capability_id),
        Some(selected_capability)
    );
    assert_eq!(binding.map(Binding::provider_id), Some(selected_provider));
    Ok(())
}

#[test]
fn policy_hook_rejects_binding_with_inspectable_reason() -> Result<(), KernelError> {
    // Arrange
    let addon = AddonId::new(3);
    let contract = CapabilityContractId::new(2_000);
    let requirement = RequirementId::new(400);
    let consumer = ComponentInstanceId::new(40);
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(4))
        .with_capability(Capability::new(CapabilityId::new(400), contract));
    let consumer_definition = ComponentDefinition::new(ComponentDefinitionId::new(5))
        .with_requirement(Requirement::necessary(requirement, contract));
    let mut runtime = KernelRuntime::new();
    let _provider = runtime.handle(KernelEvent::register_component(
        provider,
        ComponentInstanceId::new(50),
    ))?;
    runtime.add_binding_hook(RejectHook {
        addon,
        order: HookOrder::new(10),
    });

    // Act
    let result = runtime.handle(KernelEvent::register_component(
        consumer_definition,
        consumer,
    ));

    // Assert
    assert!(matches!(
        result,
        Err(KernelError::BindingRejected {
            addon: rejected_by,
            requirement: rejected_requirement,
            reason,
        }) if rejected_by == addon
            && rejected_requirement == requirement
            && reason == "provider denied by policy"
    ));
    assert!(runtime.graph().binding(requirement).is_none());
    assert!(runtime.graph().component_runtime(consumer).is_none());
    Ok(())
}

#[test]
fn policy_cannot_select_capability_outside_declared_contract() -> Result<(), KernelError> {
    // Arrange
    let addon = AddonId::new(4);
    let contract = CapabilityContractId::new(3_000);
    let requirement = RequirementId::new(600);
    let incompatible_selection = CapabilityId::new(9_999);
    let provider = ComponentDefinition::new(ComponentDefinitionId::new(6))
        .with_capability(Capability::new(CapabilityId::new(600), contract));
    let consumer = ComponentDefinition::new(ComponentDefinitionId::new(7))
        .with_requirement(Requirement::necessary(requirement, contract));
    let mut runtime = KernelRuntime::new();
    let _provider = runtime.handle(KernelEvent::register_component(
        provider,
        ComponentInstanceId::new(60),
    ))?;
    runtime.add_binding_hook(SelectHook {
        addon,
        order: HookOrder::new(10),
        capability: incompatible_selection,
    });

    // Act
    let result = runtime.handle(KernelEvent::register_component(
        consumer,
        ComponentInstanceId::new(70),
    ));

    // Assert
    assert_eq!(
        result,
        Err(KernelError::InvalidBindingSelection {
            addon,
            requirement,
            capability: incompatible_selection,
        })
    );
    Ok(())
}

/// Addon policy that selects one compatible Capability.
#[derive(Debug)]
struct SelectHook {
    /// Addon that owns this hook.
    addon: AddonId,
    /// Declared deterministic hook position.
    order: HookOrder,
    /// Compatible Capability selected by the hook.
    capability: CapabilityId,
}

impl BindingHook for SelectHook {
    fn addon(&self) -> AddonId {
        self.addon
    }

    fn order(&self) -> HookOrder {
        self.order
    }

    fn evaluate(&self, _proposal: &BindingProposal) -> BindingDecision {
        BindingDecision::SelectCapability(self.capability)
    }
}

/// Later hook that proves earlier ordered influence is already visible.
#[derive(Debug)]
struct RequireSelectionHook {
    /// Addon that owns this hook.
    addon: AddonId,
    /// Declared deterministic hook position.
    order: HookOrder,
    /// Capability that must have been selected by an earlier hook.
    required: CapabilityId,
}

impl BindingHook for RequireSelectionHook {
    fn addon(&self) -> AddonId {
        self.addon
    }

    fn order(&self) -> HookOrder {
        self.order
    }

    fn evaluate(&self, proposal: &BindingProposal) -> BindingDecision {
        if proposal.selected() == self.required {
            BindingDecision::Allow
        } else {
            BindingDecision::Reject(String::from("earlier selection was not applied"))
        }
    }
}

/// Policy hook that rejects every proposed Binding.
#[derive(Debug)]
struct RejectHook {
    /// Addon that owns this hook.
    addon: AddonId,
    /// Declared deterministic hook position.
    order: HookOrder,
}

impl BindingHook for RejectHook {
    fn addon(&self) -> AddonId {
        self.addon
    }

    fn order(&self) -> HookOrder {
        self.order
    }

    fn evaluate(&self, _proposal: &BindingProposal) -> BindingDecision {
        BindingDecision::Reject(String::from("provider denied by policy"))
    }
}
