//! Matchable failures produced by the Loader seam.

use crate::{KernelError, LoadId, LoadPhase, MaterializerError};
use thiserror::Error;

/// Failure that prevents one Loader Event transition.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LoaderError {
    /// A declaration reused an existing lifecycle identity.
    #[error("Loader lifecycle {0:?} already exists")]
    DuplicateLoad(LoadId),
    /// An Event referenced an unknown Loader lifecycle.
    #[error("Loader lifecycle {0:?} does not exist")]
    UnknownLoad(LoadId),
    /// An Event attempted to bypass or reorder required Loader phases.
    #[error("invalid Loader transition from {actual:?}; expected {expected:?}")]
    InvalidTransition {
        /// Phase required by the typed Event.
        expected: LoadPhase,
        /// Phase currently observed in the Loader.
        actual: LoadPhase,
    },
    /// The configured materializer adapter rejected a bootstrap operation.
    #[error("materializer failed during {phase:?}")]
    Materializer {
        /// Phase that could not complete.
        phase: LoadPhase,
        /// Adapter-owned failure.
        #[source]
        error: MaterializerError,
    },
    /// Registration was requested without the Definition promised by inspection.
    #[error("Loader lifecycle {0:?} has no inspected Component Definition")]
    MissingInspectedDefinition(LoadId),
    /// Kernel Runtime registration rejected the complete Component Definition.
    #[error("Kernel Runtime rejected Loader registration")]
    Kernel(#[source] KernelError),
}
