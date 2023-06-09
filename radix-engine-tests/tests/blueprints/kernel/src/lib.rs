use scrypto::api::substate_api::LockFlags;
use scrypto::api::*;
use scrypto::engine::scrypto_env::*;
use scrypto::prelude::*;

#[blueprint]
mod read {
    struct Read {}

    impl Read {
        pub fn read_global_substate(component_address: ComponentAddress) {
            ScryptoEnv
                .sys_lock_substate(
                    RENodeId::GlobalObject(component_address.into()),
                    SubstateOffset::TypeInfo(TypeInfoOffset::TypeInfo),
                    LockFlags::read_only(),
                )
                .unwrap();
        }
    }
}

#[blueprint]
mod node_create {
    struct NodeCreate {}

    impl NodeCreate {
        pub fn create_node_with_invalid_blueprint() {
            ScryptoEnv
                .new_object(
                    "invalid_blueprint",
                    vec![scrypto_encode(&NodeCreate {}).unwrap()],
                )
                .unwrap();
        }
    }
}
