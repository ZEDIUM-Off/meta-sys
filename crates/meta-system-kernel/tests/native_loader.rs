//! End-to-end Loader contract for a trusted native dynamic-library fixture.

use std::path::PathBuf;

use meta_system_kernel::{
    Capability, CapabilityId, ComponentDefinitionId, ComponentInstanceId, ComponentSource,
    KernelRuntime, LoadId, LoadPhase, LoadRecord, LoadRequest, LoadTransition, Loader, LoaderError,
    LoaderEvent, NativeMaterializer, Requirement, RequirementId,
};

/// Resolves the un-hashed cdylib emitted beside this test profile.
fn fixture_path() -> PathBuf {
    assert_eq!(meta_system_native_fixture::fixture_marker(), 42);
    if let Some(path) = std::env::var_os("META_SYS_NATIVE_FIXTURE") {
        return PathBuf::from(path);
    }
    let executable = std::env::current_exe().expect("test executable path is available");
    executable
        .parent()
        .expect("Cargo dependency artifact directory is available")
        .join(libloading::library_filename("meta_system_native_fixture"))
}

/// Drives the native adapter through every ordered Loader phase.
fn drive_native_loader(
    loader: &mut Loader<NativeMaterializer>,
    runtime: &mut KernelRuntime,
    load: LoadId,
    instance: ComponentInstanceId,
) -> Result<Vec<LoadPhase>, LoaderError> {
    let request = LoadRequest::new(
        load,
        ComponentSource::new(fixture_path().to_string_lossy()),
        instance,
    );
    [
        LoaderEvent::Declare(request),
        LoaderEvent::Locate(load),
        LoaderEvent::Materialize(load),
        LoaderEvent::Inspect(load),
        LoaderEvent::Admit(load),
        LoaderEvent::Register(load),
        LoaderEvent::MarkReady(load),
    ]
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

#[test]
fn native_materializer_loads_complete_definition_end_to_end() -> Result<(), LoaderError> {
    // Arrange
    let mut loader = Loader::new(NativeMaterializer::new());
    let mut runtime = KernelRuntime::new();
    let load = LoadId::new(1);
    let instance = ComponentInstanceId::new(42);

    // Act
    let phases = drive_native_loader(&mut loader, &mut runtime, load, instance)?;

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
    let graph = runtime.graph();
    let definition = graph
        .definition(ComponentDefinitionId::new(42))
        .expect("native Definition is registered");
    assert_eq!(
        definition
            .requirements()
            .iter()
            .map(Requirement::id)
            .collect::<Vec<_>>(),
        vec![RequirementId::new(700)]
    );
    assert_eq!(
        definition
            .capabilities()
            .iter()
            .map(Capability::id)
            .collect::<Vec<_>>(),
        vec![CapabilityId::new(800)]
    );
    assert!(graph.instance(instance).is_some());
    assert_eq!(
        loader.load(load).map(LoadRecord::phase),
        Some(LoadPhase::Ready)
    );
    Ok(())
}
