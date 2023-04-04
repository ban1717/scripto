use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct Data {}

#[blueprint]
mod vault_test {
    struct NonFungibleVault {
        vault: Vault,
    }

    impl NonFungibleVault {
        fn create_non_fungible_vault() -> Vault {
            let bucket = ResourceBuilder::new_integer_non_fungible()
                .metadata("name", "TestToken")
                .mint_initial_supply([(1u64.into(), Data {})]);
            Vault::with_bucket(bucket)
        }

        pub fn new_non_fungible_vault() -> ComponentAddress {
            let vault = Self::create_non_fungible_vault();
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_non_fungible_vault_with_take() -> ComponentAddress {
            let mut vault = Self::create_non_fungible_vault();
            let bucket = vault.take(1);
            vault.put(bucket);
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_non_fungible_vault_with_take_twice() -> ComponentAddress {
            let mut vault = Self::create_non_fungible_vault();
            let bucket = vault.take(1);
            vault.put(bucket);
            let bucket = vault.take(1);
            vault.put(bucket);
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_non_fungible_vault_with_take_non_fungible() -> ComponentAddress {
            let mut vault = Self::create_non_fungible_vault();
            let bucket = vault.take_non_fungible(&NonFungibleLocalId::integer(1));
            vault.put(bucket);
            Self {
                vault
            }
                .instantiate()
                .globalize()
        }

        pub fn new_vault_with_get_non_fungible_local_ids() -> ComponentAddress {
            let vault = Self::create_non_fungible_vault();
            let _ids = vault.non_fungible_local_ids();
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_vault_with_get_non_fungible_local_id() -> ComponentAddress {
            let vault = Self::create_non_fungible_vault();
            let _id = vault.non_fungible_local_id();
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_vault_with_get_amount() -> ComponentAddress {
            let vault = Self::create_non_fungible_vault();
            let _amount = vault.amount();
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn new_vault_with_get_resource_manager() -> ComponentAddress {
            let vault = Self::create_non_fungible_vault();
            let _resource_manager = vault.resource_address();
            Self {
                vault,
            }
                .instantiate()
                .globalize()
        }

        pub fn take_twice(&mut self) {
            let bucket = self.vault.take(1);
            self.vault.put(bucket);
            let bucket = self.vault.take(1);
            self.vault.put(bucket);
        }
    }
}