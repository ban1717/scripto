use radix_engine::ledger::{
    OutputValue, QueryableSubstateStore, ReadableSubstateStore, WriteableSubstateStore,
};
use radix_engine::system::node_substates::{PersistedSubstate, RuntimeSubstate};
use radix_engine::types::*;
use radix_engine_interface::api::types::RENodeId;

/// A substate store that stores all serialized substates in host memory.
#[derive(Debug, PartialEq, Eq)]
pub struct SerializedInMemorySubstateStore {
    /// A hashmap from SBOR-encoded `SubstateId`s to SBOR-encoded `OutputValue`s.
    /// This structure does not preserve deterministic ordering, but it is only used for test
    /// purposes (where it actually puts the Engine's determinism under test).
    substates: HashMap<Vec<u8>, Vec<u8>>,
}

impl SerializedInMemorySubstateStore {
    pub fn new() -> Self {
        Self {
            substates: HashMap::new(),
        }
    }
}

impl Default for SerializedInMemorySubstateStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ReadableSubstateStore for SerializedInMemorySubstateStore {
    fn get_substate(&self, substate_id: &SubstateId) -> Option<OutputValue> {
        self.substates
            .get(&scrypto_encode(substate_id).expect("Could not encode substate id"))
            .map(|b| scrypto_decode(&b).unwrap())
    }

    fn first_in_iterable(
        &self,
        node_id: &RENodeId,
        module_id: NodeModuleId,
        count: u32,
    ) -> Vec<(SubstateId, RuntimeSubstate)> {
        todo!()
    }
}

impl WriteableSubstateStore for SerializedInMemorySubstateStore {
    fn put_substate(&mut self, substate_id: SubstateId, substate: OutputValue) {
        self.substates.insert(
            scrypto_encode(&substate_id).expect("Could not encode substate id"),
            scrypto_encode(&substate).expect("Could not encode substate"),
        );
    }

    fn remove_substate(&mut self, substate_id: &SubstateId) {
        let encoded = scrypto_encode(substate_id).unwrap();
        self.substates.remove(&encoded);
    }
}

impl QueryableSubstateStore for SerializedInMemorySubstateStore {
    fn get_kv_store_entries(
        &self,
        kv_store_id: &KeyValueStoreId,
    ) -> HashMap<Vec<u8>, PersistedSubstate> {
        self.substates
            .iter()
            .filter_map(|(key, value)| {
                let substate_id: SubstateId = scrypto_decode(key).unwrap();
                if let SubstateId(
                    RENodeId::KeyValueStore(id),
                    NodeModuleId::SELF,
                    SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(entry_id)),
                ) = substate_id
                {
                    let output_value: OutputValue = scrypto_decode(value).unwrap();
                    if id == *kv_store_id {
                        Some((entry_id.clone(), output_value.substate))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }
}
