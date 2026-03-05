use std::os::raw::c_char;

use ffi::bindings::*;

use crate::{
    enum_wrappers::vgpu::{VgpuGuestInfoState, VgpuLicenseState},
    error::NvmlError,
};

pub struct VgpuLicenseInfo {
    pub is_licensed: bool,
    pub expiry: VgpuLicenseExpiry,
    pub state: VgpuLicenseState,
}

impl From<nvmlVgpuLicenseInfo_t> for VgpuLicenseInfo {
    fn from(value: nvmlVgpuLicenseInfo_t) -> Self {
        Self {
            is_licensed: value.isLicensed != 0,
            expiry: VgpuLicenseExpiry::from(value.licenseExpiry),
            state: VgpuLicenseState::from(value.currentState),
        }
    }
}

pub struct VgpuLicenseExpiry {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub status: u8,
}

impl From<nvmlVgpuLicenseExpiry_t> for VgpuLicenseExpiry {
    fn from(value: nvmlVgpuLicenseExpiry_t) -> Self {
        Self {
            year: u16::try_from(value.year).unwrap_or(u16::MAX),
            month: u8::try_from(value.month).unwrap_or(u8::MAX),
            day: u8::try_from(value.day).unwrap_or(u8::MAX),
            hour: u8::try_from(value.hour).unwrap_or(u8::MAX),
            min: u8::try_from(value.min).unwrap_or(u8::MAX),
            sec: u8::try_from(value.sec).unwrap_or(u8::MAX),
            status: value.status,
        }
    }
}

pub struct VgpuMetadata {
    pub version: u32,
    pub revision: u32,
    pub guest_info_state: VgpuGuestInfoState,
    pub guest_driver_version: String,
    pub host_driver_version: String,
    pub vgpu_virtualization_caps: u32,
    pub guest_vgpu_version: u32,
}

impl TryFrom<nvmlVgpuMetadata_t> for VgpuMetadata {
    type Error = NvmlError;
    fn try_from(value: nvmlVgpuMetadata_t) -> Result<Self, Self::Error> {
        let convert_c_str = |c_str: &[c_char]| {
            let mut ret = String::with_capacity(c_str.len());
            for &byte in c_str {
                if byte == 0 {
                    break;
                }
                let us = u8::try_from(byte).map_err(|_| NvmlError::Unknown)?;
                ret.push(us as char);
            }

            Ok::<String, NvmlError>(ret)
        };

        Ok(Self {
            version: value.version,
            revision: value.revision,
            guest_driver_version: convert_c_str(&value.guestDriverVersion)?,
            host_driver_version: convert_c_str(&value.hostDriverVersion)?,
            vgpu_virtualization_caps: value.vgpuVirtualizationCaps,
            guest_vgpu_version: value.guestVgpuVersion,
            guest_info_state: value.guestInfoState.try_into()?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VgpuPlacementId {
    pub version: u32,
    pub id: u32,
}

impl From<nvmlVgpuPlacementId_t> for VgpuPlacementId {
    fn from(value: nvmlVgpuPlacementId_t) -> Self {
        Self {
            version: value.version,
            id: value.placementId,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct VgpuRuntimeState {
    pub version: u32,
    pub size: u64,
}

impl From<nvmlVgpuRuntimeState_t> for VgpuRuntimeState {
    fn from(value: nvmlVgpuRuntimeState_t) -> Self {
        Self {
            version: value.version,
            size: value.size,
        }
    }
}

pub struct Bar1Info {
    pub version: u32,
    pub size: u64,
}

impl From<nvmlVgpuTypeBar1Info_v1_t> for Bar1Info {
    fn from(value: nvmlVgpuTypeBar1Info_v1_t) -> Self {
        Self {
            version: value.version,
            size: value.bar1Size,
        }
    }
}
