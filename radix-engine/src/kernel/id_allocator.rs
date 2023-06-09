use crate::errors::{IdAllocationError, KernelError, RuntimeError};
use crate::types::*;
use radix_engine_interface::address::EntityType;

/// An ID allocator defines how identities are generated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdAllocator {
    pre_allocated_ids: BTreeSet<RENodeId>,
    frame_allocated_ids: Vec<BTreeSet<RENodeId>>,
    next_entity_ids: BTreeMap<EntityType, u32>,
    next_id: u32,
    transaction_hash: Hash,
}

impl IdAllocator {
    /// Creates an ID allocator.
    pub fn new(transaction_hash: Hash, pre_allocated_ids: BTreeSet<RENodeId>) -> Self {
        Self {
            pre_allocated_ids,
            frame_allocated_ids: vec![BTreeSet::new()],
            next_entity_ids: BTreeMap::new(),
            next_id: 0u32,
            transaction_hash,
        }
    }

    pub fn push(&mut self) {
        self.frame_allocated_ids.push(BTreeSet::new());
    }

    pub fn pop(&mut self) -> Result<(), RuntimeError> {
        let ids = self.frame_allocated_ids.pop().expect("No frame found");
        if !ids.is_empty() {
            return Err(RuntimeError::KernelError(KernelError::IdAllocationError(
                IdAllocationError::AllocatedIDsNotEmpty(ids),
            )));
        }
        Ok(())
    }

    pub fn take_node_id(&mut self, node_id: RENodeId) -> Result<(), RuntimeError> {
        let ids = self.frame_allocated_ids.last_mut().expect("No frame found");
        let frame_allocated = ids.remove(&node_id);
        let pre_allocated = self.pre_allocated_ids.remove(&node_id);
        if !frame_allocated && !pre_allocated {
            return Err(RuntimeError::KernelError(KernelError::IdAllocationError(
                IdAllocationError::RENodeIdWasNotAllocated(node_id),
            )));
        }
        Ok(())
    }

    // Protected, only virtual manager should call this
    // TODO: Clean up interface
    pub fn allocate_virtual_node_id(&mut self, node_id: RENodeId) {
        let ids = self
            .frame_allocated_ids
            .last_mut()
            .expect("No frame found.");
        ids.insert(node_id);
    }

    pub fn allocate_node_id(
        &mut self,
        node_type: AllocateEntityType,
    ) -> Result<RENodeId, RuntimeError> {
        let node_id = match node_type {
            AllocateEntityType::AuthZoneStack => Ok(RENodeId::AuthZoneStack),
            AllocateEntityType::KeyValueStore => {
                self.new_kv_store_id().map(|id| RENodeId::KeyValueStore(id))
            }
            AllocateEntityType::Object => self.new_object_id().map(|id| RENodeId::Object(id)),
            AllocateEntityType::Vault => self.new_vault_id().map(|id| RENodeId::Object(id)),
            AllocateEntityType::GlobalPackage => self
                .new_package_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalEpochManager => self
                .new_epoch_manager_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalValidator => self
                .new_validator_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalFungibleResourceManager => self
                .new_fungible_resource_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalNonFungibleResourceManager => self
                .new_non_fungible_resource_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalAccount => self
                .new_account_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalIdentity => self
                .new_identity_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalComponent => self
                .new_component_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
            AllocateEntityType::GlobalAccessController => self
                .new_access_controller_address()
                .map(|address| RENodeId::GlobalObject(address.into())),
        }
        .map_err(|e| RuntimeError::KernelError(KernelError::IdAllocationError(e)))?;

        let ids = self
            .frame_allocated_ids
            .last_mut()
            .expect("No frame found.");
        ids.insert(node_id);

        Ok(node_id)
    }

    fn next(&mut self) -> Result<u32, IdAllocationError> {
        if self.next_id == u32::MAX {
            Err(IdAllocationError::OutOfID)
        } else {
            let rtn = self.next_id;
            self.next_id += 1;
            Ok(rtn)
        }
    }

    fn next_object_id(
        &mut self,
        entity_id: u8,
    ) -> Result<[u8; OBJECT_ID_LENGTH], IdAllocationError> {
        let mut buf = [0u8; OBJECT_ID_LENGTH];
        buf[0] = entity_id;
        (&mut buf[1..OBJECT_HASH_END])
            .copy_from_slice(&self.transaction_hash.0[0..OBJECT_HASH_LENGTH]);
        (&mut buf[OBJECT_HASH_END..]).copy_from_slice(&self.next()?.to_le_bytes());
        Ok(buf)
    }

    fn next_entity_id(&mut self, entity_type: EntityType) -> Result<u32, IdAllocationError> {
        let rtn = if let Some(next) = self.next_entity_ids.get_mut(&entity_type) {
            let cur = *next;
            if cur == u32::MAX {
                return Err(IdAllocationError::OutOfID);
            }
            *next += 1;
            cur
        } else {
            self.next_entity_ids.insert(entity_type, 1u32);
            0u32
        };

        Ok(rtn)
    }

    /// Creates a new package ID.
    pub fn new_package_address(&mut self) -> Result<PackageAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::Package)?;
        data.extend(next_id.to_le_bytes());
        Ok(PackageAddress::Normal(hash(data).lower_26_bytes()))
    }

    pub fn new_identity_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        data.extend(self.next()?.to_le_bytes());
        Ok(ComponentAddress::Identity(hash(data).lower_26_bytes()))
    }

    pub fn new_account_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::AccountComponent)?;
        data.extend(next_id.to_le_bytes());
        Ok(ComponentAddress::Account(hash(data).lower_26_bytes()))
    }

    /// Creates a new component address.
    pub fn new_component_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::NormalComponent)?;
        data.extend(next_id.to_le_bytes());
        Ok(ComponentAddress::Normal(hash(data).lower_26_bytes()))
    }

    pub fn new_validator_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::Validator)?;
        data.extend(next_id.to_le_bytes());
        Ok(ComponentAddress::Validator(hash(data).lower_26_bytes()))
    }

    pub fn new_epoch_manager_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::EpochManager)?;
        data.extend(next_id.to_le_bytes());
        Ok(ComponentAddress::EpochManager(hash(data).lower_26_bytes()))
    }

    pub fn new_clock_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::Clock)?;
        data.extend(next_id.to_le_bytes());
        Ok(ComponentAddress::Clock(hash(data).lower_26_bytes()))
    }

    pub fn new_access_controller_address(&mut self) -> Result<ComponentAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        data.extend(self.next()?.to_le_bytes());

        Ok(ComponentAddress::AccessController(
            hash(data).lower_26_bytes(),
        ))
    }

    pub fn new_non_fungible_resource_address(
        &mut self,
    ) -> Result<ResourceAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::NonFungibleResource)?;
        data.extend(next_id.to_le_bytes());
        Ok(ResourceAddress::NonFungible(hash(data).lower_26_bytes()))
    }

    pub fn new_fungible_resource_address(&mut self) -> Result<ResourceAddress, IdAllocationError> {
        let mut data = self.transaction_hash.to_vec();
        let next_id = self.next_entity_id(EntityType::FungibleResource)?;
        data.extend(next_id.to_le_bytes());
        Ok(ResourceAddress::Fungible(hash(data).lower_26_bytes()))
    }

    pub fn new_object_id(&mut self) -> Result<ObjectId, IdAllocationError> {
        self.next_object_id(INTERNAL_OBJECT_NORMAL_COMPONENT_ID)
    }

    pub fn new_vault_id(&mut self) -> Result<ObjectId, IdAllocationError> {
        self.next_object_id(INTERNAL_OBJECT_VAULT_ID)
    }

    /// Creates a new key value store ID.
    pub fn new_kv_store_id(&mut self) -> Result<KeyValueStoreId, IdAllocationError> {
        self.next_object_id(INTERNAL_KV_STORE_ID)
    }
}
