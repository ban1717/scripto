use super::*;
use sbor::*;

use well_known_scrypto_custom_types::*;

pub mod well_known_scrypto_custom_types {
    use super::*;

    pub const ADDRESS_ID: u8 = VALUE_KIND_ADDRESS;
    pub const PACKAGE_ADDRESS_ID: u8 = VALUE_KIND_ADDRESS + 1;
    pub const COMPONENT_ADDRESS_ID: u8 = VALUE_KIND_ADDRESS + 2;
    pub const RESOURCE_ADDRESS_ID: u8 = VALUE_KIND_ADDRESS + 3;

    pub const OWN_ID: u8 = VALUE_KIND_OWN;
    pub const OWN_BUCKET_ID: u8 = VALUE_KIND_OWN + 1;
    pub const OWN_PROOF_ID: u8 = VALUE_KIND_OWN + 2;
    pub const OWN_VAULT_ID: u8 = VALUE_KIND_OWN + 3;
    pub const OWN_KEY_VALUE_STORE_ID: u8 = VALUE_KIND_OWN + 4;

    pub const DECIMAL_ID: u8 = VALUE_KIND_DECIMAL;
    pub const PRECISE_DECIMAL_ID: u8 = VALUE_KIND_PRECISE_DECIMAL;
    pub const NON_FUNGIBLE_LOCAL_ID_ID: u8 = VALUE_KIND_NON_FUNGIBLE_LOCAL_ID;
    pub const REFERENCE_ID: u8 = VALUE_KIND_REFERENCE;
}

fn unnamed_type_kind(
    custom_type_kind: ScryptoCustomTypeKind,
) -> TypeData<ScryptoCustomTypeKind, LocalTypeIndex> {
    TypeData::unnamed(TypeKind::Custom(custom_type_kind))
}

create_well_known_lookup!(
    WELL_KNOWN_LOOKUP,
    ScryptoCustomTypeKind,
    [
        // Addresses
        (
            ADDRESS_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Address)
        ),
        (
            PACKAGE_ADDRESS_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::PackageAddress)
        ),
        (
            COMPONENT_ADDRESS_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::ComponentAddress)
        ),
        (
            RESOURCE_ADDRESS_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::ResourceAddress)
        ),
        // Owned entities
        (OWN_ID, unnamed_type_kind(ScryptoCustomTypeKind::Own)),
        (
            OWN_BUCKET_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Bucket)
        ),
        (
            OWN_PROOF_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Proof)
        ),
        (
            OWN_VAULT_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Vault)
        ),
        (
            OWN_KEY_VALUE_STORE_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::KeyValueStore)
        ),
        // Others
        (
            DECIMAL_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Decimal)
        ),
        (
            PRECISE_DECIMAL_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::PreciseDecimal)
        ),
        (
            NON_FUNGIBLE_LOCAL_ID_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::NonFungibleLocalId)
        ),
        (
            REFERENCE_ID,
            unnamed_type_kind(ScryptoCustomTypeKind::Reference)
        ),
    ]
);

pub(crate) fn resolve_scrypto_well_known_type(
    well_known_index: u8,
) -> Option<&'static TypeData<ScryptoCustomTypeKind, LocalTypeIndex>> {
    // We know that WELL_KNOWN_LOOKUP has 255 elements, so can use `get_unchecked` for fast look-ups
    unsafe {
        WELL_KNOWN_LOOKUP
            .get_unchecked(well_known_index as usize)
            .as_ref()
    }
}
