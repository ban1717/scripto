use crate::api::types::*;
use crate::scrypto;
use crate::Describe;

// Native function identifier used by transaction model
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[scrypto(TypeId, Encode, Decode)]
pub struct NativeFunctionIdent {
    pub blueprint_name: String,
    pub function_name: String,
}

// Native method identifier used by transaction model
#[derive(Debug, Clone, Eq, PartialEq)]
#[scrypto(TypeId, Encode, Decode)]
pub struct NativeMethodIdent {
    pub receiver: RENodeId,
    pub method_name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[scrypto(TypeId, Encode, Decode, Describe)]
pub enum NativeFn {
    Method(NativeMethod),
    Function(NativeFunction),
}

// Native function enum used by Kernel SystemAPI and WASM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[scrypto(TypeId, Encode, Decode, Describe)]
pub enum NativeMethod {
    AccessRulesChain(AccessRulesChainMethod),
    Component(ComponentMethod), // TODO: investigate whether to make royalty universal and take any "receiver".
    Package(PackageMethod),
    Metadata(MetadataMethod),
    EpochManager(EpochManagerMethod),
    AuthZoneStack(AuthZoneStackMethod),
    ResourceManager(ResourceManagerMethod),
    Bucket(BucketMethod),
    Vault(VaultMethod),
    Proof(ProofMethod),
    Worktop(WorktopMethod),
    Logger(LoggerMethod),
    Clock(ClockMethod),
    TransactionHash(TransactionHashMethod),
}

impl Into<FnIdentifier> for NativeMethod {
    fn into(self) -> FnIdentifier {
        FnIdentifier::NativeMethod(self)
    }
}

// Native method enum used by Kernel SystemAPI and WASM
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[scrypto(TypeId, Encode, Decode, Describe)]
pub enum NativeFunction {
    Component(ComponentFunction),
    EpochManager(EpochManagerFunction),
    ResourceManager(ResourceManagerFunction),
    Package(PackageFunction),
    TransactionProcessor(TransactionProcessorFunction),
    Clock(ClockFunction),
}

impl Into<FnIdentifier> for NativeFunction {
    fn into(self) -> FnIdentifier {
        FnIdentifier::NativeFunction(self)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum AccessRulesChainMethod {
    AddAccessCheck,
    SetMethodAccessRule,
    SetGroupAccessRule,
    SetMethodMutability,
    SetGroupMutability,
    GetLength,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum MetadataMethod {
    Set,
    Get,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentFunction {
    Globalize,
    GlobalizeWithOwner,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum EpochManagerFunction {
    Create,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ComponentMethod {
    SetRoyaltyConfig,
    ClaimRoyalty,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum PackageMethod {
    SetRoyaltyConfig,
    ClaimRoyalty,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum EpochManagerMethod {
    GetCurrentEpoch,
    NextRound,
    SetEpoch,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum AuthZoneStackMethod {
    Pop,
    Push,
    CreateProof,
    CreateProofByAmount,
    CreateProofByIds,
    Clear,
    Drain,
    AssertAccessRule,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ResourceManagerFunction {
    Create,
    BurnBucket,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ResourceManagerMethod {
    Mint,
    Burn,
    UpdateVaultAuth,
    LockAuth,
    UpdateNonFungibleData,
    GetNonFungible,
    GetResourceType,
    GetTotalSupply,
    NonFungibleExists,
    CreateBucket,
    CreateVault,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum BucketMethod {
    Take,
    TakeNonFungibles,
    Put,
    GetNonFungibleIds,
    GetAmount,
    GetResourceAddress,
    CreateProof,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum VaultMethod {
    Take,
    LockFee,
    Put,
    TakeNonFungibles,
    GetAmount,
    GetResourceAddress,
    GetNonFungibleIds,
    CreateProof,
    CreateProofByAmount,
    CreateProofByIds,
    Recall,
    RecallNonFungibles,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ProofMethod {
    Clone,
    GetAmount,
    GetNonFungibleIds,
    GetResourceAddress,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum WorktopMethod {
    TakeAll,
    TakeAmount,
    TakeNonFungibles,
    Put,
    AssertContains,
    AssertContainsAmount,
    AssertContainsNonFungibles,
    Drain,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum LoggerMethod {
    Log,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ClockFunction {
    Create,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum ClockMethod {
    SetCurrentTime,
    GetCurrentTime,
    CompareCurrentTime,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum TransactionHashMethod {
    Get,
    GenerateUuid,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum PackageFunction {
    Publish,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    EnumString,
    EnumVariantNames,
    IntoStaticStr,
    AsRefStr,
    Display,
)]
#[scrypto(TypeId, Encode, Decode, Describe)]
#[strum(serialize_all = "snake_case")]
pub enum TransactionProcessorFunction {
    Run,
}
