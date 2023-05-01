use radix_engine_common::data::scrypto::{
    scrypto_decode, scrypto_encode, ScryptoDecode, ScryptoEncode,
};
use sbor::rust::prelude::*;

pub type KeyValueEntryHandle = u32;

// TODO: Add locked entry interface rather than using substate api
pub trait ClientKeyValueEntryApi<E> {
    fn key_value_entry_get(&mut self, handle: KeyValueEntryHandle) -> Result<Vec<u8>, E>;

    fn key_value_entry_get_typed<S: ScryptoDecode>(
        &mut self,
        handle: KeyValueEntryHandle,
    ) -> Result<S, E> {
        let buffer = self.key_value_entry_get(handle)?;
        let value: S = scrypto_decode(&buffer).unwrap();
        Ok(value)
    }

    fn key_value_entry_set(
        &mut self,
        handle: KeyValueEntryHandle,
        buffer: Vec<u8>,
    ) -> Result<(), E>;

    fn key_value_entry_set_typed<S: ScryptoEncode>(
        &mut self,
        handle: KeyValueEntryHandle,
        value: S,
    ) -> Result<(), E> {
        let buffer = scrypto_encode(&value).unwrap();
        self.key_value_entry_set(handle, buffer)
    }

    fn key_value_entry_release(&mut self, handle: KeyValueEntryHandle) -> Result<(), E>;
}