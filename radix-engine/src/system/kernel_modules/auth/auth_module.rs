use super::auth_converter::convert_contextless;
use super::authorization::MethodAuthorization;
use super::Authentication;
use super::HardAuthRule;
use super::HardProofRule;
use super::HardResourceOrNonFungible;
use crate::blueprints::resource::AuthZone;
use crate::errors::*;
use crate::kernel::actor::Actor;
use crate::kernel::call_frame::CallFrameUpdate;
use crate::kernel::call_frame::RefType;
use crate::kernel::kernel_api::KernelModuleApi;
use crate::kernel::module::KernelModule;
use crate::system::kernel_modules::auth::convert;
use crate::system::node::RENodeInit;
use crate::system::node::RENodeModuleInit;
use crate::system::node_modules::access_rules::{
    AccessRulesNativePackage, FunctionAccessRulesSubstate, MethodAccessRulesSubstate,
};
use crate::system::node_modules::type_info::TypeInfoBlueprint;
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::system::node_substates::RuntimeSubstate;
use crate::types::*;
use radix_engine_interface::api::component::ComponentStateSubstate;
use radix_engine_interface::api::node_modules::auth::*;
use radix_engine_interface::api::substate_api::LockFlags;
use radix_engine_interface::api::types::{RENodeId, SubstateOffset};
use radix_engine_interface::blueprints::package::{
    PackageInfoSubstate, PACKAGE_BLUEPRINT, PACKAGE_PUBLISH_NATIVE_IDENT,
};
use radix_engine_interface::blueprints::resource::*;
use radix_engine_interface::blueprints::transaction_processor::TRANSACTION_PROCESSOR_BLUEPRINT;
use transaction::model::AuthZoneParams;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum AuthError {
    VisibilityError(RENodeId),
    Unauthorized(Box<Unauthorized>),
    ChildBlueprintDoesNotExist,
}
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct Unauthorized {
    pub callee: Actor,
    pub authorization: MethodAuthorization,
}

#[derive(Debug, Clone)]
pub struct AuthModule {
    pub params: AuthZoneParams,
    /// Stack of auth zones
    pub auth_zone_stack: Vec<RENodeId>,
}

impl AuthModule {
    fn is_barrier(actor: &Actor) -> bool {
        matches!(
            actor,
            Actor::Method {
                node_id: RENodeId::GlobalObject(..),
                ..
            }
        )
    }

    fn is_transaction_processor(actor: &Option<Actor>) -> bool {
        match actor {
            Some(actor) => {
                let blueprint = actor.blueprint();
                blueprint.eq(&Blueprint::new(
                    &TRANSACTION_PROCESSOR_PACKAGE,
                    TRANSACTION_PROCESSOR_BLUEPRINT,
                ))
            }
            None => false,
        }
    }

    fn function_auth<Y: KernelModuleApi<RuntimeError>>(
        blueprint: &Blueprint,
        ident: &str,
        api: &mut Y,
    ) -> Result<MethodAuthorization, RuntimeError> {
        let auth = if blueprint.package_address.eq(&PACKAGE_PACKAGE) {
            // TODO: remove
            if blueprint.blueprint_name.eq(PACKAGE_BLUEPRINT)
                && ident.eq(PACKAGE_PUBLISH_NATIVE_IDENT)
            {
                MethodAuthorization::Protected(HardAuthRule::ProofRule(HardProofRule::Require(
                    HardResourceOrNonFungible::NonFungible(AuthAddresses::system_role()),
                )))
            } else {
                MethodAuthorization::AllowAll
            }
        } else {
            let handle = api.kernel_lock_substate(
                &RENodeId::GlobalObject((blueprint.package_address).into()),
                NodeModuleId::SELF,
                SubstateOffset::Package(PackageOffset::FunctionAccessRules),
                LockFlags::read_only(),
            )?;
            let package_access_rules: &FunctionAccessRulesSubstate =
                api.kernel_get_substate_ref(handle)?;
            let function_key = FnKey::new(blueprint.blueprint_name.to_string(), ident.to_string());
            let access_rule = package_access_rules
                .access_rules
                .get(&function_key)
                .unwrap_or(&package_access_rules.default_auth);
            convert_contextless(access_rule)
        };

        Ok(auth)
    }

    fn method_auth<Y: KernelModuleApi<RuntimeError>>(
        node_id: &RENodeId,
        module_id: &NodeModuleId,
        ident: &str,
        args: &IndexedScryptoValue,
        api: &mut Y,
    ) -> Result<MethodAuthorization, RuntimeError> {
        let auth = match (node_id, module_id, ident) {
            (node_id, module_id, ident) if matches!(module_id, NodeModuleId::AccessRules) => {
                AccessRulesNativePackage::authorization(node_id, ident, args, api)?
            }

            (RENodeId::Object(object_id), ..) => {
                let node_id = RENodeId::Object(*object_id);
                let info = api.get_object_info(node_id)?;
                if let Some(parent) = info.type_parent {
                    let (ref_type, _) =
                        api.kernel_get_node_info(node_id)
                            .ok_or(RuntimeError::CallFrameError(
                                CallFrameError::RENodeNotVisible(node_id),
                            ))?;
                    let method_key = MethodKey::new(*module_id, ident);
                    let auth = Self::method_authorization_stateless(
                        ref_type,
                        &RENodeId::GlobalObject(parent.into()),
                        ObjectKey::ChildBlueprint(info.blueprint.blueprint_name),
                        method_key,
                        api,
                    )?;

                    auth
                } else {
                    MethodAuthorization::AllowAll
                }
            }

            (node_id, module_id, ..) => {
                let method_key = MethodKey::new(*module_id, ident);

                // TODO: Clean this up
                let auth = if matches!(
                    node_id,
                    RENodeId::GlobalObject(Address::Component(ComponentAddress::Normal(..)))
                ) && module_id.eq(&NodeModuleId::SELF)
                {
                    Self::method_authorization_stateful(&node_id, ObjectKey::SELF, method_key, api)?
                } else {
                    Self::method_authorization_stateless(
                        RefType::Normal,
                        &node_id,
                        ObjectKey::SELF,
                        method_key,
                        api,
                    )?
                };

                auth
            }
        };

        Ok(auth)
    }

    fn method_authorization_stateful<Y: KernelModuleApi<RuntimeError>>(
        receiver: &RENodeId,
        object_key: ObjectKey,
        method_key: MethodKey,
        api: &mut Y,
    ) -> Result<MethodAuthorization, RuntimeError> {
        let (blueprint_schema, index) = {
            let type_info = TypeInfoBlueprint::get_type(receiver, api)?;
            let blueprint = match type_info {
                TypeInfoSubstate::Object(ObjectInfo { blueprint, .. }) => blueprint,
                TypeInfoSubstate::KeyValueStore(..) => {
                    return Err(RuntimeError::SystemError(SystemError::NotAnObject))
                }
                TypeInfoSubstate::IterableMap(..) => {
                    return Err(RuntimeError::SystemError(SystemError::NotAnObject))
                }
            };

            let handle = api.kernel_lock_substate(
                &RENodeId::GlobalObject(blueprint.package_address.into()),
                NodeModuleId::SELF,
                SubstateOffset::Package(PackageOffset::Info),
                LockFlags::read_only(),
            )?;
            let package: &PackageInfoSubstate = api.kernel_get_substate_ref(handle)?;
            let schema = package
                .schema
                .blueprints
                .get(&blueprint.blueprint_name)
                .expect("Blueprint schema not found")
                .clone();
            let index = match schema.substates.get(0) {
                Some(index) => index.clone(),
                None => {
                    return Self::method_authorization_stateless(
                        RefType::Normal,
                        receiver,
                        object_key,
                        method_key,
                        api,
                    );
                }
            };

            api.kernel_drop_lock(handle)?;
            (schema, index)
        };

        let state = {
            let offset = SubstateOffset::Component(ComponentOffset::State0);
            let handle = api.kernel_lock_substate(
                receiver,
                NodeModuleId::SELF,
                offset,
                LockFlags::read_only(),
            )?;
            let state: &ComponentStateSubstate = api.kernel_get_substate_ref(handle)?;
            let state = IndexedScryptoValue::from_scrypto_value(state.0.clone());
            api.kernel_drop_lock(handle)?;
            state
        };

        let handle = api.kernel_lock_substate(
            receiver,
            NodeModuleId::AccessRules,
            SubstateOffset::AccessRules(AccessRulesOffset::AccessRules),
            LockFlags::read_only(),
        )?;
        let access_rules: &MethodAccessRulesSubstate = api.kernel_get_substate_ref(handle)?;

        let method_auth = access_rules
            .access_rules
            .get_access_rule(false, &method_key);
        let authorization = convert(&blueprint_schema.schema, index, &state, &method_auth);

        api.kernel_drop_lock(handle)?;

        Ok(authorization)
    }

    fn method_authorization_stateless<Y: KernelModuleApi<RuntimeError>>(
        ref_type: RefType,
        receiver: &RENodeId,
        object_key: ObjectKey,
        key: MethodKey,
        api: &mut Y,
    ) -> Result<MethodAuthorization, RuntimeError> {
        let handle = api.kernel_lock_substate(
            receiver,
            NodeModuleId::AccessRules,
            SubstateOffset::AccessRules(AccessRulesOffset::AccessRules),
            LockFlags::read_only(),
        )?;
        let access_rules: &MethodAccessRulesSubstate = api.kernel_get_substate_ref(handle)?;

        let is_direct_access = matches!(ref_type, RefType::DirectAccess);

        let method_auth = match object_key {
            ObjectKey::SELF => access_rules
                .access_rules
                .get_access_rule(is_direct_access, &key),
            ObjectKey::ChildBlueprint(blueprint_name) => {
                let child_rules = access_rules
                    .child_blueprint_rules
                    .get(&blueprint_name)
                    .ok_or(RuntimeError::ModuleError(ModuleError::AuthError(
                        AuthError::ChildBlueprintDoesNotExist,
                    )))?;
                child_rules.get_access_rule(is_direct_access, &key)
            }
        };

        // TODO: Remove
        let authorization = convert_contextless(&method_auth);

        api.kernel_drop_lock(handle)?;

        Ok(authorization)
    }

    pub fn last_auth_zone(&self) -> RENodeId {
        self.auth_zone_stack
            .last()
            .cloned()
            .expect("Missing auth zone")
    }
}

impl KernelModule for AuthModule {
    fn on_init<Y: KernelModuleApi<RuntimeError>>(api: &mut Y) -> Result<(), RuntimeError> {
        // Create sentinel node
        Self::on_execution_start(api, &None)
    }

    fn on_teardown<Y: KernelModuleApi<RuntimeError>>(api: &mut Y) -> Result<(), RuntimeError> {
        // Destroy sentinel node
        Self::on_execution_finish(api, &None, &CallFrameUpdate::empty())
    }

    fn before_push_frame<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        callee: &Actor,
        _call_frame_update: &mut CallFrameUpdate,
        args: &IndexedScryptoValue,
    ) -> Result<(), RuntimeError> {
        // Decide `authorization`, `barrier_crossing_allowed`, and `tip_auth_zone_id`
        let authorization = match &callee {
            Actor::Method {
                node_id,
                module_id,
                ident,
                ..
            } => Self::method_auth(node_id, module_id, ident.as_str(), &args, api)?,
            Actor::Function { blueprint, ident } => {
                Self::function_auth(blueprint, ident.as_str(), api)?
            }
            Actor::VirtualLazyLoad { .. } => return Ok(()),
        };
        let barrier_crossings_required = 0;
        let barrier_crossings_allowed = if Self::is_barrier(callee) { 0 } else { 1 };
        let auth_zone_id = api.kernel_get_module_state().auth.last_auth_zone();

        // Authenticate
        if !Authentication::verify_method_auth(
            barrier_crossings_required,
            barrier_crossings_allowed,
            auth_zone_id,
            &authorization,
            api,
        )? {
            return Err(RuntimeError::ModuleError(ModuleError::AuthError(
                AuthError::Unauthorized(Box::new(Unauthorized {
                    callee: callee.clone(),
                    authorization,
                })),
            )));
        }

        Ok(())
    }

    fn on_execution_start<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        _caller: &Option<Actor>,
    ) -> Result<(), RuntimeError> {
        let actor = api.kernel_get_current_actor();

        // Add Global Object and Package Actor Auth
        let mut virtual_non_fungibles_non_extending = BTreeSet::new();
        if let Some(actor) = &actor {
            let package_address = actor.package_address();
            let id = scrypto_encode(&package_address).unwrap();
            let non_fungible_global_id =
                NonFungibleGlobalId::new(PACKAGE_TOKEN, NonFungibleLocalId::bytes(id).unwrap());
            virtual_non_fungibles_non_extending.insert(non_fungible_global_id);

            if let Actor::Method {
                global_address: Some(address),
                ..
            } = &actor
            {
                let id = scrypto_encode(&address).unwrap();
                let non_fungible_global_id = NonFungibleGlobalId::new(
                    GLOBAL_OBJECT_TOKEN,
                    NonFungibleLocalId::bytes(id).unwrap(),
                );
                virtual_non_fungibles_non_extending.insert(non_fungible_global_id);
            }
        }

        // Prepare a new auth zone
        let is_barrier = if let Some(actor) = &actor {
            Self::is_barrier(actor)
        } else {
            false
        };
        let is_transaction_processor = Self::is_transaction_processor(&actor);
        let (virtual_resources, virtual_non_fungibles) = if is_transaction_processor {
            let auth_module = &api.kernel_get_module_state().auth;
            (
                auth_module.params.virtual_resources.clone(),
                auth_module.params.initial_proofs.clone(),
            )
        } else {
            (BTreeSet::new(), BTreeSet::new())
        };
        let parent = api
            .kernel_get_module_state()
            .auth
            .auth_zone_stack
            .last()
            .map(|x| InternalRef(x.clone().into()));
        let auth_zone = AuthZone::new(
            vec![],
            virtual_resources,
            virtual_non_fungibles,
            virtual_non_fungibles_non_extending,
            is_barrier,
            parent,
        );

        // Create node
        let auth_zone_node_id = api.kernel_allocate_node_id(AllocateEntityType::Object)?;
        api.kernel_create_node(
            auth_zone_node_id,
            RENodeInit::Object(btreemap!(
                SubstateOffset::AuthZone(AuthZoneOffset::AuthZone) => RuntimeSubstate::AuthZone(auth_zone)
            )),
            btreemap!(
                NodeModuleId::TypeInfo => RENodeModuleInit::TypeInfo(TypeInfoSubstate::Object(ObjectInfo {
                    blueprint: Blueprint::new(&RESOURCE_MANAGER_PACKAGE, AUTH_ZONE_BLUEPRINT),
                    global: false,
                    type_parent: None,
                }))
            ),
        )?;

        api.kernel_get_module_state()
            .auth
            .auth_zone_stack
            .push(auth_zone_node_id);

        Ok(())
    }

    fn on_execution_finish<Y: KernelModuleApi<RuntimeError>>(
        api: &mut Y,
        _caller: &Option<Actor>,
        _update: &CallFrameUpdate,
    ) -> Result<(), RuntimeError> {
        let auth_zone = api
            .kernel_get_module_state()
            .auth
            .auth_zone_stack
            .pop()
            .expect("Auth zone stack is broken");

        api.kernel_drop_node(&auth_zone)?;

        // Proofs in auth zone will be re-owned by the frame and auto dropped.

        Ok(())
    }
}
