use crate::ffi::bindings::*;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Bits {
    U32(u32),
    U64(u64),
}

/// An `NvmlError` with an optionally present source error for chaining errors
#[derive(Error, Debug)]
#[error("{error}")]
pub struct NvmlErrorWithSource {
    pub error: NvmlError,
    pub source: Option<NvmlError>,
}

impl From<NvmlError> for NvmlErrorWithSource {
    fn from(error: NvmlError) -> Self {
        Self {
            error,
            source: None,
        }
    }
}

#[derive(Error, Debug)]
pub enum NvmlError {
    #[error("could not interpret string as utf-8")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("nul byte inside string")]
    NulError(#[from] std::ffi::NulError),
    #[error("a libloading error occurred: {0}")]
    LibloadingError(#[from] libloading::Error),

    /**
    A function symbol failed to load.

    This variant is constructed with a textual description of a
    `libloading::Error`. The error variant itself can't be provided because we're
    unable to take ownership of the error when attempting to use a symbol, and
    `libloading::Error` doesn't impl `Clone`.
    */
    #[error("function symbol failed to load: {0}")]
    FailedToLoadSymbol(String),

    #[error("max string length was {max_len} but string length is {actual_len}")]
    StringTooLong { max_len: usize, actual_len: usize },

    #[error("invalid combination of bits ({0:?}) when trying to interpret as bitflags")]
    IncorrectBits(Bits),

    /**
    An unexpected enum variant was encountered.

    This error is specific to this Rust wrapper. It is used to represent the
    possibility that an enum variant that is not defined within the Rust bindings
    can be returned from a C call.

    The single field contains the value that could not be mapped to a
    defined enum variant.

    See [this issue](https://github.com/rust-lang/rust/issues/36927).
    */
    #[error("unexpected enum variant value: {0}")]
    UnexpectedVariant(u32),

    #[error("a call to `EventSet.release_events()` failed")]
    SetReleaseFailed,

    #[error("a call to `Device.pci_info()` failed")]
    GetPciInfoFailed,

    #[error("a call to `PciInfo.try_into_c()` failed")]
    PciInfoToCFailed,

    #[error("NVML was not first initialized with `Nvml::init()`")]
    Uninitialized,

    #[error("a supplied argument was invalid")]
    InvalidArg,

    #[error("the requested operation is not available on the target device")]
    NotSupported,

    #[error("the current user does not have permission to perform this operation")]
    NoPermission,

    #[error("NVML was already initialized")]
    #[deprecated = "deprecated in NVML (multiple initializations now allowed via refcounting)"]
    AlreadyInitialized,

    #[error("a query to find an object was unsuccessful")]
    NotFound,

    /**
    An input argument is not large enough.

    The single field is the size required for a successful call (if `Some`)
    and `None` if unknown.
    */
    // TODO: verify that ^
    #[error(
        "an input argument is not large enough{}",
        if let Some(size) = .0 {
            format!(", needs to be at least {}", size)
        } else {
            "".into()
        }
    )]
    InsufficientSize(Option<usize>),

    #[error("device's external power cables are not properly attached")]
    InsufficientPower,

    #[error("NVIDIA driver is not loaded")]
    DriverNotLoaded,

    #[error("the provided timeout was reached")]
    Timeout,

    #[error("NVIDIA kernel detected an interrupt issue with a device")]
    IrqIssue,

    #[error("a shared library couldn't be found or loaded")]
    LibraryNotFound,

    #[error("a function couldn't be found in a shared library")]
    FunctionNotFound,

    #[error("the infoROM is corrupted")]
    CorruptedInfoROM,

    #[error("device fell off the bus or has otherwise become inacessible")]
    GpuLost,

    #[error("device requires a reset before it can be used again")]
    ResetRequired,

    #[error("device control has been blocked by the operating system/cgroups")]
    OperatingSystem,

    #[error("RM detects a driver/library version mismatch")]
    LibRmVersionMismatch,

    #[error("operation cannot be performed because the GPU is currently in use")]
    InUse,

    #[error("insufficient memory")]
    InsufficientMemory,

    #[error("no data")]
    NoData,

    #[error(
        "the requested vgpu operation is not available on the target device because \
        ECC is enabled"
    )]
    VgpuEccNotSupported,

    #[error("an internal driver error occurred")]
    Unknown,
}

/// Converts an `nvmlReturn_t` type into a `Result<(), NvmlError>`.
pub fn nvml_try(code: nvmlReturn_t) -> Result<(), NvmlError> {
    if code == nvmlReturn_enum_NVML_SUCCESS {
        return Ok(());
    }
    Err(code.into())
}

/// Converts an `nvmlReturn_t` type into a `Result<(), NvmlError>`, allowing for the call to return the
/// value `nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE` which is a common return value when using an
/// in/out parameter that provides the size of a buffer needed to complete that call
pub fn nvml_try_count(code: nvmlReturn_t) -> Result<(), NvmlError> {
    if code == nvmlReturn_enum_NVML_SUCCESS || code == nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE
    {
        return Ok(());
    }
    Err(code.into())
}

#[allow(deprecated)]
impl From<nvmlReturn_t> for NvmlError {
    fn from(value: nvmlReturn_t) -> Self {
        use NvmlError::*;
        match value {
            nvmlReturn_enum_NVML_ERROR_UNINITIALIZED => Uninitialized,
            nvmlReturn_enum_NVML_ERROR_INVALID_ARGUMENT => InvalidArg,
            nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED => NotSupported,
            nvmlReturn_enum_NVML_ERROR_NO_PERMISSION => NoPermission,
            nvmlReturn_enum_NVML_ERROR_ALREADY_INITIALIZED => AlreadyInitialized,
            nvmlReturn_enum_NVML_ERROR_NOT_FOUND => NotFound,
            nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE => InsufficientSize(None),
            nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_POWER => InsufficientPower,
            nvmlReturn_enum_NVML_ERROR_DRIVER_NOT_LOADED => DriverNotLoaded,
            nvmlReturn_enum_NVML_ERROR_TIMEOUT => Timeout,
            nvmlReturn_enum_NVML_ERROR_IRQ_ISSUE => IrqIssue,
            nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND => LibraryNotFound,
            nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND => FunctionNotFound,
            nvmlReturn_enum_NVML_ERROR_CORRUPTED_INFOROM => CorruptedInfoROM,
            nvmlReturn_enum_NVML_ERROR_GPU_IS_LOST => GpuLost,
            nvmlReturn_enum_NVML_ERROR_RESET_REQUIRED => ResetRequired,
            nvmlReturn_enum_NVML_ERROR_OPERATING_SYSTEM => OperatingSystem,
            nvmlReturn_enum_NVML_ERROR_LIB_RM_VERSION_MISMATCH => LibRmVersionMismatch,
            nvmlReturn_enum_NVML_ERROR_IN_USE => InUse,
            nvmlReturn_enum_NVML_ERROR_MEMORY => InsufficientMemory,
            nvmlReturn_enum_NVML_ERROR_NO_DATA => NoData,
            nvmlReturn_enum_NVML_ERROR_VGPU_ECC_NOT_SUPPORTED => VgpuEccNotSupported,
            nvmlReturn_enum_NVML_ERROR_UNKNOWN => Unknown,
            _ => UnexpectedVariant(value),
        }
    }
}

#[allow(deprecated)]
impl From<NvmlError> for nvmlReturn_t {
    fn from(error: NvmlError) -> Self {
        use NvmlError::*;

        match error {
            Uninitialized => nvmlReturn_enum_NVML_ERROR_UNINITIALIZED,
            InvalidArg => nvmlReturn_enum_NVML_ERROR_INVALID_ARGUMENT,
            NotSupported => nvmlReturn_enum_NVML_ERROR_NOT_SUPPORTED,
            NoPermission => nvmlReturn_enum_NVML_ERROR_NO_PERMISSION,
            AlreadyInitialized => nvmlReturn_enum_NVML_ERROR_ALREADY_INITIALIZED,
            NotFound => nvmlReturn_enum_NVML_ERROR_NOT_FOUND,
            InsufficientSize(_) => nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_SIZE,
            InsufficientPower => nvmlReturn_enum_NVML_ERROR_INSUFFICIENT_POWER,
            DriverNotLoaded => nvmlReturn_enum_NVML_ERROR_DRIVER_NOT_LOADED,
            Timeout => nvmlReturn_enum_NVML_ERROR_TIMEOUT,
            IrqIssue => nvmlReturn_enum_NVML_ERROR_IRQ_ISSUE,
            LibraryNotFound => nvmlReturn_enum_NVML_ERROR_LIBRARY_NOT_FOUND,
            FunctionNotFound => nvmlReturn_enum_NVML_ERROR_FUNCTION_NOT_FOUND,
            CorruptedInfoROM => nvmlReturn_enum_NVML_ERROR_CORRUPTED_INFOROM,
            GpuLost => nvmlReturn_enum_NVML_ERROR_GPU_IS_LOST,
            ResetRequired => nvmlReturn_enum_NVML_ERROR_RESET_REQUIRED,
            OperatingSystem => nvmlReturn_enum_NVML_ERROR_OPERATING_SYSTEM,
            LibRmVersionMismatch => nvmlReturn_enum_NVML_ERROR_LIB_RM_VERSION_MISMATCH,
            InUse => nvmlReturn_enum_NVML_ERROR_IN_USE,
            InsufficientMemory => nvmlReturn_enum_NVML_ERROR_MEMORY,
            NoData => nvmlReturn_enum_NVML_ERROR_NO_DATA,
            VgpuEccNotSupported => nvmlReturn_enum_NVML_ERROR_VGPU_ECC_NOT_SUPPORTED,
            Unknown => nvmlReturn_enum_NVML_ERROR_UNKNOWN,
            UnexpectedVariant(code) => code,
            // For non-NVML errors, return UNKNOWN
            Utf8Error(_)
            | NulError(_)
            | LibloadingError(_)
            | FailedToLoadSymbol(_)
            | StringTooLong { .. }
            | IncorrectBits(_)
            | SetReleaseFailed
            | GetPciInfoFailed
            | PciInfoToCFailed => nvmlReturn_enum_NVML_ERROR_UNKNOWN,
        }
    }
}

/// Helper to map a `&libloading::Error` into an `NvmlError`
pub fn nvml_sym<'a, T>(sym: Result<&'a T, &libloading::Error>) -> Result<&'a T, NvmlError> {
    sym.map_err(|e| NvmlError::FailedToLoadSymbol(e.to_string()))
}
