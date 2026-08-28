//! Public typed-Event seam of the ordered Loader state machine.

use std::collections::BTreeMap;

use crate::{
    ComponentMaterializer, EventLoopDriver, KernelEvent, LoadId, LoadPhase, LoadRecord,
    LoadRequest, LoaderError, LoaderEvent, LoaderOutcome,
};

/// Ordered Loader machine backed by one deterministic materializer adapter.
#[derive(Debug)]
pub struct Loader<Materializer> {
    /// Bootstrap adapter kept outside Component Capabilities.
    materializer: Materializer,
    /// Independent inspectable Loader lifecycles.
    loads: BTreeMap<LoadId, LoadRecord>,
}

impl<Materializer: ComponentMaterializer> Loader<Materializer> {
    /// Creates an empty Loader using the supplied bootstrap adapter.
    #[must_use]
    pub const fn new(materializer: Materializer) -> Self {
        Self {
            materializer,
            loads: BTreeMap::new(),
        }
    }

    /// Interprets one typed Loader Event against its Current State.
    ///
    /// # Errors
    ///
    /// Returns [`LoaderError`] when the Event reorders a required phase or an
    /// adapter or Kernel Runtime transition fails.
    pub fn handle<Driver: EventLoopDriver>(
        &mut self,
        event: LoaderEvent,
        runtime: &mut crate::KernelRuntime<Driver>,
    ) -> Result<LoaderOutcome, LoaderError> {
        match event {
            LoaderEvent::Declare(request) => self.declare(&request),
            LoaderEvent::Locate(id) => self.locate(id),
            LoaderEvent::Materialize(id) => self.materialize(id),
            LoaderEvent::Inspect(id) => self.inspect(id),
            LoaderEvent::Admit(id) => self.admit(id),
            LoaderEvent::Reject { id, reason } => self.reject(id, reason),
            LoaderEvent::Register(id) => self.register(id, runtime),
            LoaderEvent::MarkReady(id) => self.mark_ready(id),
        }
    }

    /// Creates the sole initial state for a new lifecycle identity.
    fn declare(&mut self, request: &LoadRequest) -> Result<LoaderOutcome, LoaderError> {
        let id = request.id();
        if self.loads.contains_key(&id) {
            return Err(LoaderError::DuplicateLoad(id));
        }
        self.loads.insert(id, LoadRecord::declared(request));
        let transition = crate::LoadTransition::new(id, None, LoadPhase::Declared);
        Ok(LoaderOutcome::transitioned(transition))
    }

    /// Resolves the opaque source before publishing `Located`.
    fn locate(&mut self, id: LoadId) -> Result<LoaderOutcome, LoaderError> {
        let record = self.loads.get(&id).ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Declared)?;
        self.materializer
            .locate(id, record.source())
            .map_err(|error| LoaderError::Materializer {
                phase: LoadPhase::Located,
                error,
            })?;
        let transition = self
            .loads
            .get_mut(&id)
            .expect("validated load exists")
            .advance(LoadPhase::Located);
        Ok(LoaderOutcome::transitioned(transition))
    }

    /// Materializes support before publishing `Materialized`.
    fn materialize(&mut self, id: LoadId) -> Result<LoaderOutcome, LoaderError> {
        let record = self.loads.get(&id).ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Located)?;
        self.materializer
            .materialize(id)
            .map_err(|error| LoaderError::Materializer {
                phase: LoadPhase::Materialized,
                error,
            })?;
        let transition = self
            .loads
            .get_mut(&id)
            .expect("validated load exists")
            .advance(LoadPhase::Materialized);
        Ok(LoaderOutcome::transitioned(transition))
    }

    /// Stores one complete Definition before publishing `Inspected`.
    fn inspect(&mut self, id: LoadId) -> Result<LoaderOutcome, LoaderError> {
        self.loads
            .get(&id)
            .ok_or(LoaderError::UnknownLoad(id))?
            .require(LoadPhase::Materialized)?;
        let definition =
            self.materializer
                .inspect(id)
                .map_err(|error| LoaderError::Materializer {
                    phase: LoadPhase::Inspected,
                    error,
                })?;
        let record = self.loads.get_mut(&id).expect("validated load exists");
        record.set_definition(definition);
        Ok(LoaderOutcome::transitioned(
            record.advance(LoadPhase::Inspected),
        ))
    }

    /// Admits the inspected Definition without skipping inspection.
    fn admit(&mut self, id: LoadId) -> Result<LoaderOutcome, LoaderError> {
        let record = self
            .loads
            .get_mut(&id)
            .ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Inspected)?;
        Ok(LoaderOutcome::transitioned(
            record.advance(LoadPhase::Admitted),
        ))
    }

    /// Terminates the lifecycle with an inspectable rejection reason.
    fn reject(&mut self, id: LoadId, reason: String) -> Result<LoaderOutcome, LoaderError> {
        let record = self
            .loads
            .get_mut(&id)
            .ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Inspected)?;
        record.set_rejection(reason);
        Ok(LoaderOutcome::transitioned(
            record.advance(LoadPhase::Rejected),
        ))
    }

    /// Registers the complete Definition before publishing `Registered`.
    fn register<Driver: EventLoopDriver>(
        &mut self,
        id: LoadId,
        runtime: &mut crate::KernelRuntime<Driver>,
    ) -> Result<LoaderOutcome, LoaderError> {
        let record = self.loads.get(&id).ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Admitted)?;
        let definition = record
            .definition()
            .cloned()
            .ok_or(LoaderError::MissingInspectedDefinition(id))?;
        let _ = runtime
            .handle(KernelEvent::register_component(
                definition,
                record.instance(),
            ))
            .map_err(LoaderError::Kernel)?;
        let transition = self
            .loads
            .get_mut(&id)
            .expect("validated load exists")
            .advance(LoadPhase::Registered);
        Ok(LoaderOutcome::transitioned(transition))
    }

    /// Publishes readiness only after successful Kernel registration.
    fn mark_ready(&mut self, id: LoadId) -> Result<LoaderOutcome, LoaderError> {
        let record = self
            .loads
            .get_mut(&id)
            .ok_or(LoaderError::UnknownLoad(id))?;
        record.require(LoadPhase::Registered)?;
        Ok(LoaderOutcome::transitioned(
            record.advance(LoadPhase::Ready),
        ))
    }

    /// Finds one inspectable Loader lifecycle by identity.
    #[must_use]
    pub fn load(&self, id: LoadId) -> Option<&LoadRecord> {
        self.loads.get(&id)
    }
}
