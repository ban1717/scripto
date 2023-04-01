use super::node_modules::access_rules::MethodAccessRulesSubstate;
use crate::blueprints::access_controller::AccessControllerSubstate;
use crate::blueprints::account::AccountSubstate;
use crate::blueprints::clock::ClockSubstate;
use crate::blueprints::epoch_manager::{EpochManagerSubstate, RegisteredValidatorsSubstate};
use crate::blueprints::epoch_manager::ValidatorSetSubstate;
use crate::blueprints::epoch_manager::ValidatorSubstate;
use crate::blueprints::package::PackageCodeTypeSubstate;
use crate::blueprints::resource::*;
use crate::errors::*;
use crate::system::node_modules::access_rules::FunctionAccessRulesSubstate;
use crate::system::node_modules::type_info::TypeInfoSubstate;
use crate::types::*;
use radix_engine_interface::api::component::*;
use radix_engine_interface::api::types::{
    ComponentOffset, KeyValueStoreOffset, RENodeId, SubstateOffset,
};
use radix_engine_interface::blueprints::package::*;
use radix_engine_interface::blueprints::resource::LiquidFungibleResource;
use radix_engine_interface::blueprints::resource::LiquidNonFungibleResource;
use radix_engine_interface::blueprints::resource::LockedFungibleResource;
use radix_engine_interface::blueprints::resource::LockedNonFungibleResource;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum PersistedSubstate {
    EpochManager(EpochManagerSubstate),
    RegisteredValidators(RegisteredValidatorsSubstate),
    ValidatorSet(ValidatorSetSubstate),
    Validator(ValidatorSubstate),
    CurrentTimeRoundedToMinutes(ClockSubstate),
    ResourceManager(FungibleResourceManagerSubstate),
    NonFungibleResourceManager(NonFungibleResourceManagerSubstate),
    ComponentState(ComponentStateSubstate),
    PackageInfo(PackageInfoSubstate),
    PackageCodeType(PackageCodeTypeSubstate),
    PackageCode(PackageCodeSubstate),
    PackageRoyalty(PackageRoyaltySubstate),
    FunctionAccessRules(FunctionAccessRulesSubstate),
    Account(AccountSubstate),
    AccessController(AccessControllerSubstate),
    FungibleVaultInfo(FungibleVaultDivisibilitySubstate),
    NonFungibleVaultInfo(NonFungibleVaultIdTypeSubstate),
    VaultLiquidFungible(LiquidFungibleResource),
    VaultLiquidNonFungible(LiquidNonFungibleResource),
    VaultLockedFungible(LockedFungibleResource),
    VaultLockedNonFungible(LockedNonFungibleResource),

    /* Type info */
    TypeInfo(TypeInfoSubstate),

    /* Access rules */
    MethodAccessRules(MethodAccessRulesSubstate),

    /* Royalty */
    ComponentRoyaltyConfig(ComponentRoyaltyConfigSubstate),
    ComponentRoyaltyAccumulator(ComponentRoyaltyAccumulatorSubstate),

    /* KVStore entry */
    KeyValueStoreEntry(Option<ScryptoValue>),
}

impl PersistedSubstate {
    pub fn fungible_vault_info(&self) -> &FungibleVaultDivisibilitySubstate {
        if let PersistedSubstate::FungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a fungible vault info");
        }
    }

    pub fn non_fungible_vault_info(&self) -> &NonFungibleVaultIdTypeSubstate {
        if let PersistedSubstate::NonFungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a vault info");
        }
    }

    pub fn vault_liquid_fungible(&self) -> &LiquidFungibleResource {
        if let PersistedSubstate::VaultLiquidFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault liquid fungible");
        }
    }

    pub fn vault_liquid_fungible_mut(&mut self) -> &mut LiquidFungibleResource {
        if let PersistedSubstate::VaultLiquidFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault liquid fungible");
        }
    }

    pub fn vault_liquid_non_fungible(&self) -> &LiquidNonFungibleResource {
        if let PersistedSubstate::VaultLiquidNonFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault liquid non-fungible");
        }
    }

    pub fn vault_liquid_non_fungible_mut(&mut self) -> &mut LiquidNonFungibleResource {
        if let PersistedSubstate::VaultLiquidNonFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault liquid non-fungible");
        }
    }

    pub fn vault_info_mut(&mut self) -> &mut NonFungibleVaultIdTypeSubstate {
        if let PersistedSubstate::NonFungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a vault info");
        }
    }

    pub fn component_royalty_accumulator(&self) -> &ComponentRoyaltyAccumulatorSubstate {
        if let PersistedSubstate::ComponentRoyaltyAccumulator(state) = self {
            state
        } else {
            panic!("Not a component royalty accumulator");
        }
    }

    pub fn package_royalty(&self) -> &PackageRoyaltySubstate {
        if let PersistedSubstate::PackageRoyalty(state) = self {
            state
        } else {
            panic!("Not a package royalty");
        }
    }

    pub fn package_info(&self) -> &PackageInfoSubstate {
        if let PersistedSubstate::PackageInfo(info) = self {
            info
        } else {
            panic!("Not a package royalty accumulator");
        }
    }

    pub fn package_code(&self) -> &PackageCodeSubstate {
        if let PersistedSubstate::PackageCode(code) = self {
            code
        } else {
            panic!("Not a package royalty accumulator");
        }
    }

    pub fn type_info(&self) -> &TypeInfoSubstate {
        if let PersistedSubstate::TypeInfo(info) = self {
            info
        } else {
            panic!("Not a package royalty accumulator");
        }
    }

    pub fn resource_manager(&self) -> &FungibleResourceManagerSubstate {
        if let PersistedSubstate::ResourceManager(state) = self {
            state
        } else {
            panic!("Not a resource manager substate");
        }
    }
}

impl Into<TypeInfoSubstate> for PersistedSubstate {
    fn into(self) -> TypeInfoSubstate {
        if let PersistedSubstate::TypeInfo(type_info) = self {
            type_info
        } else {
            panic!("Not type info");
        }
    }
}

impl Into<FungibleVaultDivisibilitySubstate> for PersistedSubstate {
    fn into(self) -> FungibleVaultDivisibilitySubstate {
        if let PersistedSubstate::FungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a fungible vault");
        }
    }
}

impl Into<NonFungibleVaultIdTypeSubstate> for PersistedSubstate {
    fn into(self) -> NonFungibleVaultIdTypeSubstate {
        if let PersistedSubstate::NonFungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }
}

impl Into<LiquidFungibleResource> for PersistedSubstate {
    fn into(self) -> LiquidFungibleResource {
        if let PersistedSubstate::VaultLiquidFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }
}

impl Into<LiquidNonFungibleResource> for PersistedSubstate {
    fn into(self) -> LiquidNonFungibleResource {
        if let PersistedSubstate::VaultLiquidNonFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }
}

impl PersistedSubstate {
    pub fn to_runtime(self) -> RuntimeSubstate {
        match self {
            PersistedSubstate::EpochManager(value) => RuntimeSubstate::EpochManager(value),
            PersistedSubstate::RegisteredValidators(value) => RuntimeSubstate::RegisteredValidators(value),
            PersistedSubstate::ValidatorSet(value) => RuntimeSubstate::ValidatorSet(value),
            PersistedSubstate::Validator(value) => RuntimeSubstate::Validator(value),
            PersistedSubstate::CurrentTimeRoundedToMinutes(value) => {
                RuntimeSubstate::CurrentTimeRoundedToMinutes(value)
            }
            PersistedSubstate::ResourceManager(value) => RuntimeSubstate::ResourceManager(value),
            PersistedSubstate::NonFungibleResourceManager(value) => {
                RuntimeSubstate::NonFungibleResourceManager(value)
            }
            PersistedSubstate::ComponentState(value) => RuntimeSubstate::ComponentState(value),
            PersistedSubstate::PackageInfo(value) => RuntimeSubstate::PackageInfo(value),
            PersistedSubstate::PackageCodeType(value) => RuntimeSubstate::PackageCodeType(value),
            PersistedSubstate::PackageCode(value) => RuntimeSubstate::PackageCode(value),
            PersistedSubstate::PackageRoyalty(value) => RuntimeSubstate::PackageRoyalty(value),
            PersistedSubstate::FunctionAccessRules(value) => {
                RuntimeSubstate::FunctionAccessRules(value)
            }
            PersistedSubstate::FungibleVaultInfo(value) => {
                RuntimeSubstate::FungibleVaultInfo(value)
            }
            PersistedSubstate::NonFungibleVaultInfo(value) => {
                RuntimeSubstate::NonFungibleVaultInfo(value)
            }
            PersistedSubstate::VaultLiquidFungible(value) => {
                RuntimeSubstate::VaultLiquidFungible(value)
            }
            PersistedSubstate::VaultLiquidNonFungible(value) => {
                RuntimeSubstate::VaultLiquidNonFungible(value)
            }
            PersistedSubstate::VaultLockedFungible(value) => {
                RuntimeSubstate::VaultLockedFungible(value)
            }
            PersistedSubstate::VaultLockedNonFungible(value) => {
                RuntimeSubstate::VaultLockedNonFungible(value)
            }
            PersistedSubstate::KeyValueStoreEntry(value) => {
                RuntimeSubstate::KeyValueStoreEntry(value)
            }
            PersistedSubstate::Account(value) => RuntimeSubstate::Account(value),
            PersistedSubstate::AccessController(value) => RuntimeSubstate::AccessController(value),

            /* Node module starts */
            PersistedSubstate::TypeInfo(value) => RuntimeSubstate::TypeInfo(value),
            PersistedSubstate::MethodAccessRules(value) => {
                RuntimeSubstate::MethodAccessRules(value)
            }
            PersistedSubstate::ComponentRoyaltyConfig(value) => {
                RuntimeSubstate::ComponentRoyaltyConfig(value)
            }
            PersistedSubstate::ComponentRoyaltyAccumulator(value) => {
                RuntimeSubstate::ComponentRoyaltyAccumulator(value)
            }
        }
    }
}

#[derive(Debug)]
pub enum RuntimeSubstate {
    EpochManager(EpochManagerSubstate),
    RegisteredValidators(RegisteredValidatorsSubstate),
    ValidatorSet(ValidatorSetSubstate),
    Validator(ValidatorSubstate),
    CurrentTimeRoundedToMinutes(ClockSubstate),
    ResourceManager(FungibleResourceManagerSubstate),
    NonFungibleResourceManager(NonFungibleResourceManagerSubstate),
    ComponentState(ComponentStateSubstate),
    PackageCode(PackageCodeSubstate),
    PackageInfo(PackageInfoSubstate),
    PackageCodeType(PackageCodeTypeSubstate),
    PackageRoyalty(PackageRoyaltySubstate),
    FunctionAccessRules(FunctionAccessRulesSubstate),
    Worktop(WorktopSubstate),
    AuthZone(AuthZone),
    Account(AccountSubstate),
    AccessController(AccessControllerSubstate),

    // TODO: we may want to move some of the static info into `TypeInfo`
    // And split the "Blueprint" into fungible and non-fungible.
    FungibleVaultInfo(FungibleVaultDivisibilitySubstate),
    NonFungibleVaultInfo(NonFungibleVaultIdTypeSubstate),
    VaultLiquidFungible(LiquidFungibleResource),
    VaultLiquidNonFungible(LiquidNonFungibleResource),
    VaultLockedFungible(LockedFungibleResource),
    VaultLockedNonFungible(LockedNonFungibleResource),

    BucketInfo(BucketInfoSubstate),
    BucketLiquidFungible(LiquidFungibleResource),
    BucketLiquidNonFungible(LiquidNonFungibleResource),
    BucketLockedFungible(LockedFungibleResource),
    BucketLockedNonFungible(LockedNonFungibleResource),

    ProofInfo(ProofInfoSubstate),
    FungibleProof(FungibleProof),
    NonFungibleProof(NonFungibleProof),

    /* Type info */
    TypeInfo(TypeInfoSubstate),

    /* Access rules */
    MethodAccessRules(MethodAccessRulesSubstate),

    /* Royalty */
    ComponentRoyaltyConfig(ComponentRoyaltyConfigSubstate),
    ComponentRoyaltyAccumulator(ComponentRoyaltyAccumulatorSubstate),

    /* KVStore entry */
    KeyValueStoreEntry(Option<ScryptoValue>),
}

impl RuntimeSubstate {
    pub fn clone_to_persisted(&self) -> PersistedSubstate {
        match self {
            RuntimeSubstate::EpochManager(value) => PersistedSubstate::EpochManager(value.clone()),
            RuntimeSubstate::RegisteredValidators(value) => PersistedSubstate::RegisteredValidators(value.clone()),
            RuntimeSubstate::ValidatorSet(value) => PersistedSubstate::ValidatorSet(value.clone()),
            RuntimeSubstate::Validator(value) => PersistedSubstate::Validator(value.clone()),
            RuntimeSubstate::CurrentTimeRoundedToMinutes(value) => {
                PersistedSubstate::CurrentTimeRoundedToMinutes(value.clone())
            }
            RuntimeSubstate::ResourceManager(value) => {
                PersistedSubstate::ResourceManager(value.clone())
            }
            RuntimeSubstate::NonFungibleResourceManager(value) => {
                PersistedSubstate::NonFungibleResourceManager(value.clone())
            }
            RuntimeSubstate::ComponentState(value) => {
                PersistedSubstate::ComponentState(value.clone())
            }
            RuntimeSubstate::PackageInfo(value) => PersistedSubstate::PackageInfo(value.clone()),
            RuntimeSubstate::PackageCodeType(value) => {
                PersistedSubstate::PackageCodeType(value.clone())
            }
            RuntimeSubstate::PackageCode(value) => PersistedSubstate::PackageCode(value.clone()),
            RuntimeSubstate::PackageRoyalty(value) => {
                PersistedSubstate::PackageRoyalty(value.clone())
            }
            RuntimeSubstate::FunctionAccessRules(value) => {
                PersistedSubstate::FunctionAccessRules(value.clone())
            }
            RuntimeSubstate::KeyValueStoreEntry(value) => {
                PersistedSubstate::KeyValueStoreEntry(value.clone())
            }
            RuntimeSubstate::FungibleVaultInfo(value) => {
                PersistedSubstate::FungibleVaultInfo(value.clone())
            }
            RuntimeSubstate::NonFungibleVaultInfo(value) => {
                PersistedSubstate::NonFungibleVaultInfo(value.clone())
            }
            RuntimeSubstate::VaultLiquidFungible(value) => {
                PersistedSubstate::VaultLiquidFungible(value.clone())
            }
            RuntimeSubstate::VaultLiquidNonFungible(value) => {
                PersistedSubstate::VaultLiquidNonFungible(value.clone())
            }
            RuntimeSubstate::Account(value) => PersistedSubstate::Account(value.clone()),
            RuntimeSubstate::AccessController(value) => {
                PersistedSubstate::AccessController(value.clone())
            }

            /* Node module starts */
            RuntimeSubstate::TypeInfo(value) => PersistedSubstate::TypeInfo(value.clone()),
            RuntimeSubstate::MethodAccessRules(value) => {
                PersistedSubstate::MethodAccessRules(value.clone())
            }
            RuntimeSubstate::ComponentRoyaltyConfig(value) => {
                PersistedSubstate::ComponentRoyaltyConfig(value.clone())
            }
            RuntimeSubstate::ComponentRoyaltyAccumulator(value) => {
                PersistedSubstate::ComponentRoyaltyAccumulator(value.clone())
            }
            /* Node module ends */
            RuntimeSubstate::BucketInfo(..)
            | RuntimeSubstate::BucketLiquidFungible(..)
            | RuntimeSubstate::BucketLiquidNonFungible(..)
            | RuntimeSubstate::BucketLockedFungible(..)
            | RuntimeSubstate::BucketLockedNonFungible(..)
            | RuntimeSubstate::VaultLockedFungible(..)
            | RuntimeSubstate::VaultLockedNonFungible(..)
            | RuntimeSubstate::ProofInfo(..)
            | RuntimeSubstate::FungibleProof(..)
            | RuntimeSubstate::NonFungibleProof(..)
            | RuntimeSubstate::Worktop(..)
            | RuntimeSubstate::AuthZone(..) => {
                panic!("Should not get here");
            }
        }
    }

    pub fn to_persisted(self) -> PersistedSubstate {
        match self {
            RuntimeSubstate::EpochManager(value) => PersistedSubstate::EpochManager(value),
            RuntimeSubstate::RegisteredValidators(value) => PersistedSubstate::RegisteredValidators(value),
            RuntimeSubstate::ValidatorSet(value) => PersistedSubstate::ValidatorSet(value),
            RuntimeSubstate::Validator(value) => PersistedSubstate::Validator(value),
            RuntimeSubstate::CurrentTimeRoundedToMinutes(value) => {
                PersistedSubstate::CurrentTimeRoundedToMinutes(value)
            }
            RuntimeSubstate::ResourceManager(value) => PersistedSubstate::ResourceManager(value),
            RuntimeSubstate::NonFungibleResourceManager(value) => {
                PersistedSubstate::NonFungibleResourceManager(value)
            }
            RuntimeSubstate::ComponentState(value) => PersistedSubstate::ComponentState(value),
            RuntimeSubstate::PackageInfo(value) => PersistedSubstate::PackageInfo(value),
            RuntimeSubstate::PackageCodeType(value) => PersistedSubstate::PackageCodeType(value),
            RuntimeSubstate::PackageCode(value) => PersistedSubstate::PackageCode(value),
            RuntimeSubstate::PackageRoyalty(value) => PersistedSubstate::PackageRoyalty(value),
            RuntimeSubstate::FunctionAccessRules(value) => {
                PersistedSubstate::FunctionAccessRules(value)
            }
            RuntimeSubstate::KeyValueStoreEntry(value) => {
                PersistedSubstate::KeyValueStoreEntry(value)
            }
            RuntimeSubstate::FungibleVaultInfo(value) => {
                PersistedSubstate::FungibleVaultInfo(value)
            }
            RuntimeSubstate::NonFungibleVaultInfo(value) => {
                PersistedSubstate::NonFungibleVaultInfo(value)
            }
            RuntimeSubstate::VaultLiquidFungible(value) => {
                PersistedSubstate::VaultLiquidFungible(value)
            }
            RuntimeSubstate::VaultLiquidNonFungible(value) => {
                PersistedSubstate::VaultLiquidNonFungible(value)
            }
            RuntimeSubstate::VaultLockedFungible(value) => {
                PersistedSubstate::VaultLockedFungible(value)
            }
            RuntimeSubstate::VaultLockedNonFungible(value) => {
                PersistedSubstate::VaultLockedNonFungible(value)
            }
            RuntimeSubstate::Account(value) => PersistedSubstate::Account(value),
            RuntimeSubstate::AccessController(value) => PersistedSubstate::AccessController(value),

            /* Node module starts */
            RuntimeSubstate::TypeInfo(value) => PersistedSubstate::TypeInfo(value),
            RuntimeSubstate::MethodAccessRules(value) => {
                PersistedSubstate::MethodAccessRules(value)
            }
            RuntimeSubstate::ComponentRoyaltyConfig(value) => {
                PersistedSubstate::ComponentRoyaltyConfig(value)
            }
            RuntimeSubstate::ComponentRoyaltyAccumulator(value) => {
                PersistedSubstate::ComponentRoyaltyAccumulator(value)
            }
            /* Node module ends */
            RuntimeSubstate::BucketInfo(..)
            | RuntimeSubstate::BucketLiquidFungible(..)
            | RuntimeSubstate::BucketLiquidNonFungible(..)
            | RuntimeSubstate::BucketLockedFungible(..)
            | RuntimeSubstate::BucketLockedNonFungible(..)
            | RuntimeSubstate::ProofInfo(..)
            | RuntimeSubstate::FungibleProof(..)
            | RuntimeSubstate::NonFungibleProof(..)
            | RuntimeSubstate::Worktop(..)
            | RuntimeSubstate::AuthZone(..) => {
                panic!("Should not get here");
            }
        }
    }

    pub fn decode_from_buffer(
        offset: &SubstateOffset,
        buffer: &[u8],
    ) -> Result<Self, RuntimeError> {
        let substate = match offset {
            SubstateOffset::Component(ComponentOffset::State0) => {
                let substate =
                    scrypto_decode(buffer).map_err(|e| KernelError::SborDecodeError(e))?;
                RuntimeSubstate::ComponentState(substate)
            }
            SubstateOffset::KeyValueStore(KeyValueStoreOffset::Entry(..)) => {
                let substate =
                    scrypto_decode(buffer).map_err(|e| KernelError::SborDecodeError(e))?;
                RuntimeSubstate::KeyValueStoreEntry(substate)
            }
            offset => {
                return Err(RuntimeError::KernelError(KernelError::InvalidOffset(
                    offset.clone(),
                )))
            }
        };

        Ok(substate)
    }

    pub fn to_ref_mut(&mut self) -> SubstateRefMut {
        match self {
            RuntimeSubstate::EpochManager(value) => SubstateRefMut::EpochManager(value),
            RuntimeSubstate::RegisteredValidators(value) => SubstateRefMut::RegisteredValidators(value),
            RuntimeSubstate::ValidatorSet(value) => SubstateRefMut::ValidatorSet(value),
            RuntimeSubstate::Validator(value) => SubstateRefMut::Validator(value),
            RuntimeSubstate::CurrentTimeRoundedToMinutes(value) => {
                SubstateRefMut::CurrentTimeRoundedToMinutes(value)
            }
            RuntimeSubstate::MethodAccessRules(value) => SubstateRefMut::MethodAccessRules(value),
            RuntimeSubstate::ResourceManager(value) => SubstateRefMut::ResourceManager(value),
            RuntimeSubstate::NonFungibleResourceManager(value) => {
                SubstateRefMut::NonFungibleResourceManager(value)
            }
            RuntimeSubstate::TypeInfo(value) => SubstateRefMut::TypeInfo(value),
            RuntimeSubstate::ComponentState(value) => SubstateRefMut::ComponentState(value),
            RuntimeSubstate::ComponentRoyaltyConfig(value) => {
                SubstateRefMut::ComponentRoyaltyConfig(value)
            }
            RuntimeSubstate::ComponentRoyaltyAccumulator(value) => {
                SubstateRefMut::ComponentRoyaltyAccumulator(value)
            }
            RuntimeSubstate::PackageInfo(value) => SubstateRefMut::PackageInfo(value),
            RuntimeSubstate::FunctionAccessRules(value) => {
                SubstateRefMut::PackageAccessRules(value)
            }
            RuntimeSubstate::PackageCodeType(value) => SubstateRefMut::PackageCodeType(value),
            RuntimeSubstate::PackageCode(value) => SubstateRefMut::PackageCode(value),
            RuntimeSubstate::PackageRoyalty(value) => SubstateRefMut::PackageRoyalty(value),
            RuntimeSubstate::FungibleVaultInfo(value) => SubstateRefMut::FungibleVaultInfo(value),
            RuntimeSubstate::NonFungibleVaultInfo(value) => {
                SubstateRefMut::NonFungibleVaultInfo(value)
            }
            RuntimeSubstate::VaultLiquidFungible(value) => {
                SubstateRefMut::VaultLiquidFungible(value)
            }
            RuntimeSubstate::VaultLiquidNonFungible(value) => {
                SubstateRefMut::VaultLiquidNonFungible(value)
            }
            RuntimeSubstate::VaultLockedFungible(value) => {
                SubstateRefMut::VaultLockedFungible(value)
            }
            RuntimeSubstate::VaultLockedNonFungible(value) => {
                SubstateRefMut::VaultLockedNonFungible(value)
            }
            RuntimeSubstate::BucketInfo(value) => SubstateRefMut::BucketInfo(value),
            RuntimeSubstate::BucketLiquidFungible(value) => {
                SubstateRefMut::BucketLiquidFungible(value)
            }
            RuntimeSubstate::BucketLiquidNonFungible(value) => {
                SubstateRefMut::BucketLiquidNonFungible(value)
            }
            RuntimeSubstate::BucketLockedFungible(value) => {
                SubstateRefMut::BucketLockedFungible(value)
            }
            RuntimeSubstate::BucketLockedNonFungible(value) => {
                SubstateRefMut::BucketLockedNonFungible(value)
            }
            RuntimeSubstate::ProofInfo(value) => SubstateRefMut::ProofInfo(value),
            RuntimeSubstate::FungibleProof(value) => SubstateRefMut::FungibleProof(value),
            RuntimeSubstate::NonFungibleProof(value) => SubstateRefMut::NonFungibleProof(value),
            RuntimeSubstate::KeyValueStoreEntry(value) => SubstateRefMut::KeyValueStoreEntry(value),
            RuntimeSubstate::Worktop(value) => SubstateRefMut::Worktop(value),
            RuntimeSubstate::AuthZone(value) => SubstateRefMut::AuthZone(value),
            RuntimeSubstate::Account(value) => SubstateRefMut::Account(value),
            RuntimeSubstate::AccessController(value) => SubstateRefMut::AccessController(value),
        }
    }

    pub fn to_ref(&self) -> SubstateRef {
        match self {
            RuntimeSubstate::TypeInfo(value) => SubstateRef::TypeInfo(value),
            RuntimeSubstate::EpochManager(value) => SubstateRef::EpochManager(value),
            RuntimeSubstate::RegisteredValidators(value) => SubstateRef::RegisteredValidators(value),
            RuntimeSubstate::ValidatorSet(value) => SubstateRef::ValidatorSet(value),
            RuntimeSubstate::Validator(value) => SubstateRef::Validator(value),
            RuntimeSubstate::CurrentTimeRoundedToMinutes(value) => {
                SubstateRef::CurrentTimeRoundedToMinutes(value)
            }
            RuntimeSubstate::MethodAccessRules(value) => SubstateRef::MethodAccessRules(value),
            RuntimeSubstate::ResourceManager(value) => SubstateRef::FungibleResourceManager(value),
            RuntimeSubstate::NonFungibleResourceManager(value) => {
                SubstateRef::NonFungibleResourceManager(value)
            }
            RuntimeSubstate::ComponentState(value) => SubstateRef::ComponentState(value),
            RuntimeSubstate::ComponentRoyaltyConfig(value) => {
                SubstateRef::ComponentRoyaltyConfig(value)
            }
            RuntimeSubstate::ComponentRoyaltyAccumulator(value) => {
                SubstateRef::ComponentRoyaltyAccumulator(value)
            }
            RuntimeSubstate::PackageInfo(value) => SubstateRef::PackageInfo(value),
            RuntimeSubstate::PackageCodeType(value) => SubstateRef::PackageCodeType(value),
            RuntimeSubstate::FunctionAccessRules(value) => SubstateRef::PackageAccessRules(value),
            RuntimeSubstate::PackageCode(value) => SubstateRef::PackageCode(value),
            RuntimeSubstate::PackageRoyalty(value) => SubstateRef::PackageRoyalty(value),
            RuntimeSubstate::FungibleVaultInfo(value) => SubstateRef::FungibleVaultInfo(value),
            RuntimeSubstate::NonFungibleVaultInfo(value) => {
                SubstateRef::NonFungibleVaultInfo(value)
            }
            RuntimeSubstate::VaultLiquidFungible(value) => SubstateRef::VaultLiquidFungible(value),
            RuntimeSubstate::VaultLiquidNonFungible(value) => {
                SubstateRef::VaultLiquidNonFungible(value)
            }
            RuntimeSubstate::VaultLockedFungible(value) => SubstateRef::VaultLockedFungible(value),
            RuntimeSubstate::VaultLockedNonFungible(value) => {
                SubstateRef::VaultLockedNonFungible(value)
            }
            RuntimeSubstate::BucketInfo(value) => SubstateRef::BucketInfo(value),
            RuntimeSubstate::BucketLiquidFungible(value) => {
                SubstateRef::BucketLiquidFungible(value)
            }
            RuntimeSubstate::BucketLiquidNonFungible(value) => {
                SubstateRef::BucketLiquidNonFungible(value)
            }
            RuntimeSubstate::BucketLockedFungible(value) => {
                SubstateRef::BucketLockedFungible(value)
            }
            RuntimeSubstate::BucketLockedNonFungible(value) => {
                SubstateRef::BucketLockedNonFungible(value)
            }
            RuntimeSubstate::ProofInfo(value) => SubstateRef::ProofInfo(value),
            RuntimeSubstate::FungibleProof(value) => SubstateRef::FungibleProof(value),
            RuntimeSubstate::NonFungibleProof(value) => SubstateRef::NonFungibleProof(value),
            RuntimeSubstate::KeyValueStoreEntry(value) => SubstateRef::KeyValueStoreEntry(value),
            RuntimeSubstate::Worktop(value) => SubstateRef::Worktop(value),
            RuntimeSubstate::AuthZone(value) => SubstateRef::AuthZone(value),
            RuntimeSubstate::Account(value) => SubstateRef::Account(value),
            RuntimeSubstate::AccessController(value) => SubstateRef::AccessController(value),
        }
    }

    pub fn vault_info_mut(&mut self) -> &mut NonFungibleVaultIdTypeSubstate {
        if let RuntimeSubstate::NonFungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }

    pub fn vault_liquid_fungible_mut(&mut self) -> &mut LiquidFungibleResource {
        if let RuntimeSubstate::VaultLiquidFungible(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }

    pub fn package_info(&self) -> &PackageInfoSubstate {
        if let RuntimeSubstate::PackageInfo(package) = self {
            package
        } else {
            panic!("Not a package info");
        }
    }

    pub fn kv_store_entry(&self) -> &Option<ScryptoValue> {
        if let RuntimeSubstate::KeyValueStoreEntry(kv_store_entry) = self {
            kv_store_entry
        } else {
            panic!("Not a KVEntry");
        }
    }

    pub fn validator_set(&self) -> &ValidatorSetSubstate {
        if let RuntimeSubstate::ValidatorSet(validator_set) = self {
            validator_set
        } else {
            panic!("Not a validator set");
        }
    }

    pub fn account(&self) -> &AccountSubstate {
        if let RuntimeSubstate::Account(account) = self {
            account
        } else {
            panic!("Not an account");
        }
    }

    pub fn method_access_rules(&self) -> &MethodAccessRulesSubstate {
        if let RuntimeSubstate::MethodAccessRules(method_access_rules) = self {
            method_access_rules
        } else {
            panic!("Not an access rules chain");
        }
    }

    pub fn access_controller(&self) -> &AccessControllerSubstate {
        if let RuntimeSubstate::AccessController(access_controller) = self {
            access_controller
        } else {
            panic!("Not an access controller");
        }
    }
}

impl Into<RuntimeSubstate> for MethodAccessRulesSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::MethodAccessRules(self)
    }
}

impl Into<RuntimeSubstate> for EpochManagerSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::EpochManager(self)
    }
}

impl Into<RuntimeSubstate> for ValidatorSetSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::ValidatorSet(self)
    }
}

impl Into<RuntimeSubstate> for ValidatorSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::Validator(self)
    }
}

impl Into<RuntimeSubstate> for ClockSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::CurrentTimeRoundedToMinutes(self)
    }
}

impl Into<RuntimeSubstate> for PackageInfoSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::PackageInfo(self)
    }
}

impl Into<RuntimeSubstate> for PackageCodeTypeSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::PackageCodeType(self)
    }
}
impl Into<RuntimeSubstate> for FunctionAccessRulesSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::FunctionAccessRules(self)
    }
}

impl Into<RuntimeSubstate> for PackageCodeSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::PackageCode(self)
    }
}

impl Into<RuntimeSubstate> for TypeInfoSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::TypeInfo(self)
    }
}

impl Into<RuntimeSubstate> for ComponentStateSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::ComponentState(self)
    }
}

impl Into<RuntimeSubstate> for FungibleResourceManagerSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::ResourceManager(self)
    }
}

impl Into<RuntimeSubstate> for ComponentRoyaltyConfigSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::ComponentRoyaltyConfig(self)
    }
}

impl Into<RuntimeSubstate> for ComponentRoyaltyAccumulatorSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::ComponentRoyaltyAccumulator(self)
    }
}

impl Into<RuntimeSubstate> for NonFungibleVaultIdTypeSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::NonFungibleVaultInfo(self)
    }
}

impl Into<RuntimeSubstate> for Option<ScryptoValue> {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::KeyValueStoreEntry(self)
    }
}

impl Into<RuntimeSubstate> for PackageRoyaltySubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::PackageRoyalty(self)
    }
}

impl Into<RuntimeSubstate> for AccountSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::Account(self)
    }
}

impl Into<RuntimeSubstate> for AccessControllerSubstate {
    fn into(self) -> RuntimeSubstate {
        RuntimeSubstate::AccessController(self)
    }
}

impl Into<WorktopSubstate> for RuntimeSubstate {
    fn into(self) -> WorktopSubstate {
        if let RuntimeSubstate::Worktop(component) = self {
            component
        } else {
            panic!("Not a worktop");
        }
    }
}

impl Into<TypeInfoSubstate> for RuntimeSubstate {
    fn into(self) -> TypeInfoSubstate {
        if let RuntimeSubstate::TypeInfo(component) = self {
            component
        } else {
            panic!("Not a component info");
        }
    }
}

impl Into<ComponentStateSubstate> for RuntimeSubstate {
    fn into(self) -> ComponentStateSubstate {
        if let RuntimeSubstate::ComponentState(component_state) = self {
            component_state
        } else {
            panic!("Not a component");
        }
    }
}

impl Into<ComponentRoyaltyConfigSubstate> for RuntimeSubstate {
    fn into(self) -> ComponentRoyaltyConfigSubstate {
        if let RuntimeSubstate::ComponentRoyaltyConfig(config) = self {
            config
        } else {
            panic!("Not a component royalty config");
        }
    }
}

impl Into<ComponentRoyaltyAccumulatorSubstate> for RuntimeSubstate {
    fn into(self) -> ComponentRoyaltyAccumulatorSubstate {
        if let RuntimeSubstate::ComponentRoyaltyAccumulator(vault) = self {
            vault
        } else {
            panic!("Not a component royalty accumulator");
        }
    }
}

impl Into<PackageRoyaltySubstate> for RuntimeSubstate {
    fn into(self) -> PackageRoyaltySubstate {
        if let RuntimeSubstate::PackageRoyalty(config) = self {
            config
        } else {
            panic!("Not a package royalty");
        }
    }
}

impl Into<FungibleResourceManagerSubstate> for RuntimeSubstate {
    fn into(self) -> FungibleResourceManagerSubstate {
        if let RuntimeSubstate::ResourceManager(resource_manager) = self {
            resource_manager
        } else {
            panic!("Not a resource manager");
        }
    }
}

impl Into<NonFungibleResourceManagerSubstate> for RuntimeSubstate {
    fn into(self) -> NonFungibleResourceManagerSubstate {
        if let RuntimeSubstate::NonFungibleResourceManager(resource_manager) = self {
            resource_manager
        } else {
            panic!("Not a non fungible resource manager");
        }
    }
}

impl Into<PackageCodeSubstate> for RuntimeSubstate {
    fn into(self) -> PackageCodeSubstate {
        if let RuntimeSubstate::PackageCode(code) = self {
            code
        } else {
            panic!("Not a wasm code");
        }
    }
}

impl Into<Option<ScryptoValue>> for RuntimeSubstate {
    fn into(self) -> Option<ScryptoValue> {
        if let RuntimeSubstate::KeyValueStoreEntry(kv_store_entry) = self {
            kv_store_entry
        } else {
            panic!("Not a key value store entry wrapper");
        }
    }
}

impl Into<NonFungibleVaultIdTypeSubstate> for RuntimeSubstate {
    fn into(self) -> NonFungibleVaultIdTypeSubstate {
        if let RuntimeSubstate::NonFungibleVaultInfo(vault) = self {
            vault
        } else {
            panic!("Not a vault");
        }
    }
}

impl Into<BucketInfoSubstate> for RuntimeSubstate {
    fn into(self) -> BucketInfoSubstate {
        if let RuntimeSubstate::BucketInfo(substate) = self {
            substate
        } else {
            panic!("Not a bucket");
        }
    }
}

impl Into<LiquidFungibleResource> for RuntimeSubstate {
    fn into(self) -> LiquidFungibleResource {
        if let RuntimeSubstate::VaultLiquidFungible(substate) = self {
            substate
        } else if let RuntimeSubstate::BucketLiquidFungible(substate) = self {
            substate
        } else {
            panic!("Not a vault");
        }
    }
}

impl Into<LiquidNonFungibleResource> for RuntimeSubstate {
    fn into(self) -> LiquidNonFungibleResource {
        if let RuntimeSubstate::VaultLiquidNonFungible(substate) = self {
            substate
        } else if let RuntimeSubstate::BucketLiquidNonFungible(substate) = self {
            substate
        } else {
            panic!("Not a vault");
        }
    }
}

impl Into<EpochManagerSubstate> for RuntimeSubstate {
    fn into(self) -> EpochManagerSubstate {
        if let RuntimeSubstate::EpochManager(system) = self {
            system
        } else {
            panic!("Not an epoch manager ");
        }
    }
}

impl Into<ValidatorSubstate> for RuntimeSubstate {
    fn into(self) -> ValidatorSubstate {
        if let RuntimeSubstate::Validator(validator) = self {
            validator
        } else {
            panic!("Not a validator");
        }
    }
}

impl Into<ProofInfoSubstate> for RuntimeSubstate {
    fn into(self) -> ProofInfoSubstate {
        if let RuntimeSubstate::ProofInfo(substate) = self {
            substate
        } else {
            panic!("Not a proof");
        }
    }
}

impl Into<FungibleProof> for RuntimeSubstate {
    fn into(self) -> FungibleProof {
        if let RuntimeSubstate::FungibleProof(substate) = self {
            substate
        } else {
            panic!("Not a fungible proof");
        }
    }
}

impl Into<NonFungibleProof> for RuntimeSubstate {
    fn into(self) -> NonFungibleProof {
        if let RuntimeSubstate::NonFungibleProof(substate) = self {
            substate
        } else {
            panic!("Not a non fungible proof");
        }
    }
}

impl Into<MethodAccessRulesSubstate> for RuntimeSubstate {
    fn into(self) -> MethodAccessRulesSubstate {
        if let RuntimeSubstate::MethodAccessRules(substate) = self {
            substate
        } else {
            panic!("Not access rules");
        }
    }
}

impl Into<ValidatorSetSubstate> for RuntimeSubstate {
    fn into(self) -> ValidatorSetSubstate {
        if let RuntimeSubstate::ValidatorSet(substate) = self {
            substate
        } else {
            panic!("Not a validator set");
        }
    }
}

pub enum SubstateRef<'a> {
    TypeInfo(&'a TypeInfoSubstate),
    Worktop(&'a WorktopSubstate),
    AuthZone(&'a AuthZone),
    ComponentInfo(&'a TypeInfoSubstate),
    ComponentState(&'a ComponentStateSubstate),
    ComponentRoyaltyConfig(&'a ComponentRoyaltyConfigSubstate),
    ComponentRoyaltyAccumulator(&'a ComponentRoyaltyAccumulatorSubstate),
    KeyValueStoreEntry(&'a Option<ScryptoValue>),
    PackageInfo(&'a PackageInfoSubstate),
    PackageCodeType(&'a PackageCodeTypeSubstate),
    PackageCode(&'a PackageCodeSubstate),
    PackageRoyalty(&'a PackageRoyaltySubstate),
    FungibleVaultInfo(&'a FungibleVaultDivisibilitySubstate),
    NonFungibleVaultInfo(&'a NonFungibleVaultIdTypeSubstate),
    VaultLiquidFungible(&'a LiquidFungibleResource),
    VaultLiquidNonFungible(&'a LiquidNonFungibleResource),
    VaultLockedFungible(&'a LockedFungibleResource),
    VaultLockedNonFungible(&'a LockedNonFungibleResource),
    BucketInfo(&'a BucketInfoSubstate),
    BucketLiquidFungible(&'a LiquidFungibleResource),
    BucketLiquidNonFungible(&'a LiquidNonFungibleResource),
    BucketLockedFungible(&'a LockedFungibleResource),
    BucketLockedNonFungible(&'a LockedNonFungibleResource),
    ProofInfo(&'a ProofInfoSubstate),
    FungibleProof(&'a FungibleProof),
    NonFungibleProof(&'a NonFungibleProof),
    FungibleResourceManager(&'a FungibleResourceManagerSubstate),
    NonFungibleResourceManager(&'a NonFungibleResourceManagerSubstate),
    EpochManager(&'a EpochManagerSubstate),
    RegisteredValidators(&'a RegisteredValidatorsSubstate),
    ValidatorSet(&'a ValidatorSetSubstate),
    Validator(&'a ValidatorSubstate),
    CurrentTimeRoundedToMinutes(&'a ClockSubstate),
    MethodAccessRules(&'a MethodAccessRulesSubstate),
    PackageAccessRules(&'a FunctionAccessRulesSubstate),
    Account(&'a AccountSubstate),
    AccessController(&'a AccessControllerSubstate),
}

impl<'a> From<SubstateRef<'a>> for &'a FungibleVaultDivisibilitySubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::FungibleVaultInfo(value) => value,
            _ => panic!("Not a FungibleVaultInfo"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a NonFungibleVaultIdTypeSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::NonFungibleVaultInfo(value) => value,
            _ => panic!("Not a VaultInfo"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a LiquidFungibleResource {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::VaultLiquidFungible(value) => value,
            SubstateRef::BucketLiquidFungible(value) => value,
            _ => panic!("Not a vault/bucket liquid fungible"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a LiquidNonFungibleResource {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::VaultLiquidNonFungible(value) => value,
            SubstateRef::BucketLiquidNonFungible(value) => value,
            _ => panic!("Not a vault/bucket liquid non-fungible"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a LockedFungibleResource {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::VaultLockedFungible(value) => value,
            SubstateRef::BucketLockedFungible(value) => value,
            _ => panic!("Not a vault/bucket locked fungible"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a LockedNonFungibleResource {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::VaultLockedNonFungible(value) => value,
            SubstateRef::BucketLockedNonFungible(value) => value,
            _ => panic!("Not a vault/bucket locked non-fungible"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a BucketInfoSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::BucketInfo(value) => value,
            _ => panic!("Not a BucketInfo"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ProofInfoSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::ProofInfo(value) => value,
            _ => panic!("Not a ProofInfo"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a FungibleProof {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::FungibleProof(value) => value,
            _ => panic!("Not a FungibleProof"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a NonFungibleProof {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::NonFungibleProof(value) => value,
            _ => panic!("Not a NonFungibleProof"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a TypeInfoSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::TypeInfo(value) => value,
            _ => panic!("Not a TypeInfo"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a EpochManagerSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::EpochManager(value) => value,
            _ => panic!("Not an epoch manager"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ValidatorSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::Validator(value) => value,
            _ => panic!("Not a validator"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ComponentStateSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::ComponentState(value) => value,
            _ => panic!("Not a component state"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ComponentRoyaltyConfigSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::ComponentRoyaltyConfig(value) => value,
            _ => panic!("Not a component royalty config"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ComponentRoyaltyAccumulatorSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::ComponentRoyaltyAccumulator(value) => value,
            _ => panic!("Not a component royalty accumulator"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a PackageCodeSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::PackageCode(value) => value,
            _ => panic!("Not a package code"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a PackageRoyaltySubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::PackageRoyalty(value) => value,
            _ => panic!("Not a package royalty"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a FunctionAccessRulesSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::PackageAccessRules(value) => value,
            _ => panic!("Not a package access rules"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a WorktopSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::Worktop(value) => value,
            _ => panic!("Not a worktop"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a AuthZone {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::AuthZone(value) => value,
            _ => panic!("Not a AuthZone"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a Option<ScryptoValue> {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::KeyValueStoreEntry(value) => value,
            _ => panic!("Not a kv entry"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a FungibleResourceManagerSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::FungibleResourceManager(value) => value,
            _ => panic!("Not a resource manager"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a NonFungibleResourceManagerSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::NonFungibleResourceManager(value) => value,
            _ => panic!("Not a non fungible resource manager"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a PackageInfoSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::PackageInfo(value) => value,
            _ => panic!("Not package info"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a PackageCodeTypeSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::PackageCodeType(value) => value,
            _ => panic!("Not package code type"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a MethodAccessRulesSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::MethodAccessRules(value) => value,
            _ => panic!("Not access rules chain"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a ClockSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::CurrentTimeRoundedToMinutes(value) => value,
            _ => panic!("Not current time"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a AccountSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::Account(value) => value,
            _ => panic!("Not an account"),
        }
    }
}

impl<'a> From<SubstateRef<'a>> for &'a AccessControllerSubstate {
    fn from(value: SubstateRef<'a>) -> Self {
        match value {
            SubstateRef::AccessController(value) => value,
            _ => panic!("Not an access controller"),
        }
    }
}

impl<'a> SubstateRef<'a> {
    pub fn to_scrypto_value(&self) -> IndexedScryptoValue {
        match self {
            SubstateRef::PackageCodeType(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::EpochManager(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::CurrentTimeRoundedToMinutes(value) => {
                IndexedScryptoValue::from_typed(*value)
            }
            SubstateRef::FungibleResourceManager(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::TypeInfo(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::ComponentState(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::ComponentRoyaltyConfig(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::ComponentRoyaltyAccumulator(value) => {
                IndexedScryptoValue::from_typed(*value)
            }
            SubstateRef::PackageInfo(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::PackageRoyalty(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::KeyValueStoreEntry(value) => IndexedScryptoValue::from_typed(*value),
            SubstateRef::MethodAccessRules(value) => IndexedScryptoValue::from_typed(*value),
            _ => panic!("Unsupported scrypto value"),
        }
    }

    // TODO: remove this method
    pub fn references_and_owned_nodes(&self) -> (IndexSet<RENodeId>, Vec<RENodeId>) {
        match self {
            SubstateRef::Worktop(worktop) => {
                let nodes = worktop
                    .resources
                    .values()
                    .map(|o| RENodeId::Object(o.bucket_id()))
                    .collect();
                (index_set_new(), nodes)
            }
            SubstateRef::FungibleVaultInfo(..) => (index_set_new(), Vec::new()),
            SubstateRef::NonFungibleVaultInfo(..) => (index_set_new(), Vec::new()),
            SubstateRef::ProofInfo(proof) => {
                let mut references = index_set_new();
                references.insert(RENodeId::GlobalObject(proof.resource_address.into()));
                (references, Vec::new())
            }
            SubstateRef::FungibleProof(proof) => {
                let mut references = index_set_new();
                for r in proof.evidence.keys() {
                    references.insert(r.to_re_node_id());
                }
                (references, Vec::new())
            }
            SubstateRef::NonFungibleProof(proof) => {
                let mut references = index_set_new();
                for r in proof.evidence.keys() {
                    references.insert(r.to_re_node_id());
                }
                (references, Vec::new())
            }
            SubstateRef::BucketInfo(bucket) => {
                let mut references = index_set_new();
                references.insert(RENodeId::GlobalObject(bucket.resource_address.into()));
                (references, Vec::new())
            }
            SubstateRef::PackageInfo(substate) => {
                let mut references = index_set_new();
                for component_ref in &substate.dependent_components {
                    references.insert(RENodeId::GlobalObject(component_ref.clone().into()));
                }
                for resource_ref in &substate.dependent_resources {
                    references.insert(RENodeId::GlobalObject(resource_ref.clone().into()));
                }
                (references, Vec::new())
            }
            SubstateRef::PackageRoyalty(substate) => {
                let mut owns = Vec::new();
                if let Some(vault) = substate.royalty_vault {
                    owns.push(RENodeId::Object(vault.id()));
                }
                (index_set_new(), owns)
            }
            SubstateRef::TypeInfo(substate) => {
                let mut references = index_set_new();
                match substate {
                    TypeInfoSubstate::Object(ObjectInfo {
                        blueprint,
                        type_parent: parent,
                        ..
                    }) => {
                        references.insert(RENodeId::GlobalObject(blueprint.package_address.into()));
                        if let Some(parent) = parent {
                            references.insert(RENodeId::GlobalObject(parent.clone()));
                        }
                    }
                    TypeInfoSubstate::KeyValueStore(..) => {}
                }
                (references, Vec::new())
            }
            SubstateRef::FungibleResourceManager(..) => (index_set_new(), Vec::new()),
            SubstateRef::NonFungibleResourceManager(substate) => {
                let mut owned_nodes = Vec::new();
                owned_nodes.push(RENodeId::KeyValueStore(substate.non_fungible_table));

                (index_set_new(), owned_nodes)
            }
            SubstateRef::EpochManager(substate) => {
                let (_, owns, refs) = IndexedScryptoValue::from_typed(&substate).unpack();
                (refs, owns)
            }
            SubstateRef::Validator(substate) => {
                let (_, owns, refs) = IndexedScryptoValue::from_typed(&substate).unpack();
                (refs, owns)
            }
            SubstateRef::MethodAccessRules(substate) => {
                let (_, owns, refs) = IndexedScryptoValue::from_typed(&substate).unpack();
                (refs, owns)
            }
            SubstateRef::AccessController(substate) => {
                let mut owned_nodes = Vec::new();
                owned_nodes.push(RENodeId::Object(substate.controlled_asset));
                (index_set_new(), owned_nodes)
            }
            SubstateRef::ComponentState(substate) => {
                let (_, owns, refs) =
                    IndexedScryptoValue::from_scrypto_value(substate.0.clone()).unpack();
                (refs, owns)
            }
            SubstateRef::ComponentRoyaltyAccumulator(substate) => {
                let mut owned_nodes = Vec::new();
                if let Some(vault) = substate.royalty_vault {
                    owned_nodes.push(RENodeId::Object(vault.vault_id()));
                }
                (index_set_new(), owned_nodes)
            }
            SubstateRef::KeyValueStoreEntry(substate) => {
                if let Some(substate) = substate {
                    let (_, own, refs) =
                        IndexedScryptoValue::from_scrypto_value(substate.clone()).unpack();
                    (refs, own)
                } else {
                    (index_set_new(), Vec::new())
                }
            }
            SubstateRef::Account(substate) => {
                let mut owned_nodes = Vec::new();
                owned_nodes.push(RENodeId::KeyValueStore(
                    substate.vaults.key_value_store_id(),
                ));
                (index_set_new(), owned_nodes)
            }
            SubstateRef::AuthZone(substate) => {
                let mut references = index_set_new();
                let mut owned_nodes = Vec::new();
                for proof in &substate.proofs {
                    owned_nodes.push(RENodeId::Object(proof.0));
                }
                if let Some(parent) = substate.parent {
                    references.insert(RENodeId::Object(parent.0));
                }
                (references, owned_nodes)
            }
            _ => (index_set_new(), Vec::new()),
        }
    }
}

pub enum SubstateRefMut<'a> {
    TypeInfo(&'a mut TypeInfoSubstate),
    ComponentState(&'a mut ComponentStateSubstate),
    ComponentRoyaltyConfig(&'a mut ComponentRoyaltyConfigSubstate),
    ComponentRoyaltyAccumulator(&'a mut ComponentRoyaltyAccumulatorSubstate),
    PackageInfo(&'a mut PackageInfoSubstate),
    PackageCodeType(&'a mut PackageCodeTypeSubstate),
    PackageCode(&'a mut PackageCodeSubstate),
    PackageRoyalty(&'a mut PackageRoyaltySubstate),
    PackageAccessRules(&'a mut FunctionAccessRulesSubstate),
    KeyValueStoreEntry(&'a mut Option<ScryptoValue>),
    FungibleVaultInfo(&'a mut FungibleVaultDivisibilitySubstate),
    NonFungibleVaultInfo(&'a mut NonFungibleVaultIdTypeSubstate),
    VaultLiquidFungible(&'a mut LiquidFungibleResource),
    VaultLiquidNonFungible(&'a mut LiquidNonFungibleResource),
    VaultLockedFungible(&'a mut LockedFungibleResource),
    VaultLockedNonFungible(&'a mut LockedNonFungibleResource),
    BucketInfo(&'a mut BucketInfoSubstate),
    BucketLiquidFungible(&'a mut LiquidFungibleResource),
    BucketLiquidNonFungible(&'a mut LiquidNonFungibleResource),
    BucketLockedFungible(&'a mut LockedFungibleResource),
    BucketLockedNonFungible(&'a mut LockedNonFungibleResource),
    ResourceManager(&'a mut FungibleResourceManagerSubstate),
    NonFungibleResourceManager(&'a mut NonFungibleResourceManagerSubstate),
    EpochManager(&'a mut EpochManagerSubstate),
    RegisteredValidators(&'a mut RegisteredValidatorsSubstate),
    ValidatorSet(&'a mut ValidatorSetSubstate),
    Validator(&'a mut ValidatorSubstate),
    CurrentTimeRoundedToMinutes(&'a mut ClockSubstate),
    MethodAccessRules(&'a mut MethodAccessRulesSubstate),
    ProofInfo(&'a mut ProofInfoSubstate),
    FungibleProof(&'a mut FungibleProof),
    NonFungibleProof(&'a mut NonFungibleProof),
    Worktop(&'a mut WorktopSubstate),
    AuthZone(&'a mut AuthZone),
    Account(&'a mut AccountSubstate),
    AccessController(&'a mut AccessControllerSubstate),
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut WorktopSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::Worktop(value) => value,
            _ => panic!("Not a worktop"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut AuthZone {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::AuthZone(value) => value,
            _ => panic!("Not a AuthZone"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut FungibleResourceManagerSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ResourceManager(value) => value,
            _ => panic!("Not a resource manager"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut NonFungibleResourceManagerSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::NonFungibleResourceManager(value) => value,
            _ => panic!("Not a non fungible resource manager"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut Option<ScryptoValue> {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::KeyValueStoreEntry(value) => value,
            _ => panic!("Not a kv store entry"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ComponentStateSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ComponentState(value) => value,
            _ => panic!("Not a component state"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ComponentRoyaltyConfigSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ComponentRoyaltyConfig(value) => value,
            _ => panic!("Not a component royalty config"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ComponentRoyaltyAccumulatorSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ComponentRoyaltyAccumulator(value) => value,
            _ => panic!("Not a component royalty accumulator"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut PackageRoyaltySubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::PackageRoyalty(value) => value,
            _ => panic!("Not a package royalty"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut EpochManagerSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::EpochManager(value) => value,
            _ => panic!("Not a epoch manager"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ValidatorSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::Validator(value) => value,
            _ => panic!("Not a validator"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut RegisteredValidatorsSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::RegisteredValidators(value) => value,
            _ => panic!("Not a validator set"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ValidatorSetSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ValidatorSet(value) => value,
            _ => panic!("Not a validator set"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ClockSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::CurrentTimeRoundedToMinutes(value) => value,
            _ => panic!("Not current time"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut MethodAccessRulesSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::MethodAccessRules(value) => value,
            _ => panic!("Not a logger"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut AccessControllerSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::AccessController(value) => value,
            _ => panic!("Not access controller"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut ProofInfoSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::ProofInfo(value) => value,
            _ => panic!("Not ProofInfo"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut NonFungibleVaultIdTypeSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::NonFungibleVaultInfo(value) => value,
            _ => panic!("Not a VaultInfo"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut LiquidFungibleResource {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::VaultLiquidFungible(value) => value,
            SubstateRefMut::BucketLiquidFungible(value) => value,
            _ => panic!("Not a vault/bucket liquid fungible"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut LiquidNonFungibleResource {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::VaultLiquidNonFungible(value) => value,
            SubstateRefMut::BucketLiquidNonFungible(value) => value,
            _ => panic!("Not a vault/bucket liquid non-fungible"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut LockedFungibleResource {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::VaultLockedFungible(value) => value,
            SubstateRefMut::BucketLockedFungible(value) => value,
            _ => panic!("Not a vault/bucket locked fungible"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut LockedNonFungibleResource {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::VaultLockedNonFungible(value) => value,
            SubstateRefMut::BucketLockedNonFungible(value) => value,
            _ => panic!("Not a vault/bucket locked non-fungible"),
        }
    }
}

impl<'a> From<SubstateRefMut<'a>> for &'a mut BucketInfoSubstate {
    fn from(value: SubstateRefMut<'a>) -> Self {
        match value {
            SubstateRefMut::BucketInfo(value) => value,
            _ => panic!("Not a BucketInfo"),
        }
    }
}
