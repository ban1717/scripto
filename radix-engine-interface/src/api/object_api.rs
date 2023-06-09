use crate::api::types::*;
use crate::data::scrypto::model::*;
use sbor::rust::collections::*;
use sbor::rust::prelude::*;
use sbor::rust::vec::Vec;
use scrypto_schema::KeyValueStoreSchema;

pub trait ClientObjectApi<E> {
    // TODO: refine the interface
    fn new_object(
        &mut self,
        blueprint_ident: &str,
        app_states: Vec<Vec<u8>>,
    ) -> Result<ObjectId, E>;

    fn get_object_type_info(&mut self, node_id: RENodeId) -> Result<(PackageAddress, String), E>;

    fn new_key_value_store(&mut self, schema: KeyValueStoreSchema) -> Result<KeyValueStoreId, E>;

    fn get_key_value_store_info(&mut self, node_id: RENodeId) -> Result<KeyValueStoreSchema, E>;

    fn globalize(
        &mut self,
        node_id: RENodeId,
        modules: BTreeMap<NodeModuleId, ObjectId>,
    ) -> Result<Address, E>;

    fn globalize_with_address(
        &mut self,
        node_id: RENodeId,
        modules: BTreeMap<NodeModuleId, ObjectId>,
        address: Address,
    ) -> Result<Address, E>;

    fn call_method(
        &mut self,
        receiver: RENodeId,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E>;

    fn call_module_method(
        &mut self,
        receiver: RENodeId,
        node_module_id: NodeModuleId,
        method_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E>;

    fn call_function(
        &mut self,
        package_address: PackageAddress,
        blueprint_name: &str,
        function_name: &str,
        args: Vec<u8>,
    ) -> Result<Vec<u8>, E>;

    fn drop_object(&mut self, node_id: RENodeId) -> Result<(), E>;
}
