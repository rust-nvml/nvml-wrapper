#[cfg(target_os = "windows")]
use crate::enum_wrappers::device::DriverModel;
use crate::enum_wrappers::device::OperationMode;
use ffi::bindings::{
    NVML_CC_GPU_ATTESTATION_REPORT_SIZE, NVML_CC_GPU_CEC_ATTESTATION_REPORT_SIZE,
    NVML_GPU_ATTESTATION_CERT_CHAIN_SIZE, NVML_GPU_CERT_CHAIN_SIZE,
};
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

/// Returned from `Device.auto_boosted_clocks_enabled()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AutoBoostClocksEnabledInfo {
    /// Current state of auto boosted clocks for the `Device`
    pub is_enabled: bool,
    /// Default auto boosted clocks behavior for the `Device`
    ///
    /// The GPU will revert to this default when no applications are using the
    /// GPU.
    pub is_enabled_default: bool,
}

/// Returned from `Device.decoder_utilization()` and
/// `Device.encoder_utilization()`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UtilizationInfo {
    pub utilization: u32,
    /// Sampling period in μs.
    pub sampling_period: u32,
}

/// Returned from `Device.driver_model()`
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg(target_os = "windows")]
pub struct DriverModelState {
    pub current: DriverModel,
    pub pending: DriverModel,
}

/// Returned from `Device.is_ecc_enabled()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EccModeState {
    pub currently_enabled: bool,
    pub pending_enabled: bool,
}

/// Returned from `Device.gpu_operation_mode()`
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OperationModeState {
    pub current: OperationMode,
    pub pending: OperationMode,
}

/// Returned from `Device.power_management_limit_constraints()`
///
/// Values are in milliwatts.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PowerManagementConstraints {
    pub min_limit: u32,
    pub max_limit: u32,
}

/// Returned from `Device.encoder_stats()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EncoderStats {
    /// The number of active encoder sessions.
    pub session_count: u32,
    /// The trailing average FPS of all active encoder sessions.
    pub average_fps: u32,
    /// The encode latency in μs.
    pub average_latency: u32,
}

/// Returned from `Device.cuda_compute_capability()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CudaComputeCapability {
    pub major: i32,
    pub minor: i32,
}

/// Returned from `Device.retired_pages()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RetiredPage {
    /// The hardware address of the page that was retired.
    ///
    /// Note that this does not match the virtual address used in CUDA but does
    /// match the address information in XID 63.
    pub address: u64,
    /// The retirement timestamp.
    pub timestamp: u64,
}

/// Populate this newtype with the constants `nvml_wrapper::sys_exports::field_id::*`.
///
/// Used in `FieldValue` and `Device.field_values_for()`.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldId(pub u32);

/// Returned from `Device.get_confidential_compute_capabilities()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfidentialComputeCapabilities {
    /// The CPU capabilities.
    pub cpu_caps: ConfidentialComputeCpuCapabilities,
    /// The GPU capabilities.
    pub gpus_caps: ConfidentialComputeGpuCapabilities,
}

/// The possible CPU capabilities for confidential compute (either None, AMD SEV or Intel TDX)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConfidentialComputeCpuCapabilities {
    /// No CPU capabilities.
    None,
    /// AMD SEV confidential compute capabilities.
    AmdSev,
    /// Intel TDX confidential compute capabilities.
    IntelTdx,
}

/// The possible GPU capabilities for confidential compute (either not capable or capable)
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ConfidentialComputeGpuCapabilities {
    /// Capable.
    Capable,
    /// Not capable.
    NotCapable,
}

/// Returned from `Device.confidential_compute_gpu_certificate()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfidentialComputeGpuCertificate {
    /// The size of the certificate chain.
    pub cert_chain_size: u32,
    /// The size of the attestation certificate chain.
    pub attestation_cert_chain_size: u32,
    /// The certificate chain.
    pub cert_chain: [u8; NVML_GPU_CERT_CHAIN_SIZE as usize],
    /// The attestation certificate chain.
    pub attestation_cert_chain: [u8; NVML_GPU_ATTESTATION_CERT_CHAIN_SIZE as usize],
}

/// Returned from `Device.confidential_compute_gpu_attestation_report_bytes()`
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ConfidentialComputeGpuAttestationReport {
    /// The size of the attestation report.
    pub attestation_report_size: u32,
    /// The attestation report.
    pub attestation_report: [u8; NVML_CC_GPU_ATTESTATION_REPORT_SIZE as usize],
    /// Whether the CEC attestation report is present.
    pub is_cec_attestation_report_present: bool,
    /// The size of the CEC attestation report.
    pub cec_attestation_report_size: u32,
    /// The CEC attestation report.
    pub cec_attestation_report: [u8; NVML_CC_GPU_CEC_ATTESTATION_REPORT_SIZE as usize],
}
