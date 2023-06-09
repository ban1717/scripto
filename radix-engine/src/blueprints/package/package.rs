use super::PackageCodeTypeSubstate;
use crate::errors::*;
use crate::kernel::kernel_api::{KernelNodeApi, KernelSubstateApi};
use crate::system::kernel_modules::costing::{FIXED_HIGH_FEE, FIXED_MEDIUM_FEE};
use crate::system::node::{RENodeInit, RENodeModuleInit};
use crate::system::node_modules::access_rules::{
    FunctionAccessRulesSubstate, MethodAccessRulesSubstate,
};
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::system::node_substates::RuntimeSubstate;
use crate::types::*;
use crate::wasm::{PrepareError, WasmValidator};
use native_sdk::resource::{ResourceManager, Vault};
use radix_engine_interface::api::component::{
    ComponentRoyaltyAccumulatorSubstate, ComponentRoyaltyConfigSubstate,
};
use radix_engine_interface::api::types::ClientCostingReason;
use radix_engine_interface::api::{ClientApi, LockFlags};
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::blueprints::resource::{require, AccessRule, AccessRulesConfig, FnKey};
use radix_engine_interface::schema::{BlueprintSchema, FunctionSchema, PackageSchema};

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum PackageError {
    InvalidWasm(PrepareError),

    InvalidBlueprintWasm(SchemaValidationError),
    TooManySubstateSchemas,

    FailedToResolveLocalSchema {
        local_type_index: LocalTypeIndex,
    },
    EventNameMismatch {
        expected: String,
        actual: Option<String>,
    },
    InvalidEventSchema,
}

fn validate_package_schema(schema: &PackageSchema) -> Result<(), PackageError> {
    for blueprint in schema.blueprints.values() {
        validate_schema(&blueprint.schema).map_err(|e| PackageError::InvalidBlueprintWasm(e))?;

        if blueprint.substates.len() > 0xff {
            return Err(PackageError::TooManySubstateSchemas);
        }
    }
    Ok(())
}

fn validate_package_event_schema(schema: &PackageSchema) -> Result<(), PackageError> {
    for BlueprintSchema {
        schema,
        event_schema,
        ..
    } in schema.blueprints.values()
    {
        // Package schema validation happens when the package is published. No need to redo
        // it here again.

        for (expected_event_name, local_type_index) in event_schema.iter() {
            // Checking that the event is either a struct or an enum
            let type_kind = schema.resolve_type_kind(*local_type_index).map_or(
                Err(PackageError::FailedToResolveLocalSchema {
                    local_type_index: *local_type_index,
                }),
                Ok,
            )?;
            match type_kind {
                // Structs and Enums are allowed
                TypeKind::Enum { .. } | TypeKind::Tuple { .. } => Ok(()),
                _ => Err(PackageError::InvalidEventSchema),
            }?;

            // Checking that the event name is indeed what the user claims it to be
            let actual_event_name = schema.resolve_type_metadata(*local_type_index).map_or(
                Err(PackageError::FailedToResolveLocalSchema {
                    local_type_index: *local_type_index,
                }),
                |metadata| Ok(metadata.get_name_string()),
            )?;

            if Some(expected_event_name) != actual_event_name.as_ref() {
                Err(PackageError::EventNameMismatch {
                    expected: expected_event_name.to_string(),
                    actual: actual_event_name,
                })?
            }
        }
    }

    Ok(())
}

fn globalize_package<Y>(
    package_address: Option<[u8; 26]>,
    info: PackageInfoSubstate,
    code_type: PackageCodeTypeSubstate,
    code: PackageCodeSubstate,
    royalty: PackageRoyaltySubstate,
    function_access_rules: FunctionAccessRulesSubstate,
    metadata: BTreeMap<String, String>,
    access_rules: AccessRulesConfig,
    api: &mut Y,
) -> Result<IndexedScryptoValue, RuntimeError>
where
    Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
{
    // Use kernel API to commit substates directly.
    // Can't use the ClientApi because of chicken-and-egg issue.

    // Prepare node init.
    let node_init = RENodeInit::GlobalObject(btreemap!(
        SubstateOffset::Package(PackageOffset::Info) => info.into(),
        SubstateOffset::Package(PackageOffset::CodeType) => code_type.into(),
        SubstateOffset::Package(PackageOffset::Code) => code.into(),
        SubstateOffset::Package(PackageOffset::Royalty) => royalty.into(),
        SubstateOffset::Package(PackageOffset::FunctionAccessRules) => function_access_rules.into(),
    ));

    // Prepare node modules.
    let mut node_modules = BTreeMap::new();
    node_modules.insert(
        NodeModuleId::TypeInfo,
        RENodeModuleInit::TypeInfo(TypeInfoSubstate::Object {
            package_address: PACKAGE_PACKAGE,
            blueprint_name: PACKAGE_BLUEPRINT.to_string(),
            global: true,
        }),
    );
    node_modules.insert(
        NodeModuleId::Metadata,
        RENodeModuleInit::Metadata(
            metadata
                .into_iter()
                .map(|(key, value)| {
                    (
                        SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(
                            scrypto_encode(&key).unwrap(),
                        )),
                        RuntimeSubstate::KeyValueStoreEntry(Some(ScryptoValue::String { value })),
                    )
                })
                .collect(),
        ),
    );
    node_modules.insert(
        NodeModuleId::AccessRules,
        RENodeModuleInit::MethodAccessRules(MethodAccessRulesSubstate {
            access_rules: access_rules,
        }),
    );
    node_modules.insert(
        NodeModuleId::ComponentRoyalty,
        RENodeModuleInit::ComponentRoyalty(
            ComponentRoyaltyConfigSubstate {
                royalty_config: RoyaltyConfig::default(),
            },
            ComponentRoyaltyAccumulatorSubstate {
                royalty_vault: None,
            },
        ),
    );

    let node_id = if let Some(address) = package_address {
        RENodeId::GlobalObject(PackageAddress::Normal(address).into())
    } else {
        api.kernel_allocate_node_id(AllocateEntityType::GlobalPackage)?
    };
    api.kernel_create_node(node_id, node_init, node_modules)?;

    let package_address: PackageAddress = node_id.into();
    Ok(IndexedScryptoValue::from_typed(&package_address))
}

pub struct PackageNativePackage;

impl PackageNativePackage {
    pub fn schema() -> PackageSchema {
        let mut aggregator = TypeAggregator::<ScryptoCustomTypeKind>::new();

        let substates = Vec::new();

        let mut functions = BTreeMap::new();
        functions.insert(
            PACKAGE_PUBLISH_WASM_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<PackagePublishWasmInput>(),
                output: aggregator.add_child_type_and_descendents::<PackagePublishWasmOutput>(),
                export_name: PACKAGE_PUBLISH_WASM_IDENT.to_string(),
            },
        );
        functions.insert(
            PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            FunctionSchema {
                receiver: None,
                input: aggregator.add_child_type_and_descendents::<PackagePublishNativeInput>(),
                output: aggregator.add_child_type_and_descendents::<PackagePublishNativeOutput>(),
                export_name: PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            },
        );
        functions.insert(
            PACKAGE_SET_ROYALTY_CONFIG_IDENT.to_string(),
            FunctionSchema {
                receiver: Some(schema::Receiver::SelfRefMut),
                input: aggregator.add_child_type_and_descendents::<PackageSetRoyaltyConfigInput>(),
                output: aggregator
                    .add_child_type_and_descendents::<PackageSetRoyaltyConfigOutput>(),
                export_name: PACKAGE_SET_ROYALTY_CONFIG_IDENT.to_string(),
            },
        );
        functions.insert(
            PACKAGE_CLAIM_ROYALTY_IDENT.to_string(),
            FunctionSchema {
                receiver: Some(schema::Receiver::SelfRefMut),
                input: aggregator.add_child_type_and_descendents::<PackageClaimRoyaltyInput>(),
                output: aggregator.add_child_type_and_descendents::<PackageClaimRoyaltyOutput>(),
                export_name: PACKAGE_CLAIM_ROYALTY_IDENT.to_string(),
            },
        );

        let schema = generate_full_schema(aggregator);
        PackageSchema {
            blueprints: btreemap!(
                PACKAGE_BLUEPRINT.to_string() => BlueprintSchema {
                    schema,
                    substates,
                    functions,
                    event_schema: [].into()
                }
            ),
        }
    }

    pub fn function_access_rules() -> BTreeMap<FnKey, AccessRule> {
        let mut access_rules = BTreeMap::new();
        access_rules.insert(
            FnKey::new(
                PACKAGE_BLUEPRINT.to_string(),
                PACKAGE_PUBLISH_WASM_IDENT.to_string(),
            ),
            rule!(allow_all),
        );
        access_rules.insert(
            FnKey::new(
                PACKAGE_BLUEPRINT.to_string(),
                PACKAGE_PUBLISH_NATIVE_IDENT.to_string(),
            ),
            rule!(require(SYSTEM_TOKEN)),
        );
        access_rules
    }

    pub fn invoke_export<Y>(
        export_name: &str,
        receiver: Option<RENodeId>,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        match export_name {
            PACKAGE_PUBLISH_NATIVE_IDENT => {
                api.consume_cost_units(FIXED_HIGH_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }

                Self::publish_native(input, api)
            }
            PACKAGE_PUBLISH_WASM_IDENT => {
                api.consume_cost_units(FIXED_HIGH_FEE, ClientCostingReason::RunNative)?;

                if receiver.is_some() {
                    return Err(RuntimeError::InterpreterError(
                        InterpreterError::NativeUnexpectedReceiver(export_name.to_string()),
                    ));
                }

                Self::publish_wasm(input, api)
            }

            PACKAGE_SET_ROYALTY_CONFIG_IDENT => {
                api.consume_cost_units(FIXED_MEDIUM_FEE, ClientCostingReason::RunNative)?;

                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;

                Self::set_royalty_config(receiver, input, api)
            }
            PACKAGE_CLAIM_ROYALTY_IDENT => {
                api.consume_cost_units(FIXED_MEDIUM_FEE, ClientCostingReason::RunNative)?;

                let receiver = receiver.ok_or(RuntimeError::InterpreterError(
                    InterpreterError::NativeExpectedReceiver(export_name.to_string()),
                ))?;

                Self::claim_royalty(receiver, input, api)
            }
            _ => Err(RuntimeError::InterpreterError(
                InterpreterError::NativeExportDoesNotExist(export_name.to_string()),
            )),
        }
    }

    pub(crate) fn publish_native<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: PackagePublishNativeInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        // Validate schema
        validate_package_schema(&input.schema)
            .map_err(|e| RuntimeError::ApplicationError(ApplicationError::PackageError(e)))?;
        validate_package_event_schema(&input.schema)
            .map_err(|e| RuntimeError::ApplicationError(ApplicationError::PackageError(e)))?;

        // Build node init
        let info = PackageInfoSubstate {
            schema: input.schema,
            dependent_resources: input.dependent_resources.into_iter().collect(),
            dependent_components: input.dependent_components.into_iter().collect(),
        };
        let code_type = PackageCodeTypeSubstate::Native;
        let code = PackageCodeSubstate {
            code: vec![input.native_package_code_id],
        };
        let royalty = PackageRoyaltySubstate {
            royalty_vault: None,
            blueprint_royalty_configs: BTreeMap::new(),
        };
        let function_access_rules = FunctionAccessRulesSubstate {
            access_rules: input.package_access_rules,
            default_auth: input.default_package_access_rule,
        };

        globalize_package(
            input.package_address,
            info,
            code_type,
            code,
            royalty,
            function_access_rules,
            input.metadata,
            input.access_rules,
            api,
        )
    }

    pub(crate) fn publish_wasm<Y>(
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: PackagePublishWasmInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        // Validate schema
        validate_package_schema(&input.schema)
            .map_err(|e| RuntimeError::ApplicationError(ApplicationError::PackageError(e)))?;
        validate_package_event_schema(&input.schema)
            .map_err(|e| RuntimeError::ApplicationError(ApplicationError::PackageError(e)))?;

        // Validate WASM
        WasmValidator::default()
            .validate(&input.code, &input.schema)
            .map_err(|e| {
                RuntimeError::ApplicationError(ApplicationError::PackageError(
                    PackageError::InvalidWasm(e),
                ))
            })?;

        // Build node init
        let info = PackageInfoSubstate {
            schema: input.schema,
            dependent_resources: BTreeSet::new(),
            dependent_components: BTreeSet::new(),
        };

        let code_type = PackageCodeTypeSubstate::Wasm;
        let code = PackageCodeSubstate { code: input.code };
        let royalty = PackageRoyaltySubstate {
            royalty_vault: None,
            blueprint_royalty_configs: input.royalty_config,
        };
        let function_access_rules = FunctionAccessRulesSubstate {
            access_rules: BTreeMap::new(),
            default_auth: AccessRule::AllowAll,
        };

        globalize_package(
            input.package_address,
            info,
            code_type,
            code,
            royalty,
            function_access_rules,
            input.metadata,
            input.access_rules,
            api,
        )
    }

    pub(crate) fn set_royalty_config<Y>(
        receiver: RENodeId,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let input: PackageSetRoyaltyConfigInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        // FIXME: double check if auth is set up for any package

        let handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::Package(PackageOffset::Royalty),
            LockFlags::MUTABLE,
        )?;

        let substate: &mut PackageRoyaltySubstate = api.kernel_get_substate_ref_mut(handle)?;
        substate.blueprint_royalty_configs = input.royalty_config;
        api.kernel_drop_lock(handle)?;
        Ok(IndexedScryptoValue::from_typed(&()))
    }

    pub(crate) fn claim_royalty<Y>(
        receiver: RENodeId,
        input: IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<IndexedScryptoValue, RuntimeError>
    where
        Y: KernelNodeApi + KernelSubstateApi + ClientApi<RuntimeError>,
    {
        let _input: PackageClaimRoyaltyInput = input.as_typed().map_err(|e| {
            RuntimeError::InterpreterError(InterpreterError::ScryptoInputDecodeError(e))
        })?;

        let handle = api.sys_lock_substate(
            receiver,
            SubstateOffset::Package(PackageOffset::Royalty),
            LockFlags::MUTABLE,
        )?;

        let substate: &mut PackageRoyaltySubstate = api.kernel_get_substate_ref_mut(handle)?;
        let bucket = match substate.royalty_vault.clone() {
            Some(vault) => Vault(vault.vault_id()).sys_take_all(api)?,
            None => ResourceManager(RADIX_TOKEN).new_empty_bucket(api)?,
        };
        Ok(IndexedScryptoValue::from_typed(&bucket))
    }
}
