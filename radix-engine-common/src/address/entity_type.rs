use strum::FromRepr;

/// An enum which represents the different addressable entities.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, FromRepr)]
pub enum EntityType {
    GlobalPackage,
    GlobalFungibleResource,
    GlobalNonFungibleResource,
    GlobalEpochManager,
    GlobalValidator,
    GlobalClock,
    GlobalAccessController,
    GlobalAccount,
    GlobalIdentity,
    GlobalComponent,

    GlobalVirtualEcdsaAccount,
    GlobalVirtualEddsaAccount,
    GlobalVirtualEcdsaIdentity,
    GlobalVirtualEddsaIdentity,

    InternalVault,
    InternalAccessController,
    InternalAccount,
    InternalComponent,
    InternalKeyValueStore,
}
