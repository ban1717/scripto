use crate::ledger::{
    QueryableSubstateStore, ReadableSubstateStore, StateTreeTraverser, StateTreeTraverserError,
    StateTreeVisitor,
};
use crate::types::hash_map::Entry;
use crate::types::*;
use radix_engine_interface::blueprints::resource::{
    LiquidFungibleResource, LiquidNonFungibleResource,
};
use crate::blueprints::resource::LiquidNonFungibleVault;

pub struct ResourceAccounter<'s, S: ReadableSubstateStore + QueryableSubstateStore> {
    substate_store: &'s S,
    accounting: Accounting,
}

impl<'s, S: ReadableSubstateStore + QueryableSubstateStore> ResourceAccounter<'s, S> {
    pub fn new(substate_store: &'s S) -> Self {
        ResourceAccounter {
            substate_store,
            accounting: Accounting::new(),
        }
    }

    pub fn add_resources(&mut self, node_id: RENodeId) -> Result<(), StateTreeTraverserError> {
        let mut state_tree_visitor =
            StateTreeTraverser::new(self.substate_store, &mut self.accounting, 100);
        state_tree_visitor.traverse_all_descendents(None, node_id)
    }

    pub fn into_map(self) -> HashMap<ResourceAddress, Decimal> {
        self.accounting.balances
    }
}

struct Accounting {
    balances: HashMap<ResourceAddress, Decimal>,
}

impl Accounting {
    pub fn new() -> Self {
        Accounting {
            balances: HashMap::new(),
        }
    }

    pub fn add_fungible_vault(
        &mut self,
        address: &ResourceAddress,
        resource: &LiquidFungibleResource,
    ) {
        match self.balances.entry(*address) {
            Entry::Occupied(mut e) => {
                let new_amount = resource.amount() + *e.get();
                e.insert(new_amount);
            }
            Entry::Vacant(e) => {
                e.insert(resource.amount());
            }
        }
    }

    pub fn add_non_fungible_vault(
        &mut self,
        address: &ResourceAddress,
        resource: &LiquidNonFungibleVault,
    ) {
        match self.balances.entry(*address) {
            Entry::Occupied(mut e) => {
                let new_amount = resource.amount + *e.get();
                e.insert(new_amount);
            }
            Entry::Vacant(e) => {
                e.insert(resource.amount);
            }
        }
    }
}

impl StateTreeVisitor for Accounting {
    fn visit_fungible_vault(
        &mut self,
        _vault_id: ObjectId,
        address: &ResourceAddress,
        resource: &LiquidFungibleResource,
    ) {
        self.add_fungible_vault(address, resource);
    }

    fn visit_non_fungible_vault(
        &mut self,
        _vault_id: ObjectId,
        address: &ResourceAddress,
        resource: &LiquidNonFungibleVault,
    ) {
        self.add_non_fungible_vault(address, resource);
    }
}
