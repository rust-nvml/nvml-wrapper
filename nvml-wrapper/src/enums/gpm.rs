use std::convert::TryFrom;
use std::os::raw::c_uint;

use crate::error::NvmlError;
use crate::ffi::bindings::*;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

/// Identifies a specific GPU performance metric that can be collected via the GPM API.
///
/// GPM (GPU Performance Monitoring) metrics are available on Hopper+ architectures.
/// Metrics are computed by taking two time-separated samples and comparing them.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GpmMetricId {
    /// Percentage of time any warp was active on a multiprocessor, averaged over all multiprocessors.
    GraphicsUtil,
    /// Percentage of time each multiprocessor had at least 1 warp assigned, averaged over all multiprocessors.
    SmUtil,
    /// Percentage of warps that were active vs theoretical maximum, averaged over all multiprocessors.
    SmOccupancy,
    /// Percentage of time the GPU's SMs were doing integer operations.
    IntegerUtil,
    /// Percentage of time the GPU's SMs were doing any tensor operations.
    AnyTensorUtil,
    /// Percentage of time the GPU's SMs were doing DFMA tensor operations.
    DfmaTensorUtil,
    /// Percentage of time the GPU's SMs were doing HMMA tensor operations.
    HmmaTensorUtil,
    /// Percentage of time the GPU's SMs were doing IMMA tensor operations.
    ImmaTensorUtil,
    /// Percentage of DRAM bandwidth used.
    DramBwUtil,
    /// Percentage of time the GPU's SMs were doing FP64 operations.
    Fp64Util,
    /// Percentage of time the GPU's SMs were doing FP32 operations.
    Fp32Util,
    /// Percentage of time the GPU's SMs were doing FP16 operations.
    Fp16Util,
    /// PCIe bytes transmitted per second.
    PcieTxPerSec,
    /// PCIe bytes received per second.
    PcieRxPerSec,
    /// NVDEC instance 0 utilization.
    Nvdec0Util,
    /// NVDEC instance 1 utilization.
    Nvdec1Util,
    /// NVDEC instance 2 utilization.
    Nvdec2Util,
    /// NVDEC instance 3 utilization.
    Nvdec3Util,
    /// NVDEC instance 4 utilization.
    Nvdec4Util,
    /// NVDEC instance 5 utilization.
    Nvdec5Util,
    /// NVDEC instance 6 utilization.
    Nvdec6Util,
    /// NVDEC instance 7 utilization.
    Nvdec7Util,
    /// NVJPG instance 0 utilization.
    Nvjpg0Util,
    /// NVJPG instance 1 utilization.
    Nvjpg1Util,
    /// NVJPG instance 2 utilization.
    Nvjpg2Util,
    /// NVJPG instance 3 utilization.
    Nvjpg3Util,
    /// NVJPG instance 4 utilization.
    Nvjpg4Util,
    /// NVJPG instance 5 utilization.
    Nvjpg5Util,
    /// NVJPG instance 6 utilization.
    Nvjpg6Util,
    /// NVJPG instance 7 utilization.
    Nvjpg7Util,
    /// NVOFA instance 0 utilization.
    Nvofa0Util,
    /// NVOFA instance 1 utilization.
    Nvofa1Util,
    /// Total NVLink receive bytes per second across all links.
    NvlinkTotalRxPerSec,
    /// Total NVLink transmit bytes per second across all links.
    NvlinkTotalTxPerSec,
    /// NVLink link 0 receive bytes per second.
    NvlinkL0RxPerSec,
    /// NVLink link 0 transmit bytes per second.
    NvlinkL0TxPerSec,
    /// NVLink link 1 receive bytes per second.
    NvlinkL1RxPerSec,
    /// NVLink link 1 transmit bytes per second.
    NvlinkL1TxPerSec,
    /// NVLink link 2 receive bytes per second.
    NvlinkL2RxPerSec,
    /// NVLink link 2 transmit bytes per second.
    NvlinkL2TxPerSec,
    /// NVLink link 3 receive bytes per second.
    NvlinkL3RxPerSec,
    /// NVLink link 3 transmit bytes per second.
    NvlinkL3TxPerSec,
    /// NVLink link 4 receive bytes per second.
    NvlinkL4RxPerSec,
    /// NVLink link 4 transmit bytes per second.
    NvlinkL4TxPerSec,
    /// NVLink link 5 receive bytes per second.
    NvlinkL5RxPerSec,
    /// NVLink link 5 transmit bytes per second.
    NvlinkL5TxPerSec,
    /// NVLink link 6 receive bytes per second.
    NvlinkL6RxPerSec,
    /// NVLink link 6 transmit bytes per second.
    NvlinkL6TxPerSec,
    /// NVLink link 7 receive bytes per second.
    NvlinkL7RxPerSec,
    /// NVLink link 7 transmit bytes per second.
    NvlinkL7TxPerSec,
    /// NVLink link 8 receive bytes per second.
    NvlinkL8RxPerSec,
    /// NVLink link 8 transmit bytes per second.
    NvlinkL8TxPerSec,
    /// NVLink link 9 receive bytes per second.
    NvlinkL9RxPerSec,
    /// NVLink link 9 transmit bytes per second.
    NvlinkL9TxPerSec,
    /// NVLink link 10 receive bytes per second.
    NvlinkL10RxPerSec,
    /// NVLink link 10 transmit bytes per second.
    NvlinkL10TxPerSec,
    /// NVLink link 11 receive bytes per second.
    NvlinkL11RxPerSec,
    /// NVLink link 11 transmit bytes per second.
    NvlinkL11TxPerSec,
    /// NVLink link 12 receive bytes per second.
    NvlinkL12RxPerSec,
    /// NVLink link 12 transmit bytes per second.
    NvlinkL12TxPerSec,
    /// NVLink link 13 receive bytes per second.
    NvlinkL13RxPerSec,
    /// NVLink link 13 transmit bytes per second.
    NvlinkL13TxPerSec,
    /// NVLink link 14 receive bytes per second.
    NvlinkL14RxPerSec,
    /// NVLink link 14 transmit bytes per second.
    NvlinkL14TxPerSec,
    /// NVLink link 15 receive bytes per second.
    NvlinkL15RxPerSec,
    /// NVLink link 15 transmit bytes per second.
    NvlinkL15TxPerSec,
    /// NVLink link 16 receive bytes per second.
    NvlinkL16RxPerSec,
    /// NVLink link 16 transmit bytes per second.
    NvlinkL16TxPerSec,
    /// NVLink link 17 receive bytes per second.
    NvlinkL17RxPerSec,
    /// NVLink link 17 transmit bytes per second.
    NvlinkL17TxPerSec,
}

impl GpmMetricId {
    /// Returns the C constant equivalent for the given Rust enum variant.
    pub fn as_c(&self) -> c_uint {
        match *self {
            Self::GraphicsUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_GRAPHICS_UTIL,
            Self::SmUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_SM_UTIL,
            Self::SmOccupancy => nvmlGpmMetricId_t_NVML_GPM_METRIC_SM_OCCUPANCY,
            Self::IntegerUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_INTEGER_UTIL,
            Self::AnyTensorUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_ANY_TENSOR_UTIL,
            Self::DfmaTensorUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_DFMA_TENSOR_UTIL,
            Self::HmmaTensorUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_HMMA_TENSOR_UTIL,
            Self::ImmaTensorUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_IMMA_TENSOR_UTIL,
            Self::DramBwUtil => nvmlGpmMetricId_t_NVML_GPM_METRIC_DRAM_BW_UTIL,
            Self::Fp64Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_FP64_UTIL,
            Self::Fp32Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_FP32_UTIL,
            Self::Fp16Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_FP16_UTIL,
            Self::PcieTxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_PCIE_TX_PER_SEC,
            Self::PcieRxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_PCIE_RX_PER_SEC,
            Self::Nvdec0Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_0_UTIL,
            Self::Nvdec1Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_1_UTIL,
            Self::Nvdec2Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_2_UTIL,
            Self::Nvdec3Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_3_UTIL,
            Self::Nvdec4Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_4_UTIL,
            Self::Nvdec5Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_5_UTIL,
            Self::Nvdec6Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_6_UTIL,
            Self::Nvdec7Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_7_UTIL,
            Self::Nvjpg0Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_0_UTIL,
            Self::Nvjpg1Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_1_UTIL,
            Self::Nvjpg2Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_2_UTIL,
            Self::Nvjpg3Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_3_UTIL,
            Self::Nvjpg4Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_4_UTIL,
            Self::Nvjpg5Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_5_UTIL,
            Self::Nvjpg6Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_6_UTIL,
            Self::Nvjpg7Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_7_UTIL,
            Self::Nvofa0Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVOFA_0_UTIL,
            Self::Nvofa1Util => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVOFA_1_UTIL,
            Self::NvlinkTotalRxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC,
            Self::NvlinkTotalTxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC,
            Self::NvlinkL0RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC,
            Self::NvlinkL0TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC,
            Self::NvlinkL1RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC,
            Self::NvlinkL1TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC,
            Self::NvlinkL2RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC,
            Self::NvlinkL2TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC,
            Self::NvlinkL3RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC,
            Self::NvlinkL3TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC,
            Self::NvlinkL4RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC,
            Self::NvlinkL4TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC,
            Self::NvlinkL5RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC,
            Self::NvlinkL5TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC,
            Self::NvlinkL6RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC,
            Self::NvlinkL6TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC,
            Self::NvlinkL7RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC,
            Self::NvlinkL7TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC,
            Self::NvlinkL8RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC,
            Self::NvlinkL8TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC,
            Self::NvlinkL9RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC,
            Self::NvlinkL9TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC,
            Self::NvlinkL10RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC,
            Self::NvlinkL10TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC,
            Self::NvlinkL11RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC,
            Self::NvlinkL11TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC,
            Self::NvlinkL12RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC,
            Self::NvlinkL12TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC,
            Self::NvlinkL13RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC,
            Self::NvlinkL13TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC,
            Self::NvlinkL14RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC,
            Self::NvlinkL14TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC,
            Self::NvlinkL15RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC,
            Self::NvlinkL15TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC,
            Self::NvlinkL16RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC,
            Self::NvlinkL16TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC,
            Self::NvlinkL17RxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC,
            Self::NvlinkL17TxPerSec => nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC,
        }
    }
}

impl TryFrom<c_uint> for GpmMetricId {
    type Error = NvmlError;

    fn try_from(data: c_uint) -> Result<Self, Self::Error> {
        match data {
            nvmlGpmMetricId_t_NVML_GPM_METRIC_GRAPHICS_UTIL => Ok(Self::GraphicsUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_SM_UTIL => Ok(Self::SmUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_SM_OCCUPANCY => Ok(Self::SmOccupancy),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_INTEGER_UTIL => Ok(Self::IntegerUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_ANY_TENSOR_UTIL => Ok(Self::AnyTensorUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_DFMA_TENSOR_UTIL => Ok(Self::DfmaTensorUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_HMMA_TENSOR_UTIL => Ok(Self::HmmaTensorUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_IMMA_TENSOR_UTIL => Ok(Self::ImmaTensorUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_DRAM_BW_UTIL => Ok(Self::DramBwUtil),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_FP64_UTIL => Ok(Self::Fp64Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_FP32_UTIL => Ok(Self::Fp32Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_FP16_UTIL => Ok(Self::Fp16Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_PCIE_TX_PER_SEC => Ok(Self::PcieTxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_PCIE_RX_PER_SEC => Ok(Self::PcieRxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_0_UTIL => Ok(Self::Nvdec0Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_1_UTIL => Ok(Self::Nvdec1Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_2_UTIL => Ok(Self::Nvdec2Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_3_UTIL => Ok(Self::Nvdec3Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_4_UTIL => Ok(Self::Nvdec4Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_5_UTIL => Ok(Self::Nvdec5Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_6_UTIL => Ok(Self::Nvdec6Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVDEC_7_UTIL => Ok(Self::Nvdec7Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_0_UTIL => Ok(Self::Nvjpg0Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_1_UTIL => Ok(Self::Nvjpg1Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_2_UTIL => Ok(Self::Nvjpg2Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_3_UTIL => Ok(Self::Nvjpg3Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_4_UTIL => Ok(Self::Nvjpg4Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_5_UTIL => Ok(Self::Nvjpg5Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_6_UTIL => Ok(Self::Nvjpg6Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVJPG_7_UTIL => Ok(Self::Nvjpg7Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVOFA_0_UTIL => Ok(Self::Nvofa0Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVOFA_1_UTIL => Ok(Self::Nvofa1Util),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_TOTAL_RX_PER_SEC => {
                Ok(Self::NvlinkTotalRxPerSec)
            }
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_TOTAL_TX_PER_SEC => {
                Ok(Self::NvlinkTotalTxPerSec)
            }
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L0_RX_PER_SEC => Ok(Self::NvlinkL0RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L0_TX_PER_SEC => Ok(Self::NvlinkL0TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L1_RX_PER_SEC => Ok(Self::NvlinkL1RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L1_TX_PER_SEC => Ok(Self::NvlinkL1TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L2_RX_PER_SEC => Ok(Self::NvlinkL2RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L2_TX_PER_SEC => Ok(Self::NvlinkL2TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L3_RX_PER_SEC => Ok(Self::NvlinkL3RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L3_TX_PER_SEC => Ok(Self::NvlinkL3TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L4_RX_PER_SEC => Ok(Self::NvlinkL4RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L4_TX_PER_SEC => Ok(Self::NvlinkL4TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L5_RX_PER_SEC => Ok(Self::NvlinkL5RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L5_TX_PER_SEC => Ok(Self::NvlinkL5TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L6_RX_PER_SEC => Ok(Self::NvlinkL6RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L6_TX_PER_SEC => Ok(Self::NvlinkL6TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L7_RX_PER_SEC => Ok(Self::NvlinkL7RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L7_TX_PER_SEC => Ok(Self::NvlinkL7TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L8_RX_PER_SEC => Ok(Self::NvlinkL8RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L8_TX_PER_SEC => Ok(Self::NvlinkL8TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L9_RX_PER_SEC => Ok(Self::NvlinkL9RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L9_TX_PER_SEC => Ok(Self::NvlinkL9TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L10_RX_PER_SEC => Ok(Self::NvlinkL10RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L10_TX_PER_SEC => Ok(Self::NvlinkL10TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L11_RX_PER_SEC => Ok(Self::NvlinkL11RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L11_TX_PER_SEC => Ok(Self::NvlinkL11TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L12_RX_PER_SEC => Ok(Self::NvlinkL12RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L12_TX_PER_SEC => Ok(Self::NvlinkL12TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L13_RX_PER_SEC => Ok(Self::NvlinkL13RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L13_TX_PER_SEC => Ok(Self::NvlinkL13TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L14_RX_PER_SEC => Ok(Self::NvlinkL14RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L14_TX_PER_SEC => Ok(Self::NvlinkL14TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L15_RX_PER_SEC => Ok(Self::NvlinkL15RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L15_TX_PER_SEC => Ok(Self::NvlinkL15TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L16_RX_PER_SEC => Ok(Self::NvlinkL16RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L16_TX_PER_SEC => Ok(Self::NvlinkL16TxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L17_RX_PER_SEC => Ok(Self::NvlinkL17RxPerSec),
            nvmlGpmMetricId_t_NVML_GPM_METRIC_NVLINK_L17_TX_PER_SEC => Ok(Self::NvlinkL17TxPerSec),
            _ => Err(NvmlError::UnexpectedVariant(data)),
        }
    }
}
