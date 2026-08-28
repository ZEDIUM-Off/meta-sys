//! Public-contract tests for the ordered Loader seam.

use meta_system_kernel::{
    ComponentDefinition, ComponentDefinitionId, ComponentInstanceId, ComponentSource,
    DeterministicMaterializer, KernelEvent, KernelRuntime, LoadId, LoadPhase, LoadRecord,
    LoadRequest, LoadTransition, Loader, LoaderError, LoaderEvent,
};

/// Builds a deterministic Loader fixture with stable public identities.
fn fixture() -> (
    Loader<DeterministicMaterializer>,
    KernelRuntime,
    LoadId,
    ComponentDefinitionId,
    ComponentInstanceId,
) {
    let definition_id = ComponentDefinitionId::new(1);
    let definition = ComponentDefinition::new(definition_id);
    (
        Loader::new(DeterministicMaterializer::new(definition)),
        KernelRuntime::new(),
        LoadId::new(10),
        definition_id,
        ComponentInstanceId::new(100),
    )
}

/// Drives one Loader through each successful phase and returns observations.
fn drive_to_ready(
    loader: &mut Loader<DeterministicMaterializer>,
    runtime: &mut KernelRuntime,
    load_id: LoadId,
    instance_id: ComponentInstanceId,
) -> Result<Vec<LoadPhase>, LoaderError> {
    let request = LoadRequest::new(
        load_id,
        ComponentSource::new("memory://component"),
        instance_id,
    );
    let events = [
        LoaderEvent::Declare(request),
        LoaderEvent::Locate(load_id),
        LoaderEvent::Materialize(load_id),
        LoaderEvent::Inspect(load_id),
        LoaderEvent::Admit(load_id),
        LoaderEvent::Register(load_id),
        LoaderEvent::MarkReady(load_id),
    ];
    events
        .into_iter()
        .map(|event| loader.handle(event, runtime))
        .collect::<Result<Vec<_>, _>>()
        .map(|outcomes| {
            outcomes
                .into_iter()
                .filter_map(|outcome| outcome.transition().map(LoadTransition::current))
                .collect()
        })
}

/// Drives one declared lifecycle through successful inspection.
fn drive_to_inspected(
    loader: &mut Loader<DeterministicMaterializer>,
    runtime: &mut KernelRuntime,
    load_id: LoadId,
    instance_id: ComponentInstanceId,
) -> Result<(), LoaderError> {
    let request = LoadRequest::new(
        load_id,
        ComponentSource::new("memory://component"),
        instance_id,
    );
    for event in [
        LoaderEvent::Declare(request),
        LoaderEvent::Locate(load_id),
        LoaderEvent::Materialize(load_id),
        LoaderEvent::Inspect(load_id),
    ] {
        let _ = loader.handle(event, runtime)?;
    }
    Ok(())
}

#[test]
fn loader_reaches_ready_after_registering_complete_definition() -> Result<(), LoaderError> {
    // Arrange
    let (mut loader, mut runtime, load_id, definition_id, instance_id) = fixture();

    // Act
    let phases = drive_to_ready(&mut loader, &mut runtime, load_id, instance_id)?;

    // Assert
    assert_eq!(
        phases,
        vec![
            LoadPhase::Declared,
            LoadPhase::Located,
            LoadPhase::Materialized,
            LoadPhase::Inspected,
            LoadPhase::Admitted,
            LoadPhase::Registered,
            LoadPhase::Ready,
        ]
    );
    let record = loader.load(load_id);
    assert_eq!(record.map(LoadRecord::phase), Some(LoadPhase::Ready));
    assert_eq!(
        record
            .and_then(LoadRecord::definition)
            .map(ComponentDefinition::id),
        Some(definition_id)
    );
    assert!(runtime.graph().definition(definition_id).is_some());
    assert!(runtime.graph().instance(instance_id).is_some());
    Ok(())
}

#[test]
fn loader_rejects_events_that_bypass_required_phases() {
    // Arrange
    let (mut loader, mut runtime, load_id, _, instance_id) = fixture();
    let request = LoadRequest::new(
        load_id,
        ComponentSource::new("memory://component"),
        instance_id,
    );
    let _ = loader.handle(LoaderEvent::Declare(request), &mut runtime);

    // Act
    let result = loader.handle(LoaderEvent::Inspect(load_id), &mut runtime);

    // Assert
    assert_eq!(
        result,
        Err(LoaderError::InvalidTransition {
            expected: LoadPhase::Materialized,
            actual: LoadPhase::Declared,
        })
    );
    assert_eq!(
        loader.load(load_id).map(LoadRecord::phase),
        Some(LoadPhase::Declared)
    );
}

#[test]
fn rejected_definition_is_terminal_and_inspectable() -> Result<(), LoaderError> {
    // Arrange
    let (mut loader, mut runtime, load_id, definition_id, instance_id) = fixture();
    drive_to_inspected(&mut loader, &mut runtime, load_id, instance_id)?;

    // Act
    let _ = loader.handle(
        LoaderEvent::Reject {
            id: load_id,
            reason: "policy denied".to_owned(),
        },
        &mut runtime,
    )?;
    let register = loader.handle(LoaderEvent::Register(load_id), &mut runtime);

    // Assert
    let record = loader
        .load(load_id)
        .expect("declared load remains inspectable");
    assert_eq!(record.phase(), LoadPhase::Rejected);
    assert_eq!(record.rejection(), Some("policy denied"));
    assert_eq!(
        register,
        Err(LoaderError::InvalidTransition {
            expected: LoadPhase::Admitted,
            actual: LoadPhase::Rejected,
        })
    );
    assert!(runtime.graph().definition(definition_id).is_none());
    Ok(())
}

#[test]
fn failed_kernel_registration_never_reaches_registered_or_ready() -> Result<(), LoaderError> {
    // Arrange
    let (mut loader, mut runtime, load_id, definition_id, instance_id) = fixture();
    drive_to_inspected(&mut loader, &mut runtime, load_id, instance_id)?;
    let _ = loader.handle(LoaderEvent::Admit(load_id), &mut runtime)?;
    let _ = runtime.handle(KernelEvent::register_component(
        ComponentDefinition::new(definition_id),
        ComponentInstanceId::new(999),
    ));

    // Act
    let register = loader.handle(LoaderEvent::Register(load_id), &mut runtime);
    let ready = loader.handle(LoaderEvent::MarkReady(load_id), &mut runtime);

    // Assert
    assert!(matches!(register, Err(LoaderError::Kernel(_))));
    assert_eq!(
        loader.load(load_id).map(LoadRecord::phase),
        Some(LoadPhase::Admitted)
    );
    assert_eq!(
        ready,
        Err(LoaderError::InvalidTransition {
            expected: LoadPhase::Registered,
            actual: LoadPhase::Admitted,
        })
    );
    Ok(())
}
