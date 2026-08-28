//! Public-contract tests for Addon-owned typed Facets and Contexts.

use meta_system_kernel::{
    AddonId, ComponentDefinition, ComponentDefinitionId, ComponentInstanceId, Context, ContextId,
    ContextOwner, ContextVisibility, Facet, FacetId, FacetSchema, FacetSchemaId, FacetTarget,
    FacetValue, FacetValueKind, GraphEntityKind, KernelError, KernelEvent, KernelRuntime,
};

#[test]
fn addon_attaches_typed_facet_to_instance_in_context() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = facet_fixture()?;
    let facet = Facet::new(
        fixture.facet,
        fixture.schema,
        fixture.context,
        FacetTarget::ComponentInstance(fixture.instance),
        FacetValue::Text(String::from("editor")),
    );

    // Act
    let _outcome = fixture.runtime.handle(KernelEvent::attach_facet(facet))?;

    // Assert
    let graph = fixture.runtime.graph();
    assert_eq!(
        graph.context(fixture.context).map(Context::owner),
        Some(ContextOwner::Addon(fixture.addon))
    );
    assert_eq!(
        graph.context(fixture.context).map(Context::visibility),
        Some(ContextVisibility::Descendants)
    );
    assert_eq!(
        graph.facet_schema(fixture.schema).map(FacetSchema::owner),
        Some(fixture.addon)
    );
    assert_eq!(
        graph.facet(fixture.facet).map(Facet::target),
        Some(FacetTarget::ComponentInstance(fixture.instance))
    );
    let expected_value = FacetValue::Text(String::from("editor"));
    assert_eq!(
        graph.facet(fixture.facet).map(Facet::value),
        Some(&expected_value)
    );
    Ok(())
}

#[test]
fn facet_value_kind_mismatch_is_rejected() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = facet_fixture()?;
    let facet = Facet::new(
        fixture.facet,
        fixture.schema,
        fixture.context,
        FacetTarget::ComponentInstance(fixture.instance),
        FacetValue::Integer(42),
    );

    // Act
    let result = fixture.runtime.handle(KernelEvent::attach_facet(facet));

    // Assert
    assert_eq!(
        result,
        Err(KernelError::FacetValueMismatch {
            expected: FacetValueKind::Text,
            actual: FacetValueKind::Integer,
        })
    );
    assert!(fixture.runtime.graph().facet(fixture.facet).is_none());
    Ok(())
}

#[test]
fn facet_target_kind_mismatch_is_rejected() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = facet_fixture()?;
    let facet = Facet::new(
        fixture.facet,
        fixture.schema,
        fixture.context,
        FacetTarget::Context(fixture.context),
        FacetValue::Text(String::from("editor")),
    );

    // Act
    let result = fixture.runtime.handle(KernelEvent::attach_facet(facet));

    // Assert
    assert_eq!(
        result,
        Err(KernelError::FacetTargetMismatch {
            expected: GraphEntityKind::ComponentInstance,
            actual: GraphEntityKind::Context,
        })
    );
    Ok(())
}

#[test]
fn unknown_facet_schema_is_rejected() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = facet_fixture()?;
    let unknown_schema = FacetSchemaId::new(9_999);
    let facet = Facet::new(
        fixture.facet,
        unknown_schema,
        fixture.context,
        FacetTarget::ComponentInstance(fixture.instance),
        FacetValue::Text(String::from("editor")),
    );

    // Act
    let result = fixture.runtime.handle(KernelEvent::attach_facet(facet));

    // Assert
    assert_eq!(result, Err(KernelError::UnknownFacetSchema(unknown_schema)));
    Ok(())
}

#[test]
fn unknown_eligible_facet_target_is_rejected() -> Result<(), KernelError> {
    // Arrange
    let mut fixture = facet_fixture()?;
    let facet = Facet::new(
        fixture.facet,
        fixture.schema,
        fixture.context,
        FacetTarget::ComponentInstance(ComponentInstanceId::new(9_999)),
        FacetValue::Text(String::from("editor")),
    );

    // Act
    let result = fixture.runtime.handle(KernelEvent::attach_facet(facet));

    // Assert
    assert_eq!(result, Err(KernelError::UnknownFacetTarget));
    Ok(())
}

#[test]
fn component_owned_context_and_facets_follow_owner_lifecycle() -> Result<(), KernelError> {
    // Arrange
    let addon = AddonId::new(2);
    let instance = ComponentInstanceId::new(20);
    let context = ContextId::new(200);
    let schema = FacetSchemaId::new(2_000);
    let facet = FacetId::new(20_000);
    let mut runtime = KernelRuntime::new();
    let definition = ComponentDefinition::new(ComponentDefinitionId::new(2));
    let _component = runtime.handle(KernelEvent::register_component(definition, instance))?;
    let context_definition = Context::new(
        context,
        ContextOwner::Component(instance),
        ContextVisibility::Owner,
    );
    let schema_definition = FacetSchema::new(
        schema,
        addon,
        FacetValueKind::Boolean,
        GraphEntityKind::Context,
    );
    let facet_definition = Facet::new(
        facet,
        schema,
        context,
        FacetTarget::Context(context),
        FacetValue::Boolean(true),
    );
    let _context = runtime.handle(KernelEvent::register_context(context_definition))?;
    let _schema = runtime.handle(KernelEvent::register_facet_schema(schema_definition))?;
    let _facet = runtime.handle(KernelEvent::attach_facet(facet_definition))?;

    // Act
    let _removal = runtime.handle(KernelEvent::unregister_component(instance))?;

    // Assert
    assert!(runtime.graph().context(context).is_none());
    assert!(runtime.graph().facet(facet).is_none());
    Ok(())
}

/// Runtime and identities shared by independent Facet validation scenarios.
struct FacetFixture {
    /// Isolated Kernel Runtime under test.
    runtime: KernelRuntime,
    /// System Addon that owns the Facet Schema and Context.
    addon: AddonId,
    /// Active Component Instance eligible for attachment.
    instance: ComponentInstanceId,
    /// Structural Context containing the Facet.
    context: ContextId,
    /// Typed Facet Schema registered by the Addon.
    schema: FacetSchemaId,
    /// Facet identity reserved for each scenario.
    facet: FacetId,
}

/// Registers one Addon-owned Context, Schema, and eligible Component Instance.
fn facet_fixture() -> Result<FacetFixture, KernelError> {
    let addon = AddonId::new(1);
    let instance = ComponentInstanceId::new(10);
    let context = ContextId::new(100);
    let schema = FacetSchemaId::new(1_000);
    let mut runtime = KernelRuntime::new();
    let definition = ComponentDefinition::new(ComponentDefinitionId::new(1));
    let _component = runtime.handle(KernelEvent::register_component(definition, instance))?;
    let context_definition = Context::new(
        context,
        ContextOwner::Addon(addon),
        ContextVisibility::Descendants,
    );
    let schema_definition = FacetSchema::new(
        schema,
        addon,
        FacetValueKind::Text,
        GraphEntityKind::ComponentInstance,
    );
    let _context = runtime.handle(KernelEvent::register_context(context_definition))?;
    let _schema = runtime.handle(KernelEvent::register_facet_schema(schema_definition))?;
    Ok(FacetFixture {
        runtime,
        addon,
        instance,
        context,
        schema,
        facet: FacetId::new(10_000),
    })
}
