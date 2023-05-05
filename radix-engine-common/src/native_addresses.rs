use crate::types::*;

//=========================================================================
// Please see and update REP-71 along with changes to this file
//=========================================================================

//=========================================================================
// FUNGIBLES
//=========================================================================

/// XRD is the native token of the Radix ledger.
/// It is a fungible token, measured in attos (`10^-18`).
///
/// It is used for paying fees and staking.
pub const XRD: ResourceAddress = ResourceAddress::new_or_panic([
    93, 166, 99, 24, 198, 49, 140, 97, 245, 166, 27, 76, 99, 24, 198, 49, 140, 247, 148, 170, 141,
    41, 95, 20, 230, 49, 140, 99, 24, 198,
]);
// Export some aliases for backwards compatibility of dApp developer tests
pub use XRD as RADIX_TOKEN;
pub use XRD as NATIVE_TOKEN;

//=========================================================================
// VIRTUAL BADGES
//=========================================================================

/// The non-fungible badge resource which is used for virtual proofs of ECDSA Secp256k1 transacton signatures in the transaction processor.
pub const ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE: ResourceAddress =
    ResourceAddress::new_or_panic([
        154, 76, 99, 24, 198, 49, 140, 104, 103, 1, 130, 12, 99, 24, 198, 49, 140, 247, 215, 81,
        57, 213, 170, 213, 230, 49, 140, 99, 24, 198,
    ]);

/// The non-fungible badge resource which is used for virtual proofs of EdDSA Ed25519 transacton signatures in the transaction processor.
pub const EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 108, 181, 84, 130, 12, 99, 24, 198, 49, 140, 247, 169, 81, 215,
    169, 229, 71, 198, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for virtual proofs representing the package of the current actor.
///
/// For example, an internal Pool component under a global Swap component would have a package badge of the Pool's package address.
pub const PACKAGE_VIRTUAL_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 96, 219, 184, 88, 204, 99, 24, 198, 49, 140, 247, 191, 90, 57,
    222, 245, 23, 198, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for virtual proofs representing the global ancestor of the current actor.
///
/// For example, an internal Pool component under a global Swap component would have a global actor badge of the Swap's component address.
pub const GLOBAL_ACTOR_VIRTUAL_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 100, 127, 184, 88, 204, 99, 24, 198, 49, 140, 245, 82, 53, 210,
    181, 165, 29, 230, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for virtual proofs representing the current local actor module that is automatically on an auth zone.
///
/// For example, an internal Pool component under a global Swap component would have a local actor badge of the Pool's internal component address, for the object module.
///
/// TODO: Needs implementing!
pub const LOCAL_ACTOR_VIRTUAL_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 111, 227, 184, 88, 204, 99, 24, 198, 49, 140, 247, 197, 85, 209,
    106, 114, 170, 134, 49, 140, 99, 24, 198,
]);

//=========================================================================
// TRANSACTION BADGES
//=========================================================================

/// The non-fungible badge resource which is used for virtual proofs representing the fact that the current transaction is a system transaction.
pub const SYSTEM_TRANSACTION_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 104, 18, 11, 52, 204, 99, 24, 198, 49, 140, 247, 171, 71, 140,
    85, 71, 199, 198, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for virtual proofs representing the fact that the current transaction is a consensus/validator transaction.
pub const CONSENSUS_TRANSACTION_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 108, 78, 11, 52, 204, 99, 24, 198, 49, 140, 247, 189, 90, 122,
    171, 74, 81, 70, 49, 140, 99, 24, 198,
]);

//=========================================================================
// ENTITY OWNER BADGES
//=========================================================================

/// The non-fungible badge resource which is used for package ownership when creating packages with the simple package creation set-up.
pub const PACKAGE_OWNER_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 96, 217, 14, 152, 204, 99, 24, 198, 49, 140, 247, 170, 148, 61,
    41, 26, 62, 134, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for validator ownership.
pub const VALIDATOR_OWNER_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 102, 52, 110, 152, 204, 99, 24, 198, 49, 140, 247, 214, 58, 162,
    169, 19, 198, 166, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for account ownership, if accounts have been set up with simple account creation, or have been securified.
pub const ACCOUNT_OWNER_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 110, 227, 14, 152, 204, 99, 24, 198, 49, 140, 247, 235, 90, 171,
    212, 167, 233, 70, 49, 140, 99, 24, 198,
]);

/// The non-fungible badge resource which is used for identity ownership, if identities have been set up with simple account creation, or have been securified.
pub const IDENTITY_OWNER_BADGE: ResourceAddress = ResourceAddress::new_or_panic([
    154, 76, 99, 24, 198, 49, 140, 102, 205, 110, 152, 204, 99, 24, 198, 49, 140, 247, 168, 254,
    213, 107, 82, 168, 230, 49, 140, 99, 24, 198,
]);

//=========================================================================
// PACKAGES
//=========================================================================

/// The native package for package deployment.
pub const PACKAGE_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 96, 247, 22, 70, 76, 99, 24, 198, 49, 140, 247, 191, 202, 214,
    163, 21, 43, 70, 49, 140, 99, 24, 198,
]);

/// The native package for resource managers, proofs, buckets, vaults etc.
pub const RESOURCE_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 97, 230, 3, 198, 76, 99, 24, 198, 49, 140, 247, 190, 145, 61,
    99, 170, 251, 198, 49, 140, 99, 24, 198,
]);

/// The native package for accounts.
pub const ACCOUNT_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 110, 227, 19, 89, 140, 99, 24, 198, 49, 140, 247, 188, 170, 46,
    149, 74, 150, 38, 49, 140, 99, 24, 198,
]);

/// The native package for identities.
pub const IDENTITY_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 102, 205, 100, 49, 140, 99, 24, 198, 49, 140, 247, 158, 154,
    127, 143, 23, 156, 166, 49, 140, 99, 24, 198,
]);

/// The native package for the epoch manager.
pub const EPOCH_MANAGER_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 108, 135, 23, 218, 12, 99, 24, 198, 49, 140, 247, 191, 197, 242,
    149, 31, 42, 134, 49, 140, 99, 24, 198,
]);

/// The native package for the clock.
pub const CLOCK_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 108, 127, 22, 49, 140, 99, 24, 198, 49, 140, 247, 190, 158, 244,
    107, 90, 248, 230, 49, 140, 99, 24, 198,
]);

/// The native package for access controllers.
pub const ACCESS_CONTROLLER_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 108, 77, 99, 248, 204, 99, 24, 198, 49, 140, 247, 191, 85, 61,
    60, 165, 22, 134, 49, 140, 99, 24, 198,
]);

/// The native package for the transaction processor.
pub const TRANSACTION_PROCESSOR_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 101, 154, 97, 48, 204, 99, 24, 198, 49, 140, 247, 168, 186, 82,
    149, 234, 191, 70, 49, 140, 99, 24, 198,
]);

/// The native package for the metadata module.
pub const METADATA_MODULE_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 109, 173, 189, 95, 76, 99, 24, 198, 49, 140, 247, 209, 85, 213,
    61, 229, 104, 166, 49, 140, 99, 24, 198,
]);

/// The native package for the royalty module.
pub const ROYALTY_MODULE_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 97, 147, 191, 89, 12, 99, 24, 198, 49, 140, 247, 196, 245, 45,
    61, 24, 151, 70, 49, 140, 99, 24, 198,
]);

/// The native package for the access rules module.
pub const ACCESS_RULES_MODULE_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 110, 143, 159, 204, 12, 99, 24, 198, 49, 140, 247, 170, 47, 173,
    116, 162, 158, 38, 49, 140, 99, 24, 198,
]);

/// The scrypto package for the genesis helper.
pub const GENESIS_HELPER_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 100, 102, 112, 129, 140, 99, 24, 198, 49, 140, 247, 214, 62, 86,
    170, 175, 122, 38, 49, 140, 99, 24, 198,
]);
/// The name of the genesis helper blueprint under the `GENESIS_HELPER_PACKAGE`.
pub const GENESIS_HELPER_BLUEPRINT: &str = "GenesisHelper";

/// The scrypto package for the faucet
pub const FAUCET_PACKAGE: PackageAddress = PackageAddress::new_or_panic([
    13, 144, 99, 24, 198, 49, 140, 100, 247, 152, 202, 204, 99, 24, 198, 49, 140, 247, 189, 241,
    172, 105, 67, 234, 38, 49, 140, 99, 24, 198,
]);
/// The name of the faucet blueprint under the `FAUCET_PACKAGE`.
pub const FAUCET_BLUEPRINT: &str = "Faucet";

//=========================================================================
// SYSTEM SINGLETON COMPONENTS - NATIVE
//=========================================================================

/// The epoch manager native component - in charge of validators, consensus and epochs.
pub const EPOCH_MANAGER: ComponentAddress = ComponentAddress::new_or_panic([
    134, 76, 99, 24, 198, 49, 140, 108, 134, 251, 64, 204, 99, 24, 198, 49, 140, 247, 150, 52, 85,
    30, 250, 28, 166, 49, 140, 99, 24, 198,
]);

/// The clock native component - in charge of time.
pub const CLOCK: ComponentAddress = ComponentAddress::new_or_panic([
    133, 140, 99, 24, 198, 49, 140, 108, 127, 22, 49, 140, 99, 24, 198, 49, 140, 247, 169, 84, 141,
    15, 18, 149, 70, 49, 140, 99, 24, 198,
]);

//=========================================================================
// SYSTEM SINGLETON COMPONENTS - SCRYPTO
//=========================================================================

/// The genesis helper scrypto component - used for sorting out genesis.
pub const GENESIS_HELPER: ComponentAddress = ComponentAddress::new_or_panic([
    192, 86, 99, 24, 198, 49, 140, 100, 102, 112, 129, 140, 99, 24, 198, 49, 140, 247, 188, 190,
    244, 94, 170, 68, 166, 49, 140, 99, 24, 198,
]);

/// The faucet native component - use this on testnets for getting XRD and locking fee.
pub const FAUCET: ComponentAddress = ComponentAddress::new_or_panic([
    192, 86, 99, 24, 198, 49, 140, 100, 247, 152, 202, 204, 99, 24, 198, 49, 140, 247, 190, 138,
    247, 138, 120, 248, 166, 49, 140, 99, 24, 198,
]);
// Export an alias for backwards compatibility of dApp developer tests
pub use FAUCET as FAUCET_COMPONENT;

//=========================================================================
//=========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use radix_engine_common::{address::Bech32Encoder, network::NetworkDefinition};

    #[test]
    fn test_mainnet_vanity_addresses() {
        // Fungible Resources
        check_address(
            XRD.as_ref(),
            EntityType::GlobalFungibleResource,
            "resource_rdx1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxradxrd",
        );

        // Virtual Badges
        check_address(
            ECDSA_SECP256K1_SIGNATURE_VIRTUAL_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxsecpsgxxxxxxxxx004638826440xxxxxxxxxsecpsg",
        );
        check_address(
            EDDSA_ED25519_SIGNATURE_VIRTUAL_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxed25sgxxxxxxxxx002236757237xxxxxxxxxed25sg",
        );
        check_address(
            PACKAGE_VIRTUAL_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxpkactrxxxxxxxxx000668800297xxxxxxxxxpkactr",
        );
        check_address(
            GLOBAL_ACTOR_VIRTUAL_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxglactrxxxxxxxxx025346266280xxxxxxxxxglactr",
        );
        check_address(
            LOCAL_ACTOR_VIRTUAL_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxlcactrxxxxxxxxx003246948925xxxxxxxxxlcactr",
        );

        // Transaction badges
        check_address(
            SYSTEM_TRANSACTION_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxsystxnxxxxxxxxx002683325037xxxxxxxxxsystxn",
        );
        check_address(
            CONSENSUS_TRANSACTION_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxcnstxnxxxxxxxxx000260245552xxxxxxxxxcnstxn",
        );

        // Entity owner badges
        check_address(
            PACKAGE_OWNER_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxpkgwnrxxxxxxxxx002558553505xxxxxxxxxpkgwnr",
        );
        check_address(
            VALIDATOR_OWNER_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxvdrwnrxxxxxxxxx004365253834xxxxxxxxxvdrwnr",
        );
        check_address(
            ACCOUNT_OWNER_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxaccwnrxxxxxxxxx006664022062xxxxxxxxxaccwnr",
        );
        check_address(
            IDENTITY_OWNER_BADGE.as_ref(),
            EntityType::GlobalNonFungibleResource,
            "resource_rdx1nfxxxxxxxxxxdntwnrxxxxxxxxx002876444928xxxxxxxxxdntwnr",
        );

        // Packages
        check_address(
            PACKAGE_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxpackgexxxxxxxxx000726633226xxxxxxxxxpackge",
        );
        check_address(
            RESOURCE_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxresrcexxxxxxxxx000538436477xxxxxxxxxresrce",
        );
        check_address(
            ACCOUNT_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxaccntxxxxxxxxxx000929625493xxxxxxxxxaccntx",
        );
        check_address(
            IDENTITY_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxdntyxxxxxxxxxxx008560783089xxxxxxxxxdntyxx",
        );
        check_address(
            EPOCH_MANAGER_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxepchmgxxxxxxxxx000797223725xxxxxxxxxepchmg",
        );
        check_address(
            CLOCK_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxclckxxxxxxxxxxx000577344478xxxxxxxxxclckxx",
        );
        check_address(
            ACCESS_CONTROLLER_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxcntrlrxxxxxxxxx000648572295xxxxxxxxxcntrlr",
        );
        check_address(
            TRANSACTION_PROCESSOR_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxtxnpxrxxxxxxxxx002962227406xxxxxxxxxtxnpxr",
        );
        check_address(
            METADATA_MODULE_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxmtdataxxxxxxxxx005246577269xxxxxxxxxmtdata",
        );
        check_address(
            ROYALTY_MODULE_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxryaltyxxxxxxxxx003849573396xxxxxxxxxryalty",
        );
        check_address(
            ACCESS_RULES_MODULE_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxarulesxxxxxxxxx002304462983xxxxxxxxxarules",
        );
        check_address(
            GENESIS_HELPER_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxgenssxxxxxxxxxx004372642773xxxxxxxxxgenssx",
        );
        check_address(
            FAUCET_PACKAGE.as_ref(),
            EntityType::GlobalPackage,
            "package_rdx1pkgxxxxxxxxxfaucetxxxxxxxxx000034355863xxxxxxxxxfaucet",
        );

        // System singleton components - native
        check_address(
            EPOCH_MANAGER.as_ref(),
            EntityType::GlobalEpochManager,
            "epochmanager_rdx1sexxxxxxxxxxephmgrxxxxxxxxx009352500589xxxxxxxxxephmgr",
        );
        check_address(
            CLOCK.as_ref(),
            EntityType::GlobalClock,
            "clock_rdx1skxxxxxxxxxxclckxxxxxxxxxxx002253583992xxxxxxxxxclckxx",
        );

        // System singleton components - scrypto
        check_address(
            FAUCET.as_ref(),
            EntityType::GlobalGenericComponent,
            "component_rdx1cptxxxxxxxxxfaucetxxxxxxxxx000527798379xxxxxxxxxfaucet",
        );
        check_address(
            GENESIS_HELPER.as_ref(),
            EntityType::GlobalGenericComponent,
            "component_rdx1cptxxxxxxxxxgenssxxxxxxxxxx000977302539xxxxxxxxxgenssx",
        );
    }

    fn check_address(address_bytes: &[u8], entity_type: EntityType, address_string: &str) {
        assert_eq!(address_bytes[0], entity_type as u8);
        let encoded_address = Bech32Encoder::new(&NetworkDefinition::mainnet())
            .encode(address_bytes)
            .unwrap();
        assert_eq!(encoded_address.as_str(), address_string);
    }
}