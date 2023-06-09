use crate::blueprints::access_controller::AccessControllerError;
use crate::blueprints::account::AccountError;
use crate::blueprints::epoch_manager::{EpochManagerError, ValidatorError};
use crate::blueprints::package::PackageError;
use crate::blueprints::resource::{
    BucketError, FungibleResourceManagerError, NonFungibleResourceManagerError, ProofError,
    VaultError, WorktopError,
};
use crate::blueprints::transaction_processor::TransactionProcessorError;
use crate::kernel::actor::{Actor, ExecutionMode};
use crate::kernel::track::TrackError;
use crate::system::kernel_modules::auth::AuthError;
use crate::system::kernel_modules::costing::CostingError;
use crate::system::kernel_modules::events::EventError;
use crate::system::kernel_modules::node_move::NodeMoveError;
use crate::system::kernel_modules::transaction_limits::TransactionLimitsError;
use crate::system::node_modules::access_rules::{AccessRulesChainError, AuthZoneError};
use crate::system::node_modules::metadata::MetadataPanicError;
use crate::transaction::AbortReason;
use crate::types::*;
use crate::wasm::WasmRuntimeError;
use radix_engine_interface::api::substate_api::LockFlags;

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum IdAllocationError {
    RENodeIdWasNotAllocated(RENodeId),
    AllocatedIDsNotEmpty(BTreeSet<RENodeId>),
    OutOfID,
}

pub trait CanBeAbortion {
    fn abortion(&self) -> Option<&AbortReason>;
}

/// Represents an error which causes a tranasction to be rejected.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum RejectionError {
    SuccessButFeeLoanNotRepaid,
    ErrorBeforeFeeLoanRepaid(RuntimeError),
    TransactionEpochNotYetValid {
        valid_from: u64,
        current_epoch: u64,
    },
    TransactionEpochNoLongerValid {
        valid_until: u64,
        current_epoch: u64,
    },
}

impl fmt::Display for RejectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Represents an error when executing a transaction.
#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum RuntimeError {
    /// An error occurred within the kernel.
    KernelError(KernelError),

    /// An error occurred within call frame.
    CallFrameError(CallFrameError),

    SystemError(SystemError),

    /// An error occurred within an interpreter
    InterpreterError(InterpreterError),

    /// An error occurred within a kernel module.
    ModuleError(ModuleError),

    /// An error occurred within application logic, like the RE models.
    ApplicationError(ApplicationError),
}

impl From<KernelError> for RuntimeError {
    fn from(error: KernelError) -> Self {
        RuntimeError::KernelError(error.into())
    }
}

impl From<CallFrameError> for RuntimeError {
    fn from(error: CallFrameError) -> Self {
        RuntimeError::CallFrameError(error.into())
    }
}

impl From<InterpreterError> for RuntimeError {
    fn from(error: InterpreterError) -> Self {
        RuntimeError::InterpreterError(error.into())
    }
}

impl From<ModuleError> for RuntimeError {
    fn from(error: ModuleError) -> Self {
        RuntimeError::ModuleError(error.into())
    }
}

impl From<ApplicationError> for RuntimeError {
    fn from(error: ApplicationError) -> Self {
        RuntimeError::ApplicationError(error.into())
    }
}

impl CanBeAbortion for RuntimeError {
    fn abortion(&self) -> Option<&AbortReason> {
        match self {
            RuntimeError::KernelError(err) => err.abortion(),
            RuntimeError::CallFrameError(_) => None,
            RuntimeError::InterpreterError(_) => None,
            RuntimeError::SystemError(_) => None,
            RuntimeError::ModuleError(err) => err.abortion(),
            RuntimeError::ApplicationError(_) => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum KernelError {
    InvalidModeTransition(ExecutionMode, ExecutionMode),

    // invocation
    WasmRuntimeError(WasmRuntimeError),
    RENodeNotFound(RENodeId),
    InvalidDirectAccess,

    // ID allocation
    IdAllocationError(IdAllocationError),

    // SBOR decoding
    SborDecodeError(DecodeError),
    SborEncodeError(EncodeError),

    // RENode
    ContainsDuplicatedOwns,
    StoredNodeRemoved(RENodeId),
    RENodeGlobalizeTypeNotAllowed(RENodeId),
    TrackError(Box<TrackError>),
    LockDoesNotExist(LockHandle),
    LockNotMutable(LockHandle),
    BlobNotFound(Hash),
    DropNodeFailure(RENodeId),

    // Substate Constraints
    InvalidOffset(SubstateOffset),
    InvalidOwnership(Box<InvalidOwnership>),
    InvalidId(RENodeId),

    // Actor Constraints
    InvalidDropNodeAccess(Box<InvalidDropNodeAccess>),
    InvalidSubstateAccess(Box<InvalidSubstateAccess>),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct InvalidOwnership(pub SubstateOffset, pub PackageAddress, pub String);

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct InvalidDropNodeAccess {
    pub mode: ExecutionMode,
    pub actor: Actor,
    pub node_id: RENodeId,
    pub package_address: PackageAddress,
    pub blueprint_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct InvalidSubstateAccess {
    pub mode: ExecutionMode,
    pub actor: Actor,
    pub node_id: RENodeId,
    pub offset: SubstateOffset,
    pub flags: LockFlags,
}

impl CanBeAbortion for KernelError {
    fn abortion(&self) -> Option<&AbortReason> {
        match self {
            KernelError::WasmRuntimeError(err) => err.abortion(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum CallFrameError {
    OffsetDoesNotExist(Box<OffsetDoesNotExist>),
    RENodeNotVisible(RENodeId),
    RENodeNotOwned(RENodeId),
    MovingLockedRENode(RENodeId),
    FailedToMoveSubstateToTrack(Box<TrackError>),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct OffsetDoesNotExist(pub RENodeId, pub SubstateOffset);

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum SystemError {
    NotAnObject,
    NotAKeyValueStore,
    InvalidSubstateWrite,
    InvalidKeyValueStoreOwnership,
    InvalidLockFlags,
    InvalidKeyValueStoreSchema(SchemaValidationError),
    CannotGlobalize,
    InvalidModuleSet(Box<InvalidModuleSet>),
    InvalidModule,
    InvalidModuleType(Box<InvalidModuleType>),
    SubstateValidationError(Box<SubstateValidationError>),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum SubstateValidationError {
    BlueprintNotFound(String),
    WrongNumberOfSubstates(String, usize, usize),
    SchemaValidationError(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum InterpreterError {
    CallMethodOnKeyValueStore,

    NativeUnexpectedReceiver(String),
    NativeExpectedReceiver(String),
    NativeExportDoesNotExist(String),
    NativeInvalidCodeId(u8),

    ScryptoBlueprintNotFound(PackageAddress, String),
    ScryptoFunctionNotFound(String),
    ScryptoReceiverNotMatch(String),
    ScryptoInputSchemaNotMatch(String, String),
    ScryptoInputDecodeError(DecodeError),

    ScryptoOutputDecodeError(DecodeError),
    ScryptoOutputSchemaNotMatch(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum ModuleError {
    NodeMoveError(NodeMoveError),
    AuthError(AuthError),
    CostingError(CostingError),
    TransactionLimitsError(TransactionLimitsError),
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct InvalidModuleType {
    pub expected_package: PackageAddress,
    pub expected_blueprint: String,
    pub actual_package: PackageAddress,
    pub actual_blueprint: String,
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub struct InvalidModuleSet(pub RENodeId, pub BTreeSet<NodeModuleId>);

impl CanBeAbortion for ModuleError {
    fn abortion(&self) -> Option<&AbortReason> {
        match self {
            Self::CostingError(err) => err.abortion(),
            _ => None,
        }
    }
}

impl From<NodeMoveError> for ModuleError {
    fn from(error: NodeMoveError) -> Self {
        Self::NodeMoveError(error)
    }
}

impl From<AuthError> for ModuleError {
    fn from(error: AuthError) -> Self {
        Self::AuthError(error)
    }
}

impl From<CostingError> for ModuleError {
    fn from(error: CostingError) -> Self {
        Self::CostingError(error)
    }
}

/// This enum is to help with designing intuitive error abstractions.
/// Each engine module can have its own [`SelfError`], but can also wrap arbitrary downstream errors.
/// Ultimately these errors get flattened out to a [`RuntimeError`] anyway.
#[derive(Debug, Clone)]
pub enum InvokeError<E: SelfError> {
    SelfError(E),
    Downstream(RuntimeError),
}

/// This is a trait for the non-Downstream part of [`InvokeError`]
/// We can't use `Into<RuntimeError>` because we need [`RuntimeError`] _not_ to implement it.
pub trait SelfError {
    fn into_runtime_error(self) -> RuntimeError;
}

impl<E: Into<ApplicationError>> SelfError for E {
    fn into_runtime_error(self) -> RuntimeError {
        self.into().into()
    }
}

impl<E: SelfError> From<RuntimeError> for InvokeError<E> {
    fn from(runtime_error: RuntimeError) -> Self {
        InvokeError::Downstream(runtime_error)
    }
}

impl<E: SelfError> From<E> for InvokeError<E> {
    fn from(error: E) -> Self {
        InvokeError::SelfError(error)
    }
}

impl<E: SelfError> InvokeError<E> {
    pub fn error(error: E) -> Self {
        InvokeError::SelfError(error)
    }

    pub fn downstream(runtime_error: RuntimeError) -> Self {
        InvokeError::Downstream(runtime_error)
    }
}

impl<E: SelfError> From<InvokeError<E>> for RuntimeError {
    fn from(error: InvokeError<E>) -> Self {
        match error {
            InvokeError::Downstream(runtime_error) => runtime_error,
            InvokeError::SelfError(e) => e.into_runtime_error(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, ScryptoSbor)]
pub enum ApplicationError {
    TransactionProcessorError(TransactionProcessorError),

    PackageError(PackageError),

    EpochManagerError(EpochManagerError),

    ValidatorError(ValidatorError),

    ResourceManagerError(FungibleResourceManagerError),

    NonFungibleResourceManagerError(NonFungibleResourceManagerError),

    AccessRulesChainError(AccessRulesChainError),

    BucketError(BucketError),

    ProofError(ProofError),

    VaultError(VaultError),

    WorktopError(WorktopError),

    AuthZoneError(AuthZoneError),

    AccountError(AccountError),

    AccessControllerError(AccessControllerError),

    EventError(Box<EventError>),

    MetadataError(MetadataPanicError),
}

impl From<TransactionProcessorError> for ApplicationError {
    fn from(value: TransactionProcessorError) -> Self {
        Self::TransactionProcessorError(value)
    }
}

impl From<PackageError> for ApplicationError {
    fn from(value: PackageError) -> Self {
        Self::PackageError(value)
    }
}

impl From<EpochManagerError> for ApplicationError {
    fn from(value: EpochManagerError) -> Self {
        Self::EpochManagerError(value)
    }
}

impl From<FungibleResourceManagerError> for ApplicationError {
    fn from(value: FungibleResourceManagerError) -> Self {
        Self::ResourceManagerError(value)
    }
}

impl From<AccessRulesChainError> for ApplicationError {
    fn from(value: AccessRulesChainError) -> Self {
        Self::AccessRulesChainError(value)
    }
}

impl From<BucketError> for ApplicationError {
    fn from(value: BucketError) -> Self {
        Self::BucketError(value)
    }
}

impl From<ProofError> for ApplicationError {
    fn from(value: ProofError) -> Self {
        Self::ProofError(value)
    }
}

impl From<VaultError> for ApplicationError {
    fn from(value: VaultError) -> Self {
        Self::VaultError(value)
    }
}

impl From<WorktopError> for ApplicationError {
    fn from(value: WorktopError) -> Self {
        Self::WorktopError(value)
    }
}

impl From<AuthZoneError> for ApplicationError {
    fn from(value: AuthZoneError) -> Self {
        Self::AuthZoneError(value)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
