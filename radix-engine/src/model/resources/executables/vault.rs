use crate::engine::{
    ApplicationError, CallFrameUpdate, ExecutableInvocation, LockFlags, MethodDeref, NativeProgram,
    REActor, RENode, ResolvedMethod, ResolvedReceiver, RuntimeError, SystemApi, TypedExecutor,
};
use crate::fee::FeeReserveError;
use crate::model::{
    BucketSubstate, InvokeError, ProofError, ResourceContainerId, ResourceOperationError,
};
use crate::types::*;
use radix_engine_interface::api::types::{
    GlobalAddress, NativeMethod, RENodeId, SubstateOffset, VaultMethod, VaultOffset,
};
use radix_engine_interface::data::IndexedScryptoValue;
use radix_engine_interface::model::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[scrypto(TypeId, Encode, Decode)]
pub enum VaultError {
    InvalidRequestData(DecodeError),
    ResourceOperationError(ResourceOperationError),
    CouldNotCreateBucket,
    CouldNotTakeBucket,
    ProofError(ProofError),
    CouldNotCreateProof,
    LockFeeNotRadixToken,
    LockFeeInsufficientBalance,
    LockFeeRepayFailure(FeeReserveError),
}

impl ExecutableInvocation for VaultTakeInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::Take)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultTakeInvocation {
    type Output = Bucket;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Bucket, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle =
            system_api.lock_substate(RENodeId::Vault(self.receiver), offset, LockFlags::MUTABLE)?;

        let container = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault.take(self.amount).map_err(|e| match e {
                InvokeError::Error(e) => {
                    RuntimeError::ApplicationError(ApplicationError::VaultError(e))
                }
                InvokeError::Downstream(runtime_error) => runtime_error,
            })?
        };

        let bucket_id = system_api
            .create_node(RENode::Bucket(BucketSubstate::new(container)))?
            .into();

        Ok((
            Bucket(bucket_id),
            CallFrameUpdate::move_node(RENodeId::Bucket(bucket_id)),
        ))
    }
}

impl ExecutableInvocation for VaultPutInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let mut call_frame_update = CallFrameUpdate::copy_ref(receiver);
        call_frame_update
            .nodes_to_move
            .push(RENodeId::Bucket(self.bucket.0));
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::Put)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultPutInvocation {
    type Output = ();

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<((), CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::MUTABLE)?;

        let bucket = system_api
            .drop_node(RENodeId::Bucket(self.bucket.0))?
            .into();

        let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
        let vault = substate_mut.vault();
        vault.put(bucket).map_err(|e| {
            RuntimeError::ApplicationError(ApplicationError::VaultError(
                VaultError::ResourceOperationError(e),
            ))
        })?;

        Ok(((), CallFrameUpdate::empty()))
    }
}

impl ExecutableInvocation for VaultLockFeeInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::LockFee)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultLockFeeInvocation {
    type Output = ();

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<((), CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(
            node_id,
            offset,
            LockFlags::MUTABLE | LockFlags::UNMODIFIED_BASE | LockFlags::FORCE_WRITE,
        )?;

        let fee = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();

            // Check resource and take amount
            if vault.resource_address() != RADIX_TOKEN {
                return Err(RuntimeError::ApplicationError(
                    ApplicationError::VaultError(VaultError::LockFeeNotRadixToken),
                ));
            }

            // Take fee from the vault
            vault.take(self.amount).map_err(|_| {
                RuntimeError::ApplicationError(ApplicationError::VaultError(
                    VaultError::LockFeeInsufficientBalance,
                ))
            })?
        };

        // Refill fee reserve
        let changes = system_api.lock_fee(self.receiver, fee, self.contingent)?;

        // Return changes
        {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault
                .borrow_resource_mut()
                .put(changes)
                .expect("Failed to return fee changes to a locking-fee vault");
        }

        Ok(((), CallFrameUpdate::empty()))
    }
}

impl ExecutableInvocation for VaultTakeNonFungiblesInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::TakeNonFungibles)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultTakeNonFungiblesInvocation {
    type Output = Bucket;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Bucket, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::MUTABLE)?;

        let container = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault
                .take_non_fungibles(&self.non_fungible_ids)
                .map_err(|e| match e {
                    InvokeError::Error(e) => {
                        RuntimeError::ApplicationError(ApplicationError::VaultError(e))
                    }
                    InvokeError::Downstream(runtime_error) => runtime_error,
                })?
        };

        let bucket_id = system_api
            .create_node(RENode::Bucket(BucketSubstate::new(container)))?
            .into();

        Ok((
            Bucket(bucket_id),
            CallFrameUpdate::move_node(RENodeId::Bucket(bucket_id)),
        ))
    }
}

impl ExecutableInvocation for VaultGetAmountInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::GetAmount)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultGetAmountInvocation {
    type Output = Decimal;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Decimal, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::read_only())?;

        let substate_ref = system_api.get_ref(vault_handle)?;
        let vault = substate_ref.vault();
        let amount = vault.total_amount();

        Ok((amount, CallFrameUpdate::empty()))
    }
}

impl ExecutableInvocation for VaultGetResourceAddressInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::GetResourceAddress)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultGetResourceAddressInvocation {
    type Output = ResourceAddress;

    fn main<'a, Y>(
        self,
        system_api: &mut Y,
    ) -> Result<(ResourceAddress, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::read_only())?;

        let substate_ref = system_api.get_ref(vault_handle)?;
        let vault = substate_ref.vault();
        let resource_address = vault.resource_address();

        Ok((
            resource_address,
            CallFrameUpdate::copy_ref(RENodeId::Global(GlobalAddress::Resource(resource_address))),
        ))
    }
}

impl ExecutableInvocation for VaultGetNonFungibleIdsInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::GetNonFungibleIds)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultGetNonFungibleIdsInvocation {
    type Output = BTreeSet<NonFungibleId>;

    fn main<'a, Y>(
        self,
        system_api: &mut Y,
    ) -> Result<(BTreeSet<NonFungibleId>, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::read_only())?;

        let substate_ref = system_api.get_ref(vault_handle)?;
        let vault = substate_ref.vault();
        let ids = vault.total_ids().map_err(|e| {
            RuntimeError::ApplicationError(ApplicationError::VaultError(
                VaultError::ResourceOperationError(e),
            ))
        })?;

        Ok((ids, CallFrameUpdate::empty()))
    }
}

impl ExecutableInvocation for VaultCreateProofInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::CreateProof)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultCreateProofInvocation {
    type Output = Proof;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Proof, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::MUTABLE)?;

        let proof = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault
                .create_proof(ResourceContainerId::Vault(self.receiver))
                .map_err(|e| {
                    RuntimeError::ApplicationError(ApplicationError::VaultError(
                        VaultError::ProofError(e),
                    ))
                })?
        };
        let proof_id = system_api.create_node(RENode::Proof(proof))?.into();

        Ok((
            Proof(proof_id),
            CallFrameUpdate::move_node(RENodeId::Proof(proof_id)),
        ))
    }
}

impl ExecutableInvocation for VaultCreateProofByAmountInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::CreateProofByAmount)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultCreateProofByAmountInvocation {
    type Output = Proof;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Proof, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::MUTABLE)?;

        let proof = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault
                .create_proof_by_amount(self.amount, ResourceContainerId::Vault(self.receiver))
                .map_err(|e| {
                    RuntimeError::ApplicationError(ApplicationError::VaultError(
                        VaultError::ProofError(e),
                    ))
                })?
        };

        let proof_id = system_api.create_node(RENode::Proof(proof))?.into();

        Ok((
            Proof(proof_id),
            CallFrameUpdate::move_node(RENodeId::Proof(proof_id)),
        ))
    }
}

impl ExecutableInvocation for VaultCreateProofByIdsInvocation {
    type Exec = TypedExecutor<Self>;

    fn prepare<D: MethodDeref>(
        self,
        _deref: &mut D,
    ) -> Result<(REActor, CallFrameUpdate, Self::Exec), RuntimeError> {
        let input = IndexedScryptoValue::from_typed(&self);
        let receiver = RENodeId::Vault(self.receiver);
        let call_frame_update = CallFrameUpdate::copy_ref(receiver);
        let actor = REActor::Method(
            ResolvedMethod::Native(NativeMethod::Vault(VaultMethod::CreateProofByIds)),
            ResolvedReceiver::new(receiver),
        );
        let executor = TypedExecutor(self, input);
        Ok((actor, call_frame_update, executor))
    }
}

impl NativeProgram for VaultCreateProofByIdsInvocation {
    type Output = Proof;

    fn main<'a, Y>(self, system_api: &mut Y) -> Result<(Proof, CallFrameUpdate), RuntimeError>
    where
        Y: SystemApi,
    {
        let node_id = RENodeId::Vault(self.receiver);
        let offset = SubstateOffset::Vault(VaultOffset::Vault);
        let vault_handle = system_api.lock_substate(node_id, offset, LockFlags::MUTABLE)?;

        let proof = {
            let mut substate_mut = system_api.get_ref_mut(vault_handle)?;
            let vault = substate_mut.vault();
            vault
                .create_proof_by_ids(&self.ids, ResourceContainerId::Vault(self.receiver))
                .map_err(|e| {
                    RuntimeError::ApplicationError(ApplicationError::VaultError(
                        VaultError::ProofError(e),
                    ))
                })?
        };

        let proof_id = system_api.create_node(RENode::Proof(proof))?.into();

        Ok((
            Proof(proof_id),
            CallFrameUpdate::move_node(RENodeId::Proof(proof_id)),
        ))
    }
}
