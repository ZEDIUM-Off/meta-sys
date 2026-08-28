//! Public-contract tests for Loader Addon admission timing.

use std::sync::{Arc, Mutex};

use meta_system_kernel::{
    AddonId, ComponentDefinition, ComponentDefinitionId, ComponentInstanceId, ComponentSource,
    DeterministicMaterializer, FacetSchema, FacetSchemaId, FacetValueKind, GraphEntityKind,
    HookOrder, KernelEvent, KernelRuntime, LoadId, LoadPhase, LoadRecord, LoadRejection,
    LoadRequest, LoadTransition, Loader, LoaderDecision, LoaderError, LoaderEvent, LoaderHook,
    LoaderProposal,
};

/// Hook fixture that records evaluation order and returns one stable decision.
#[derive(Debug)]
struct RecordingHook {
    addon: AddonId,
    order: HookOrder,
    decision: LoaderDecision,
    observations: Arc<Mutex<Vec<(AddonId, LoadId)>>>,
}

impl LoaderHook for RecordingHook {
    fn addon(&self) -> AddonId {
        self.addon
    }

    fn order(&self) -> HookOrder {
        self.order
    }

    fn evaluate(&self, proposal: LoaderProposal<'_>) -> LoaderDecision {
        self.observations
            .lock()
            .expect("observation lock remains available")
            .push((self.addon, proposal.load()));
        self.decision.clone()
    }
}

/// Builds one deterministic Loader with a complete Definition.
const fn loader() -> Loader<DeterministicMaterializer> {
    Loader::new(DeterministicMaterializer::new(ComponentDefinition::new(
        ComponentDefinitionId::new(1),
    )))
}

/// Drives a unique lifecycle through complete inspection.
fn inspect(
    loader: &mut Loader<DeterministicMaterializer>,
    runtime: &mut KernelRuntime,
    load: LoadId,
    instance: ComponentInstanceId,
) -> Result<(), LoaderError> {
    let request = LoadRequest::new(load, ComponentSource::new("memory://component"), instance);
    for event in [
        LoaderEvent::Declare(request),
        LoaderEvent::Locate(load),
        LoaderEvent::Materialize(load),
        LoaderEvent::Inspect(load),
    ] {
        let _ = loader.handle(event, runtime)?;
    }
    Ok(())
}

/// Creates one recording hook owned by a Loader Addon.
fn hook(
    addon: u64,
    order: u32,
    decision: LoaderDecision,
    observations: &Arc<Mutex<Vec<(AddonId, LoadId)>>>,
) -> RecordingHook {
    RecordingHook {
        addon: AddonId::new(addon),
        order: HookOrder::new(order),
        decision,
        observations: Arc::clone(observations),
    }
}

#[test]
fn active_loader_hooks_run_in_order_and_reject_inspectably() -> Result<(), LoaderError> {
    // Arrange
    let mut loader = loader();
    let mut runtime = KernelRuntime::new();
    let load = LoadId::new(10);
    let observations = Arc::new(Mutex::new(Vec::new()));
    inspect(
        &mut loader,
        &mut runtime,
        load,
        ComponentInstanceId::new(10),
    )?;
    loader.add_hook(hook(
        2,
        20,
        LoaderDecision::Reject("denied".to_owned()),
        &observations,
    ));
    loader.add_hook(hook(1, 10, LoaderDecision::Allow, &observations));

    // Act
    let outcome = loader.handle(LoaderEvent::Admit(load), &mut runtime)?;

    // Assert
    assert_eq!(
        outcome.transition().map(LoadTransition::current),
        Some(LoadPhase::Rejected)
    );
    assert_eq!(
        observations
            .lock()
            .expect("observation lock remains available")
            .as_slice(),
        &[(AddonId::new(1), load), (AddonId::new(2), load)]
    );
    let record = loader.load(load).expect("load remains inspectable");
    assert_eq!(
        record.rejection().map(LoadRejection::reason),
        Some("denied")
    );
    assert_eq!(
        record.rejection().and_then(LoadRejection::addon),
        Some(AddonId::new(2))
    );
    Ok(())
}

#[test]
fn hook_activation_never_replays_completed_admission() -> Result<(), LoaderError> {
    // Arrange
    let mut loader = loader();
    let mut runtime = KernelRuntime::new();
    let admitted = LoadId::new(10);
    let future = LoadId::new(20);
    inspect(
        &mut loader,
        &mut runtime,
        admitted,
        ComponentInstanceId::new(10),
    )?;
    let _ = loader.handle(LoaderEvent::Admit(admitted), &mut runtime)?;
    let observations = Arc::new(Mutex::new(Vec::new()));
    loader.add_hook(hook(
        1,
        10,
        LoaderDecision::Reject("late".to_owned()),
        &observations,
    ));
    inspect(
        &mut loader,
        &mut runtime,
        future,
        ComponentInstanceId::new(20),
    )?;

    // Act
    let _ = loader.handle(LoaderEvent::Admit(future), &mut runtime)?;

    // Assert
    assert_eq!(
        loader.load(admitted).map(LoadRecord::phase),
        Some(LoadPhase::Admitted)
    );
    assert_eq!(
        loader.load(future).map(LoadRecord::phase),
        Some(LoadPhase::Rejected)
    );
    assert_eq!(
        observations
            .lock()
            .expect("observation lock remains available")
            .as_slice(),
        &[(AddonId::new(1), future)]
    );
    Ok(())
}

#[test]
fn one_addon_may_hold_loader_and_system_roles_without_correlation() -> Result<(), LoaderError> {
    // Arrange
    let addon = AddonId::new(7);
    let observations = Arc::new(Mutex::new(Vec::new()));
    let mut loader = loader();
    loader.add_hook(hook(7, 10, LoaderDecision::Allow, &observations));
    let mut runtime = KernelRuntime::new();
    let schema = FacetSchema::new(
        FacetSchemaId::new(7),
        addon,
        FacetValueKind::Text,
        GraphEntityKind::ComponentInstance,
    );
    let _ = runtime
        .handle(KernelEvent::register_facet_schema(schema))
        .expect("System role is independently accepted");
    let load = LoadId::new(10);
    inspect(
        &mut loader,
        &mut runtime,
        load,
        ComponentInstanceId::new(10),
    )?;

    // Act
    let _ = loader.handle(LoaderEvent::Admit(load), &mut runtime)?;

    // Assert
    assert_eq!(
        loader.load(load).map(LoadRecord::phase),
        Some(LoadPhase::Admitted)
    );
    assert_eq!(
        runtime
            .graph()
            .facet_schema(FacetSchemaId::new(7))
            .map(FacetSchema::owner),
        Some(addon)
    );
    Ok(())
}
