//! Validation and storage for Contexts, Facet Schemas, and Facets.

use crate::{
    Context, ContextOwner, Facet, FacetSchema, FacetTarget, KernelError, graph::GraphState,
};

impl GraphState {
    /// Registers a structural Context after validating its owner and parent.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] when the Context identity exists or an explicit
    /// parent or Component owner is unknown.
    pub(super) fn register_context(&mut self, context: Context) -> Result<(), KernelError> {
        if self.contexts.contains_key(&context.id()) {
            return Err(KernelError::DuplicateContext(context.id()));
        }
        if let Some(parent) = context.parent()
            && !self.contexts.contains_key(&parent)
        {
            return Err(KernelError::UnknownContext(parent));
        }
        if let ContextOwner::Component(owner_id) = context.owner()
            && !self.instances.contains_key(&owner_id)
        {
            return Err(KernelError::UnknownComponentInstance(owner_id));
        }
        self.contexts.insert(context.id(), context);
        Ok(())
    }

    /// Registers one Addon-owned Facet Schema.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError::DuplicateFacetSchema`] when its identity exists.
    pub(super) fn register_facet_schema(&mut self, schema: FacetSchema) -> Result<(), KernelError> {
        if self.facet_schemas.contains_key(&schema.id()) {
            return Err(KernelError::DuplicateFacetSchema(schema.id()));
        }
        self.facet_schemas.insert(schema.id(), schema);
        Ok(())
    }

    /// Validates and attaches one typed Facet to an existing eligible entity.
    ///
    /// # Errors
    ///
    /// Returns [`KernelError`] for duplicate or unknown identities and when the
    /// Facet value or target category differs from its Schema.
    pub(super) fn attach_facet(&mut self, facet: Facet) -> Result<(), KernelError> {
        if self.facets.contains_key(&facet.id()) {
            return Err(KernelError::DuplicateFacet(facet.id()));
        }
        let schema_id = facet.schema();
        let schema = self
            .facet_schemas
            .get(&schema_id)
            .ok_or(KernelError::UnknownFacetSchema(schema_id))?;
        if !self.contexts.contains_key(&facet.context()) {
            return Err(KernelError::UnknownContext(facet.context()));
        }
        if schema.value_kind() != facet.value().kind() {
            return Err(KernelError::FacetValueMismatch {
                expected: schema.value_kind(),
                actual: facet.value().kind(),
            });
        }
        if schema.target_kind() != facet.target().kind() {
            return Err(KernelError::FacetTargetMismatch {
                expected: schema.target_kind(),
                actual: facet.target().kind(),
            });
        }
        if !self.facet_target_exists(facet.target()) {
            return Err(KernelError::UnknownFacetTarget);
        }
        self.facets.insert(facet.id(), facet);
        Ok(())
    }

    /// Reports whether a strongly typed Facet target exists in this graph.
    fn facet_target_exists(&self, target: FacetTarget) -> bool {
        match target {
            FacetTarget::ComponentDefinition(id) => self.definitions.contains_key(&id),
            FacetTarget::ComponentInstance(id) => self.instances.contains_key(&id),
            FacetTarget::Requirement(id) => self.requirements.contains_key(&id),
            FacetTarget::Capability(id) => self.capabilities.contains_key(&id),
            FacetTarget::ComponentRuntime(id) => {
                self.runtimes.values().any(|runtime| runtime.id() == id)
            }
            FacetTarget::Effect(id) => self.effects.contains_key(&id),
            FacetTarget::Context(id) => self.contexts.contains_key(&id),
        }
    }
}
