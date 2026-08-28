//! Trusted in-process dynamic-library adapter for the ordered Loader phases.

use std::{collections::BTreeMap, path::PathBuf};

use libloading::Library;

use crate::{
    Capability, CapabilityContractId, CapabilityId, ComponentDefinition, ComponentDefinitionId,
    ComponentMaterializer, ComponentSource, LoadId, MaterializerError,
    NATIVE_COMPONENT_ABI_VERSION, NATIVE_COMPONENT_ENTRY_POINT, NativeCapabilityDescriptor,
    NativeComponentDescriptor, NativeRequirementDescriptor, Requirement, RequirementId,
};

/// Exact trusted native entry-point signature published by ABI v1.
type NativeEntryPoint = unsafe extern "C" fn() -> NativeComponentDescriptor;

/// Native bootstrap adapter that owns every loaded dynamic-library handle.
#[derive(Debug, Default)]
pub struct NativeMaterializer {
    /// Canonical bootstrap paths indexed by Loader lifecycle.
    locations: BTreeMap<LoadId, PathBuf>,
    /// Native handles retained beyond inspection and Runtime registration.
    libraries: BTreeMap<LoadId, Library>,
}

impl NativeMaterializer {
    /// Creates an empty trusted native bootstrap adapter.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            locations: BTreeMap::new(),
            libraries: BTreeMap::new(),
        }
    }

    /// Resolves and calls the exact ABI v1 entry point while its handle lives.
    fn load_descriptor(library: &Library) -> Result<NativeComponentDescriptor, MaterializerError> {
        // SAFETY: the trusted library must export the exact versioned C symbol
        // and signature documented by Native Component ABI v1. `Symbol` stays
        // borrowed from `library`, which is retained by this materializer.
        let entry = unsafe {
            library
                .get::<NativeEntryPoint>(NATIVE_COMPONENT_ENTRY_POINT)
                .map_err(|error| MaterializerError::new(error.to_string()))?
        };
        // SAFETY: the integrator guarantees that this trusted entry point does
        // not unwind and returns a descriptor satisfying the published ABI.
        Ok(unsafe { entry() })
    }

    /// Validates and copies a transient ABI descriptor into the Rust model.
    fn complete_definition(
        descriptor: NativeComponentDescriptor,
    ) -> Result<ComponentDefinition, MaterializerError> {
        if descriptor.abi_version != NATIVE_COMPONENT_ABI_VERSION || descriptor.reserved != 0 {
            return Err(MaterializerError::new(
                "native Component uses an unsupported ABI version",
            ));
        }
        // SAFETY: ABI v1 requires both arrays to be immutable, aligned, and
        // readable for their announced lengths while the library handle lives.
        let requirements = unsafe {
            Self::copy_descriptor_slice(descriptor.requirements, descriptor.requirements_len)?
        };
        // SAFETY: the same ABI v1 lifetime and validity contract applies to the
        // Capability array; it is copied before the handle can be released.
        let capabilities = unsafe {
            Self::copy_descriptor_slice(descriptor.capabilities, descriptor.capabilities_len)?
        };
        Ok(Self::assemble_definition(
            descriptor.definition_id,
            &requirements,
            &capabilities,
        ))
    }

    /// Copies one trusted descriptor view after checking cheap structural facts.
    ///
    /// # Safety
    ///
    /// For non-zero `length`, `pointer` must reference an immutable, aligned,
    /// readable array of `length` initialized `Item` values for this call.
    unsafe fn copy_descriptor_slice<Item: Copy>(
        pointer: *const Item,
        length: usize,
    ) -> Result<Vec<Item>, MaterializerError> {
        if length == 0 {
            return Ok(Vec::new());
        }
        let byte_length = length
            .checked_mul(std::mem::size_of::<Item>())
            .filter(|bytes| *bytes <= isize::MAX.cast_unsigned())
            .ok_or_else(|| MaterializerError::new("native descriptor array is too large"))?;
        if pointer.is_null()
            || !(pointer as usize).is_multiple_of(std::mem::align_of::<Item>())
            || byte_length == 0
        {
            return Err(MaterializerError::new(
                "native descriptor array pointer is invalid",
            ));
        }
        // SAFETY: the caller provides the documented ABI validity contract;
        // bounds, null, alignment, and representable byte length were checked.
        Ok(unsafe { std::slice::from_raw_parts(pointer, length) }.to_vec())
    }

    /// Maps copied ABI contributions into one complete Component Definition.
    fn assemble_definition(
        definition_id: u64,
        requirements: &[NativeRequirementDescriptor],
        capabilities: &[NativeCapabilityDescriptor],
    ) -> ComponentDefinition {
        let mut definition = ComponentDefinition::new(ComponentDefinitionId::new(definition_id));
        for requirement in requirements {
            definition = definition.with_requirement(Requirement::necessary(
                RequirementId::new(requirement.id),
                CapabilityContractId::new(requirement.contract),
            ));
        }
        for capability in capabilities {
            definition = definition.with_capability(Capability::new(
                CapabilityId::new(capability.id),
                CapabilityContractId::new(capability.contract),
            ));
        }
        definition
    }
}

impl ComponentMaterializer for NativeMaterializer {
    fn locate(&mut self, id: LoadId, source: &ComponentSource) -> Result<(), MaterializerError> {
        let path = PathBuf::from(source.as_str())
            .canonicalize()
            .map_err(|error| MaterializerError::new(error.to_string()))?;
        if !path.is_file() {
            return Err(MaterializerError::new(
                "native Component source is not a readable file",
            ));
        }
        self.locations.insert(id, path);
        Ok(())
    }

    fn materialize(&mut self, id: LoadId) -> Result<(), MaterializerError> {
        let path = self
            .locations
            .get(&id)
            .ok_or_else(|| MaterializerError::new("native Component source is not located"))?;
        // SAFETY: native bootstrap accepts only an integrator-designated trusted
        // library whose initialization and termination routines are sound in
        // this process. The canonical path avoids ambient name lookup.
        let library = unsafe { Library::new(path) }
            .map_err(|error| MaterializerError::new(error.to_string()))?;
        self.libraries.insert(id, library);
        Ok(())
    }

    fn inspect(&mut self, id: LoadId) -> Result<ComponentDefinition, MaterializerError> {
        let library = self
            .libraries
            .get(&id)
            .ok_or_else(|| MaterializerError::new("native Component is not materialized"))?;
        Self::complete_definition(Self::load_descriptor(library)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Exercises ABI pointer validation and copying without unsupported `dlopen`.
    #[test]
    fn static_descriptor_is_copied_into_complete_definition() {
        static REQUIREMENTS: [NativeRequirementDescriptor; 1] = [NativeRequirementDescriptor {
            id: 1,
            contract: 10,
        }];
        static CAPABILITIES: [NativeCapabilityDescriptor; 1] = [NativeCapabilityDescriptor {
            id: 2,
            contract: 20,
        }];
        let descriptor = NativeComponentDescriptor {
            abi_version: NATIVE_COMPONENT_ABI_VERSION,
            reserved: 0,
            definition_id: 3,
            requirements: REQUIREMENTS.as_ptr(),
            requirements_len: REQUIREMENTS.len(),
            capabilities: CAPABILITIES.as_ptr(),
            capabilities_len: CAPABILITIES.len(),
        };

        let definition = NativeMaterializer::complete_definition(descriptor)
            .expect("valid static descriptor is copied");

        assert_eq!(definition.id(), ComponentDefinitionId::new(3));
        assert_eq!(definition.requirements()[0].id(), RequirementId::new(1));
        assert_eq!(definition.capabilities()[0].id(), CapabilityId::new(2));
    }
}
