use super::ScryptoRuntime;
use crate::blueprints::access_controller::AccessControllerNativePackage;
use crate::blueprints::account::AccountNativePackage;
use crate::blueprints::clock::ClockNativePackage;
use crate::blueprints::epoch_manager::EpochManagerNativePackage;
use crate::blueprints::identity::IdentityNativePackage;
use crate::blueprints::package::{PackageCodeTypeSubstate, PackageNativePackage};
use crate::blueprints::resource::ResourceManagerNativePackage;
use crate::blueprints::transaction_processor::TransactionProcessorNativePackage;
use crate::errors::{InterpreterError, RuntimeError};
use crate::kernel::actor::Actor;
use crate::kernel::call_frame::CallFrameUpdate;
use crate::kernel::executor::*;
use crate::kernel::kernel_api::{KernelNodeApi, KernelSubstateApi, KernelWasmApi};
use crate::system::node_modules::access_rules::{AccessRulesNativePackage, AuthZoneNativePackage};
use crate::system::node_modules::metadata::MetadataNativePackage;
use crate::system::node_modules::royalty::RoyaltyNativePackage;
use crate::system::node_modules::type_info::{TypeInfoBlueprint, TypeInfoSubstate};
use crate::types::*;
use crate::wasm::{WasmEngine, WasmInstance, WasmInstrumenter, WasmMeteringConfig, WasmRuntime};
use radix_engine_interface::api::node_modules::auth::ACCESS_RULES_BLUEPRINT;
use radix_engine_interface::api::node_modules::metadata::METADATA_BLUEPRINT;
use radix_engine_interface::api::node_modules::royalty::COMPONENT_ROYALTY_BLUEPRINT;
use radix_engine_interface::api::substate_api::LockFlags;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::schema::BlueprintSchema;

fn validate_input(
    blueprint_schema: &BlueprintSchema,
    fn_ident: &str,
    with_receiver: bool,
    input: &IndexedScryptoValue,
) -> Result<String, RuntimeError> {
    let function_schema =
        blueprint_schema
            .functions
            .get(fn_ident)
            .ok_or(RuntimeError::InterpreterError(
                InterpreterError::ScryptoFunctionNotFound(fn_ident.to_string()),
            ))?;

    if function_schema.receiver.is_some() != with_receiver {
        return Err(RuntimeError::InterpreterError(
            InterpreterError::ScryptoReceiverNotMatch(fn_ident.to_string()),
        ));
    }

    validate_payload_against_schema(
        input.as_slice(),
        &blueprint_schema.schema,
        function_schema.input,
    )
    .map_err(|err| {
        RuntimeError::InterpreterError(InterpreterError::ScryptoInputSchemaNotMatch(
            fn_ident.to_string(),
            err.error_message(&blueprint_schema.schema),
        ))
    })?;

    Ok(function_schema.export_name.clone())
}

fn validate_output(
    blueprint_schema: &BlueprintSchema,
    fn_ident: &str,
    output: Vec<u8>,
) -> Result<IndexedScryptoValue, RuntimeError> {
    let value = IndexedScryptoValue::from_vec(output).map_err(|e| {
        RuntimeError::InterpreterError(InterpreterError::ScryptoOutputDecodeError(e))
    })?;

    let function_schema = blueprint_schema
        .functions
        .get(fn_ident)
        .expect("Checked by `validate_input`");

    validate_payload_against_schema(
        value.as_slice(),
        &blueprint_schema.schema,
        function_schema.output,
    )
    .map_err(|err| {
        RuntimeError::InterpreterError(InterpreterError::ScryptoOutputSchemaNotMatch(
            fn_ident.to_string(),
            err.error_message(&blueprint_schema.schema),
        ))
    })?;

    Ok(value)
}

impl ExecutableInvocation for MethodInvocation {
    type Exec = ScryptoExecutor;

    fn resolve<D: KernelSubstateApi>(
        self,
        api: &mut D,
    ) -> Result<ResolvedInvocation<Self::Exec>, RuntimeError> {
        let value = IndexedScryptoValue::from_vec(self.args).map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;
        let nodes_to_move = value.owned_node_ids().clone();
        let mut node_refs_to_copy = value.references().clone();

        // Pass the component ref
        node_refs_to_copy.insert(self.identifier.0);

        let (package_address, blueprint_name) = match self.identifier.1 {
            NodeModuleId::SELF => {
                let type_info = TypeInfoBlueprint::get_type(self.identifier.0, api)?;
                match type_info {
                    TypeInfoSubstate::Object {
                        package_address,
                        blueprint_name,
                        ..
                    } => (package_address, blueprint_name),

                    TypeInfoSubstate::KeyValueStore(..) => {
                        return Err(RuntimeError::InterpreterError(
                            InterpreterError::CallMethodOnKeyValueStore,
                        ))
                    }
                }
            }
            NodeModuleId::Metadata => {
                // TODO: Check if type has metadata
                (METADATA_PACKAGE, METADATA_BLUEPRINT.to_string())
            }
            NodeModuleId::ComponentRoyalty => {
                // TODO: Check if type has royalty
                (ROYALTY_PACKAGE, COMPONENT_ROYALTY_BLUEPRINT.to_string())
            }
            NodeModuleId::AccessRules | NodeModuleId::AccessRules1 => {
                // TODO: Check if type has access rules
                (ACCESS_RULES_PACKAGE, ACCESS_RULES_BLUEPRINT.to_string())
            }
            _ => todo!(),
        };
        let fn_identifier = FnIdentifier::new(
            package_address,
            blueprint_name.clone(),
            self.identifier.2.clone(),
        );
        let actor = Actor::method(fn_identifier.clone(), self.identifier.clone());

        // TODO: Remove this weirdness or move to a kernel module if we still want to support this
        {
            if package_address.eq(&PACKAGE_PACKAGE) {
                node_refs_to_copy.insert(RENodeId::GlobalObject(RADIX_TOKEN.into()));
            } else {
                let handle = api.kernel_lock_substate(
                    RENodeId::GlobalObject(fn_identifier.package_address.into()),
                    NodeModuleId::SELF,
                    SubstateOffset::Package(PackageOffset::CodeType),
                    LockFlags::read_only(),
                )?;
                let code_type: &PackageCodeTypeSubstate = api.kernel_get_substate_ref(handle)?;
                let code_type = code_type.clone();
                api.kernel_drop_lock(handle)?;

                match code_type {
                    PackageCodeTypeSubstate::Wasm => {
                        node_refs_to_copy.insert(RENodeId::GlobalObject(EPOCH_MANAGER.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(CLOCK.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(RADIX_TOKEN.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(PACKAGE_TOKEN.into()));
                        node_refs_to_copy
                            .insert(RENodeId::GlobalObject(ECDSA_SECP256K1_TOKEN.into()));
                        node_refs_to_copy
                            .insert(RENodeId::GlobalObject(EDDSA_ED25519_TOKEN.into()));
                    }
                    _ => {}
                }
            }

            // TODO: remove? currently needed for `Runtime::package_address()` API.
            node_refs_to_copy.insert(RENodeId::GlobalObject(package_address.into()));
        }

        let executor = ScryptoExecutor {
            fn_identifier,
            receiver: Some(self.identifier),
        };

        let resolved = ResolvedInvocation {
            resolved_actor: actor,
            update: CallFrameUpdate {
                nodes_to_move,
                node_refs_to_copy,
            },
            executor,
            args: value,
        };

        Ok(resolved)
    }

    fn payload_size(&self) -> usize {
        self.args.len() + self.identifier.2.len()
    }
}

impl ExecutableInvocation for FunctionInvocation {
    type Exec = ScryptoExecutor;

    fn resolve<D: KernelSubstateApi>(
        self,
        api: &mut D,
    ) -> Result<ResolvedInvocation<Self::Exec>, RuntimeError> {
        let value = IndexedScryptoValue::from_vec(self.args).map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;
        let nodes_to_move = value.owned_node_ids().clone();
        let mut node_refs_to_copy = value.references().clone();

        let actor = Actor::function(self.fn_identifier.clone());

        // TODO: Remove this weirdness or move to a kernel module if we still want to support this
        {
            if self.fn_identifier.package_address.eq(&PACKAGE_PACKAGE) {
                node_refs_to_copy.insert(RENodeId::GlobalObject(RADIX_TOKEN.into()));
            } else if self
                .fn_identifier
                .package_address
                .eq(&TRANSACTION_PROCESSOR_PACKAGE)
            {
                // Required for bootstrap.
                // Can be removed once the auto reference copying logic is moved to a kernel module.
                // Will just disable the module for genesis.
            } else {
                let handle = api.kernel_lock_substate(
                    RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                    NodeModuleId::SELF,
                    SubstateOffset::Package(PackageOffset::CodeType),
                    LockFlags::read_only(),
                )?;
                let code_type: &PackageCodeTypeSubstate = api.kernel_get_substate_ref(handle)?;
                let code_type = code_type.clone();
                api.kernel_drop_lock(handle)?;

                match code_type {
                    PackageCodeTypeSubstate::Wasm => {
                        node_refs_to_copy.insert(RENodeId::GlobalObject(EPOCH_MANAGER.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(CLOCK.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(RADIX_TOKEN.into()));
                        node_refs_to_copy.insert(RENodeId::GlobalObject(PACKAGE_TOKEN.into()));
                        node_refs_to_copy
                            .insert(RENodeId::GlobalObject(ECDSA_SECP256K1_TOKEN.into()));
                        node_refs_to_copy
                            .insert(RENodeId::GlobalObject(EDDSA_ED25519_TOKEN.into()));
                    }
                    _ => {}
                }
            }

            // TODO: remove? currently needed for `Runtime::package_address()` API.
            node_refs_to_copy.insert(RENodeId::GlobalObject(
                self.fn_identifier.package_address.into(),
            ));
        }

        let resolved = ResolvedInvocation {
            resolved_actor: actor,
            update: CallFrameUpdate {
                nodes_to_move,
                node_refs_to_copy,
            },
            args: value,
            executor: ScryptoExecutor {
                fn_identifier: self.fn_identifier,
                receiver: None,
            },
        };

        Ok(resolved)
    }

    fn payload_size(&self) -> usize {
        self.args.len() + self.fn_identifier.size()
    }
}

pub struct ScryptoExecutor {
    pub fn_identifier: FnIdentifier,
    pub receiver: Option<MethodIdentifier>,
}

impl Executor for ScryptoExecutor {
    type Output = IndexedScryptoValue;

    fn execute<Y, W>(
        self,
        args: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<(IndexedScryptoValue, CallFrameUpdate), RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + KernelWasmApi<W> + ClientApi<RuntimeError>,
        W: WasmEngine,
    {
        let output = if self.fn_identifier.package_address.eq(&PACKAGE_PACKAGE) {
            // TODO: Clean this up
            // Do we need to check against the abi? Probably not since we should be able to verify this
            // in the native package itself.
            let export_name = self.fn_identifier.ident.to_string(); // TODO: Clean this up

            NativeVm::invoke_native_package(
                PACKAGE_CODE_ID,
                self.receiver,
                &export_name,
                args,
                api,
            )?
        } else if self
            .fn_identifier
            .package_address
            .eq(&TRANSACTION_PROCESSOR_PACKAGE)
        {
            // TODO: the above special rule can be removed if we move schema validation
            // into a kernel model, and turn it off for genesis.

            let export_name = self.fn_identifier.ident.to_string();

            NativeVm::invoke_native_package(
                TRANSACTION_PROCESSOR_CODE_ID,
                self.receiver,
                &export_name,
                args,
                api,
            )?
        } else {
            // Make dependent resources/components visible
            let handle = api.kernel_lock_substate(
                RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                NodeModuleId::SELF,
                SubstateOffset::Package(PackageOffset::Info),
                LockFlags::read_only(),
            )?;
            api.kernel_drop_lock(handle)?;

            // Load schema
            let schema = {
                let handle = api.kernel_lock_substate(
                    RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                    NodeModuleId::SELF,
                    SubstateOffset::Package(PackageOffset::Info),
                    LockFlags::read_only(),
                )?;
                let package_info: &PackageInfoSubstate = api.kernel_get_substate_ref(handle)?;
                let schema = package_info
                    .schema
                    .blueprints
                    .get(&self.fn_identifier.blueprint_name)
                    .ok_or(RuntimeError::InterpreterError(
                        InterpreterError::ScryptoBlueprintNotFound(
                            self.fn_identifier.package_address,
                            self.fn_identifier.blueprint_name.clone(),
                        ),
                    ))?
                    .clone();
                api.kernel_drop_lock(handle)?;
                schema
            };

            //  Validate input
            let export_name = validate_input(
                &schema,
                &self.fn_identifier.ident,
                self.receiver.is_some(),
                &args,
            )?;

            // Interpret
            let code_type = {
                let handle = api.kernel_lock_substate(
                    RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                    NodeModuleId::SELF,
                    SubstateOffset::Package(PackageOffset::CodeType),
                    LockFlags::read_only(),
                )?;
                let code_type: &PackageCodeTypeSubstate = api.kernel_get_substate_ref(handle)?;
                let code_type = code_type.clone();
                api.kernel_drop_lock(handle)?;
                code_type
            };
            let output = match code_type {
                PackageCodeTypeSubstate::Native => {
                    let handle = api.kernel_lock_substate(
                        RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                        NodeModuleId::SELF,
                        SubstateOffset::Package(PackageOffset::Code),
                        LockFlags::read_only(),
                    )?;
                    let code: &PackageCodeSubstate = api.kernel_get_substate_ref(handle)?;
                    let native_package_code_id = code.code[0];
                    api.kernel_drop_lock(handle)?;

                    NativeVm::invoke_native_package(
                        native_package_code_id,
                        self.receiver,
                        &export_name,
                        args,
                        api,
                    )?
                    .into()
                }
                PackageCodeTypeSubstate::Wasm => {
                    let mut wasm_instance = {
                        let handle = api.kernel_lock_substate(
                            RENodeId::GlobalObject(self.fn_identifier.package_address.into()),
                            NodeModuleId::SELF,
                            SubstateOffset::Package(PackageOffset::Code),
                            LockFlags::read_only(),
                        )?;
                        let wasm_instance = api.kernel_create_wasm_instance(
                            self.fn_identifier.package_address,
                            handle,
                        )?;
                        api.kernel_drop_lock(handle)?;

                        wasm_instance
                    };

                    let output = {
                        let mut runtime: Box<dyn WasmRuntime> = Box::new(ScryptoRuntime::new(api));

                        let mut input = Vec::new();
                        if let Some(MethodIdentifier(node_id, ..)) = self.receiver {
                            input.push(
                                runtime
                                    .allocate_buffer(
                                        scrypto_encode(&node_id)
                                            .expect("Failed to encode component id"),
                                    )
                                    .expect("Failed to allocate buffer"),
                            );
                        }
                        input.push(
                            runtime
                                .allocate_buffer(args.into())
                                .expect("Failed to allocate buffer"),
                        );

                        wasm_instance.invoke_export(&export_name, input, &mut runtime)?
                    };

                    api.update_wasm_memory_usage(wasm_instance.consumed_memory()?)?;

                    output
                }
            };

            // Validate output
            validate_output(&schema, &self.fn_identifier.ident, output)?
        };

        let update = CallFrameUpdate {
            node_refs_to_copy: output.references().clone(),
            nodes_to_move: output.owned_node_ids().clone(),
        };

        Ok((output, update))
    }
}

struct NativeVm;

impl NativeVm {
    pub fn invoke_native_package<Y>(
        native_package_code_id: u8,
        receiver: Option<MethodIdentifier>,
        export_name: &str,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let receiver = receiver.map(|r| r.0);

        match native_package_code_id {
            PACKAGE_CODE_ID => {
                PackageNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            RESOURCE_MANAGER_CODE_ID => {
                ResourceManagerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            EPOCH_MANAGER_CODE_ID => {
                EpochManagerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            IDENTITY_CODE_ID => {
                IdentityNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            CLOCK_CODE_ID => ClockNativePackage::invoke_export(&export_name, receiver, input, api),
            ACCOUNT_CODE_ID => {
                AccountNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ACCESS_CONTROLLER_CODE_ID => {
                AccessControllerNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            TRANSACTION_PROCESSOR_CODE_ID => {
                TransactionProcessorNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            AUTH_ZONE_CODE_ID => {
                AuthZoneNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            METADATA_CODE_ID => {
                MetadataNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ROYALTY_CODE_ID => {
                RoyaltyNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            ACCESS_RULES_CODE_ID => {
                AccessRulesNativePackage::invoke_export(&export_name, receiver, input, api)
            }
            _ => Err(RuntimeError::InterpreterError(
                InterpreterError::NativeInvalidCodeId(native_package_code_id),
            )),
        }
    }
}

pub struct ScryptoInterpreter<W: WasmEngine> {
    pub wasm_engine: W,
    /// WASM Instrumenter
    pub wasm_instrumenter: WasmInstrumenter,
    /// WASM metering config
    pub wasm_metering_config: WasmMeteringConfig,
}

impl<W: WasmEngine + Default> Default for ScryptoInterpreter<W> {
    fn default() -> Self {
        Self {
            wasm_engine: W::default(),
            wasm_instrumenter: WasmInstrumenter::default(),
            wasm_metering_config: WasmMeteringConfig::default(),
        }
    }
}

impl<W: WasmEngine> ScryptoInterpreter<W> {
    pub fn create_instance(&self, package_address: PackageAddress, code: &[u8]) -> W::WasmInstance {
        let instrumented_code =
            self.wasm_instrumenter
                .instrument(package_address, code, self.wasm_metering_config);
        self.wasm_engine.instantiate(&instrumented_code)
    }
}

#[cfg(test)]
mod tests {
    const _: () = {
        fn assert_sync<T: Sync>() {}

        fn assert_all() {
            // The ScryptoInterpreter struct captures the code and module template caches.
            // We therefore share a ScryptoInterpreter as a shared cache across Engine runs on the node.
            // This allows EG multiple mempool submission validations via the Core API at the same time
            // This test ensures the requirement for this cache to be Sync isn't broken
            // (At least when we compile with std, as the node does)
            #[cfg(not(feature = "alloc"))]
            assert_sync::<
                crate::kernel::interpreters::ScryptoInterpreter<crate::wasm::DefaultWasmEngine>,
            >();
        }
    };
}
