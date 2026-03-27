use crate::error::NvmlError;
use ffi::bindings::*;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};
use wrapcenum_derive::EnumWrapper;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VmId {
    Domain(String),
    Uuid(String),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(u32)]
pub enum VgpuLicenseState {
    Unknown = NVML_GRID_LICENSE_STATE_UNKNOWN,
    Uninitialized = NVML_GRID_LICENSE_STATE_UNINITIALIZED,
    UnlicensedUnrestricted = NVML_GRID_LICENSE_STATE_UNLICENSED_UNRESTRICTED,
    UnlicensedRestricted = NVML_GRID_LICENSE_STATE_UNLICENSED_RESTRICTED,
    Unlicensed = NVML_GRID_LICENSE_STATE_UNLICENSED,
    Licensed = NVML_GRID_LICENSE_STATE_LICENSED,
}

impl From<u32> for VgpuLicenseState {
    fn from(value: u32) -> Self {
        match value {
            NVML_GRID_LICENSE_STATE_UNINITIALIZED => Self::Uninitialized,
            NVML_GRID_LICENSE_STATE_UNLICENSED_UNRESTRICTED => Self::UnlicensedUnrestricted,
            NVML_GRID_LICENSE_STATE_UNLICENSED_RESTRICTED => Self::UnlicensedRestricted,
            NVML_GRID_LICENSE_STATE_UNLICENSED => Self::Unlicensed,
            NVML_GRID_LICENSE_STATE_LICENSED => Self::Licensed,
            _ => Self::Unknown,
        }
    }
}

//nvmlVgpuGuestInfoState_enum
#[derive(EnumWrapper, Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wrap(c_enum = "nvmlVgpuGuestInfoState_enum")]
pub enum VgpuGuestInfoState {
    #[wrap(c_variant = "NVML_VGPU_INSTANCE_GUEST_INFO_STATE_UNINITIALIZED")]
    Uninitialized,
    #[wrap(c_variant = "NVML_VGPU_INSTANCE_GUEST_INFO_STATE_INITIALIZED")]
    Initialized,
}
