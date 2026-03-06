use std::{
    ffi::CStr,
    os::raw::{c_char, c_uint},
};

use ffi::bindings::{
    nvmlEnableState_enum_NVML_FEATURE_ENABLED, nvmlEncoderSessionInfo_t, nvmlFBCSessionInfo_t,
    nvmlFBCStats_t, nvmlVgpuCapability_t, nvmlVgpuInstance_t, nvmlVgpuLicenseInfo_st,
    nvmlVgpuMetadata_t, nvmlVgpuPlacementId_t, nvmlVgpuRuntimeState_t, nvmlVgpuTypeBar1Info_v1_t,
    nvmlVgpuTypeId_t, nvmlVgpuVmIdType_NVML_VGPU_VM_ID_DOMAIN_ID,
    nvmlVgpuVmIdType_NVML_VGPU_VM_ID_UUID, NVML_DEVICE_NAME_BUFFER_SIZE,
    NVML_DEVICE_UUID_BUFFER_SIZE, NVML_GRID_LICENSE_BUFFER_SIZE,
    NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE,
};
use static_assertions::assert_impl_all;

use crate::{
    enum_wrappers::vgpu::VmId,
    error::{nvml_sym, nvml_try, nvml_try_count, NvmlError},
    struct_wrappers::{
        device::{EncoderSessionInfo, FbcSessionInfo, FbcStats},
        vgpu::{Bar1Info, VgpuLicenseInfo, VgpuMetadata, VgpuPlacementId, VgpuRuntimeState},
    },
    structs::device::EncoderStats,
    Device,
};

pub struct VgpuType<'dev> {
    id: nvmlVgpuTypeId_t,
    device: &'dev Device<'dev>,
}

assert_impl_all!(VgpuType: Send, Sync);

impl<'dev> VgpuType<'dev> {
    /// Create a new vGPU type wrapper.
    ///
    /// You probably don't need to use this yourself, but rather through
    /// [`Device::vgpu_supported_types`] and [`Device::vgpu_creatable_types`].
    pub fn new(device: &'dev Device, id: nvmlVgpuTypeId_t) -> Self {
        Self { id, device }
    }

    /// Access the `Device` this struct belongs to.
    ///
    pub fn device(&self) -> &'dev Device<'_> {
        self.device
    }

    /// Get the underlying vGPU type id.
    pub fn id(&self) -> nvmlVgpuTypeId_t {
        self.id
    }

    /// Retrieve the class of the vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetClass")]
    pub fn class_name(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetClass.as_ref())?;

        unsafe {
            let mut size = NVML_DEVICE_NAME_BUFFER_SIZE;
            let mut buffer = vec![0; size as usize];

            nvml_try(sym(self.id, buffer.as_mut_ptr(), &mut size))?;

            let version_raw = CStr::from_ptr(buffer.as_ptr());
            Ok(version_raw.to_str()?.into())
        }
    }

    /// Retrieve license requirements for a vGPU type.
    ///
    /// The license type and version required to run the specified vGPU type is returned as an
    /// alphanumeric string, in the form "\<license name\>,\<version\>", for example
    /// "GRID-Virtual-PC,2.0". If a vGPU is runnable with* more than one type of license, the
    /// licenses are delimited by a semicolon, for example
    /// "GRID-Virtual-PC,2.0;GRID-Virtual-WS,2.0;GRID-Virtual-WS-Ext,2.0".
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InsufficientSize`, if the passed-in `size` is 0 (must be > 0)
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetLicense")]
    pub fn license(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetLicense.as_ref())?;

        unsafe {
            let mut buffer = vec![0; NVML_GRID_LICENSE_BUFFER_SIZE as usize];

            nvml_try(sym(self.id, buffer.as_mut_ptr(), buffer.len() as u32))?;

            let version_raw = CStr::from_ptr(buffer.as_ptr());
            Ok(version_raw.to_str()?.into())
        }
    }

    /// Retrieve the name of the vGPU type.
    ///
    /// The name is an alphanumeric string that denotes a particular vGPU, e.g. GRID M60-2Q.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetName")]
    pub fn name(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetName.as_ref())?;

        unsafe {
            let mut size = NVML_DEVICE_NAME_BUFFER_SIZE;
            let mut buffer = vec![0; size as usize];

            nvml_try(sym(self.id, buffer.as_mut_ptr(), &mut size))?;

            let version_raw = CStr::from_ptr(buffer.as_ptr());
            Ok(version_raw.to_str()?.into())
        }
    }

    /// Retrieve the requested capability for a given vGPU type. Refer to the
    /// `nvmlVgpuCapability_t` structure for the specific capabilities that can be
    /// queried.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetCapabilities")]
    pub fn capabilities(&self, capability: nvmlVgpuCapability_t) -> Result<bool, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetCapabilities.as_ref())?;

        let mut result: c_uint = 0;
        unsafe {
            nvml_try(sym(self.id, capability, &mut result))?;
        }
        Ok(result != 0)
    }

    /// Retrieve the device ID of the vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetDeviceID")]
    pub fn device_id(&self) -> Result<(u64, u64), NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetDeviceID.as_ref())?;

        let (mut device_id, mut subsystem_id) = (0, 0);
        unsafe {
            nvml_try(sym(self.id, &mut device_id, &mut subsystem_id))?;
        }
        Ok((device_id, subsystem_id))
    }

    /// Retrieve the static frame rate limit value of the vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotSupported`, if frame rate limiter is turned off for the vGPU type
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetFrameRateLimit")]
    pub fn frame_rate_limit(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuTypeGetFrameRateLimit
                .as_ref(),
        )?;

        let mut limit = 0;
        unsafe {
            nvml_try(sym(self.id, &mut limit))?;
        }
        Ok(limit)
    }

    /// Retrieve the vGPU framebuffer size in bytes.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetFramebufferSize")]
    pub fn framebuffer_size(&self) -> Result<u64, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuTypeGetFramebufferSize
                .as_ref(),
        )?;

        let mut size = 0;
        unsafe {
            nvml_try(sym(self.id, &mut size))?;
        }
        Ok(size)
    }

    /// Retrieve the GPU Instance Profile ID for the vGPU type. The API will return a valid GPU
    /// Instance Profile ID for the MIG capable vGPU types, else
    /// [`crate::ffi::bindings::INVALID_GPU_INSTANCE_PROFILE_ID`] is returned.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetGpuInstanceProfileId")]
    pub fn instance_profile_id(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuTypeGetGpuInstanceProfileId
                .as_ref(),
        )?;

        let mut profile_id = 0;
        unsafe {
            nvml_try(sym(self.id, &mut profile_id))?;
        }
        Ok(profile_id)
    }

    /// Retrieve the maximum number of vGPU instances creatable on a device for the vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetMaxInstances")]
    pub fn max_instances(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetMaxInstances.as_ref())?;

        let mut max = 0;
        unsafe {
            nvml_try(sym(self.device.handle(), self.id, &mut max))?;
        }
        Ok(max)
    }

    /// Retrieve the maximum number of vGPU instances supported per VM for the vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetMaxInstancesPerVm")]
    pub fn max_instances_per_vm(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuTypeGetMaxInstancesPerVm
                .as_ref(),
        )?;

        let mut max = 0;
        unsafe {
            nvml_try(sym(self.id, &mut max))?;
        }
        Ok(max)
    }

    /// Retrieve count of vGPU's supported display heads.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetNumDisplayHeads")]
    pub fn num_display_heads(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuTypeGetNumDisplayHeads
                .as_ref(),
        )?;

        let mut heads = 0;
        unsafe {
            nvml_try(sym(self.id, &mut heads))?;
        }
        Ok(heads)
    }

    /// Retrieve vGPU display head's maximum supported resolution.
    ///
    /// The `display_head` argument specifies the 0-based display index, the
    /// maximum being what [`VgpuType::num_display_heads`] returns.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error
    ///
    /// # Device Support
    ///
    /// Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetResolution")]
    pub fn resolution(&self, display_head: u32) -> Result<(u32, u32), NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetResolution.as_ref())?;

        let (mut x, mut y) = (0, 0);
        unsafe {
            nvml_try(sym(self.id, display_head, &mut x, &mut y))?;
        }
        Ok((x, y))
    }

    /// Retrieve the BAR1 info for given vGPU type.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuTypeGetBAR1Info")]
    pub fn get_bar1_info(&self) -> Result<Bar1Info, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetBAR1Info.as_ref())?;
        let mut info: nvmlVgpuTypeBar1Info_v1_t;
        unsafe {
            info = std::mem::zeroed();
            nvml_try(sym(self.id, &mut info))?;
        }
        Ok(info.into())
    }

    /// Retrieve the static framebuffer reservation of the vGPU type in bytes
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    ///
    #[doc(alias = "nvmlVgpuTypeGetFbReservation")]
    pub fn get_fb_reservation(&self) -> Result<u64, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetFbReservation.as_ref())?;
        let mut res = 0;
        unsafe {
            nvml_try(sym(self.id, &mut res))?;
        }
        Ok(res)
    }

    /// Retrieve the static GSP heap size of the vGPU type in bytes
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    #[doc(alias = "nvmlVgpuTypeGetGspHeapSize")]
    pub fn get_gsp_heap_size(&self) -> Result<u64, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuTypeGetGspHeapSize.as_ref())?;
        let mut res = 0;
        unsafe {
            nvml_try(sym(self.id, &mut res))?;
        }
        Ok(res)
    }
}

pub struct VgpuInstance<'dev> {
    pub(crate) instance: nvmlVgpuInstance_t,
    device: &'dev Device<'dev>,
}

assert_impl_all!(VgpuInstance: Send, Sync);

impl<'dev> VgpuInstance<'dev> {
    pub(crate) fn new(instance: nvmlVgpuInstance_t, device: &'dev Device<'dev>) -> Self {
        Self { instance, device }
    }

    /// Retrieve the VM ID associated with a vGPU instance.
    ///
    /// The VM ID is returned as a string, not exceeding 80 characters in length (including the NUL
    /// terminator). See nvmlConstants::NVML_DEVICE_UUID_BUFFER_SIZE.
    ///
    /// The format of the VM ID varies by platform, and is indicated by the type identifier returned
    /// in vmIdType.
    ///
    /// # Errors
    ///
    /// * `Uninitialized` if the library has not been successfully initialized
    /// * `NotFound` if self does not match a valid active vGPU instance on the system
    /// * `Unknown` on any unexpected error
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetVmID")]
    pub fn get_vm_id(&self) -> Result<VmId, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetVmID.as_ref())?;
        let mut s = [0; NVML_DEVICE_UUID_BUFFER_SIZE as usize];
        let mut id_type = 0;
        let id = unsafe {
            nvml_try(sym(
                self.instance,
                s.as_mut_ptr(),
                NVML_DEVICE_UUID_BUFFER_SIZE,
                &mut id_type,
            ))?;
            CStr::from_ptr(s.as_ptr())
        };

        let id = id.to_str()?.to_string();
        Ok(match id_type {
            nvmlVgpuVmIdType_NVML_VGPU_VM_ID_DOMAIN_ID => VmId::Domain(id),
            nvmlVgpuVmIdType_NVML_VGPU_VM_ID_UUID => VmId::Uuid(id),
            _ => return Err(NvmlError::Unknown),
        })
    }

    /// Retrieve the framebuffer usage in bytes.
    ///
    /// Framebuffer usage is the amount of vGPU framebuffer memory that is currently in use by the VM
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg` if self is invalid
    /// * `NotFound` if self does not match a valid active vGPU instance on the system
    /// * `Unknown`, on any unexpected error
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetFbUsage")]
    pub fn get_fb_usage(&self) -> Result<u64, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetFbUsage.as_ref())?;
        let mut usage = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut usage))?;
        }
        Ok(usage)
    }

    /// Retrieve the vGPU type of a vGPU instance
    ///
    /// Returns the vGPU type ID of vgpu assigned to the vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg` if self is invalid
    /// * `NotFound` if self does not match a valid active vGPU instance on the system
    /// * `Unknown`, on any unexpected error
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices
    #[doc(alias = "nvmlVgpuInstanceGetType")]
    pub fn get_instance_type(&'dev self) -> Result<VgpuType<'dev>, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetType.as_ref())?;
        let mut raw_type = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut raw_type))?;
        }
        Ok(VgpuType::new(self.device, raw_type))
    }

    /// Get the list of process ids running on this vGPU instance for stats purpose
    ///
    /// see [`crate::device::Device::vgpu_accounting_pids`] for details
    #[doc(alias = "nvmlVgpuInstanceGetAccountingPids")]
    pub fn accounting_pids(&self) -> Result<Vec<u32>, NvmlError> {
        self.device.vgpu_accounting_pids(self.instance)
    }

    /// Clears accounting information of the vGPU instance that have already terminated.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NoPermission`, if the user doesn't have permission to perform this operation
    /// * `NotSupported`, if the vGPU doesn't support this feature or accounting mode is disabled
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices. Requires root/admin permissions.
    #[doc(alias = "nvmlVgpuInstanceClearAccountingPids")]
    pub fn clear_accounting_pids(&self) -> Result<(), NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceClearAccountingPids
                .as_ref(),
        )?;
        unsafe { nvml_try(sym(self.instance)) }
    }

    /// Queries the state of per process accounting mode on vGPU.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `NotSupported`, if the vGPU doesn't support this feature or accounting mode is disabled
    /// * `DriverNotLoaded`, driver is not running on the vGPU instance
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetAccountingMode")]
    pub fn get_accounting_mode(&self) -> Result<bool, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetAccountingMode
                .as_ref(),
        )?;
        let mut mode = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut mode))?;
        }
        Ok(mode == nvmlEnableState_enum_NVML_FEATURE_ENABLED)
    }

    /// Retrieve the current ECC mode of vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `NotSupported`, if the vGPU doesn't support this feature or accounting mode is disabled
    #[doc(alias = "nvmlVgpuInstanceGetEccMode")]
    pub fn get_ecc_mode(&self) -> Result<bool, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetEccMode.as_ref())?;
        let mut mode = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut mode))?;
        }
        Ok(mode == nvmlEnableState_enum_NVML_FEATURE_ENABLED)
    }

    /// Retrieve the encoder capacity of a vGPU instance, as a percentage of maximum encoder
    /// capacity with valid values in the range 0-100.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetEncoderCapacity")]
    pub fn get_encoder_capacity(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetEncoderCapacity
                .as_ref(),
        )?;
        let mut cap = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut cap))?;
        }
        Ok(cap)
    }

    /// Retrieves information about all active encoder sessions on a vGPU Instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetEncoderSessions")]
    pub fn get_encoder_sessions(&self) -> Result<Vec<EncoderSessionInfo>, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetEncoderSessions
                .as_ref(),
        )?;
        let mut count = self.get_encoder_session_count()?;
        let mut raw_sessions: Vec<nvmlEncoderSessionInfo_t>;
        unsafe {
            raw_sessions = vec![std::mem::zeroed(); count as usize];
            nvml_try(sym(self.instance, &mut count, raw_sessions.as_mut_ptr()))?;
        };
        raw_sessions
            .into_iter()
            .map(EncoderSessionInfo::try_from)
            .collect()
    }

    /// Retrieves the current encoder statistics of a vGPU Instance
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetEncoderStats")]
    pub fn get_encoder_stats(&self) -> Result<EncoderStats, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetEncoderStats
                .as_ref(),
        )?;
        let mut session_count = self.get_encoder_session_count()?;
        let mut average_fps = 0;
        let mut average_latency = 0;
        unsafe {
            nvml_try(sym(
                self.instance,
                &mut session_count,
                &mut average_fps,
                &mut average_latency,
            ))?;
        };
        Ok(EncoderStats {
            session_count,
            average_fps,
            average_latency,
        })
    }

    /// Retrieves information about active frame buffer capture sessions on a vGPU Instance.
    ///
    /// > hResolution, vResolution, averageFPS and averageLatency data for a FBC session
    /// > returned in sessionInfo may be zero if there are no new frames captured since the
    /// > session started.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetFBCSessions")]
    pub fn get_fbc_sessions(&self) -> Result<Vec<FbcSessionInfo>, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetFBCSessions
                .as_ref(),
        )?;
        let mut session_count = 0;
        let mut info: Vec<nvmlFBCSessionInfo_t>;
        unsafe {
            nvml_try_count(sym(self.instance, &mut session_count, std::ptr::null_mut()))?;
            if session_count == 0 {
                return Ok(Vec::new());
            }
            info = vec![std::mem::zeroed(); session_count as usize];
            nvml_try(sym(self.instance, &mut session_count, info.as_mut_ptr()))?;
        };
        info.into_iter().map(FbcSessionInfo::try_from).collect()
    }

    /// Retrieves the active frame buffer capture sessions statistics of a vGPU Instance
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetFBCStats")]
    pub fn get_fbc_stats(&self) -> Result<FbcStats, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetFBCStats.as_ref())?;
        unsafe {
            let mut info: nvmlFBCStats_t = std::mem::zeroed();
            nvml_try(sym(self.instance, &mut info))?;
            Ok(FbcStats::from(info))
        }
    }

    /// Retrieve the frame rate limit set for the vGPU instance.
    ///
    /// Returns the value of the frame rate limit set for the vGPU instance
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotSupported`, if frame rate limiter is turned off for the vGPU type
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetFrameRateLimit")]
    pub fn get_frame_rate_limit(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetFrameRateLimit
                .as_ref(),
        )?;
        let mut limit = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut limit))?;
        };
        Ok(limit)
    }

    /// Retrieve the GPU Instance ID for the given vGPU Instance. The API will return a valid GPU
    /// Instance ID for MIG backed vGPU Instance, else INVALID_GPU_INSTANCE_ID is returned.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetGpuInstanceId")]
    pub fn get_gpu_instance_id(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetGpuInstanceId
                .as_ref(),
        )?;
        let mut id = 0;
        unsafe {
            nvml_try(sym(self.instance, &mut id))?;
        };
        Ok(id)
    }

    /// Retrieves the PCI Id of the given vGPU Instance i.e. the PCI Id of the GPU as seen inside
    /// the VM.
    ///
    /// The vGPU PCI id is returned as "00000000:00:00.0" if NVIDIA driver is not installed on the
    /// vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `DriverNotLoaded`, driver is not running on the vGPU instance
    #[doc(alias = "nvmlVgpuInstanceGetGpuPciId")]
    pub fn get_gpu_pci_id(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetGpuPciId.as_ref())?;
        let mut buffer: Vec<c_char>;
        let mut count = 0;
        let raw_id = unsafe {
            nvml_try_count(sym(self.instance, [0; 1].as_mut_ptr(), &mut count))?;
            if count == 0 {
                return Ok(String::new());
            }
            buffer = vec![0; count as usize];
            nvml_try(sym(self.instance, buffer.as_mut_ptr(), &mut count))?;
            CStr::from_ptr(buffer.as_ptr())
        };
        Ok(raw_id.to_str()?.to_string())
    }

    /// Query the license information of the vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `DriverNotLoaded`, driver is not running on the vGPU instance
    #[cfg(feature = "legacy-functions")]
    #[doc(alias = "nvmlVgpuInstanceGetLicenseInfo")]
    pub fn get_license_info(&self) -> Result<VgpuLicenseInfo, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetLicenseInfo
                .as_ref(),
        )?;
        let mut info: nvmlVgpuLicenseInfo_st;

        unsafe {
            info = std::mem::zeroed();
            nvml_try(sym(self.instance, &mut info))?;
        };
        Ok(VgpuLicenseInfo::from(info))
    }

    /// Query the license information of the vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `DriverNotLoaded`, driver is not running on the vGPU instance
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetLicenseInfo_v2")]
    pub fn get_license_info_v2(&self) -> Result<VgpuLicenseInfo, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetLicenseInfo_v2
                .as_ref(),
        )?;
        let mut info: nvmlVgpuLicenseInfo_st;

        unsafe {
            info = std::mem::zeroed();
            nvml_try(sym(self.instance, &mut info))?;
        };
        Ok(VgpuLicenseInfo::from(info))
    }

    /// Retrieve the MDEV UUID of a vGPU instance.
    ///
    /// The MDEV UUID is a globally unique identifier of the mdev device assigned to the VM, and is
    /// returned as a 5-part hexadecimal string, not exceeding 80 characters in length (including
    /// the NULL terminator). MDEV UUID is displayed only on KVM platform.
    /// See nvmlConstants::NVML_DEVICE_UUID_BUFFER_SIZE.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    /// * `NotSupported`, on any hypervisor other than KVM
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetMdevUUID")]
    pub fn get_mdev_uuid(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetMdevUUID.as_ref())?;
        let mut buffer: [c_char; NVML_DEVICE_UUID_BUFFER_SIZE as usize] =
            [0; NVML_DEVICE_UUID_BUFFER_SIZE as usize];

        unsafe {
            nvml_try(sym(
                self.instance,
                buffer.as_mut_ptr(),
                NVML_DEVICE_UUID_BUFFER_SIZE,
            ))?;
            let raw_id = CStr::from_ptr(buffer.as_ptr());
            Ok(raw_id.to_str()?.to_string())
        }
    }

    /// Returns vGPU metadata structure for a running vGPU. The structure contains information
    /// about the vGPU and its associated VM such as the currently installed NVIDIA guest driver
    /// version, together with host driver version and an opaque data section containing internal
    /// state.
    ///
    /// May be called at any time for a vGPU instance. Some fields in the returned structure are
    /// dependent on information obtained from the guest VM, which may not yet have reached a state
    /// where that information is available.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    ///
    #[doc(alias = "nvmlVgpuInstanceGetMetadata")]
    pub fn get_metadata(&self) -> Result<Vec<VgpuMetadata>, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetMetadata.as_ref())?;
        let mut metadata: Vec<nvmlVgpuMetadata_t>;
        let mut count = 0;
        unsafe {
            nvml_try_count(sym(self.instance, std::ptr::null_mut(), &mut count))?;
            metadata = vec![std::mem::zeroed(); count as usize];
            nvml_try(sym(self.instance, metadata.as_mut_ptr(), &mut count))?;
        }
        metadata.into_iter().map(VgpuMetadata::try_from).collect()
    }

    /// Query the placement ID of active vGPU instance.
    ///
    /// When in vGPU heterogeneous mode, this function returns a valid placement ID as
    /// [`VgpuPlacementId::id`], [`VgpuPlacementId::version`] is the version number
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    #[doc(alias = "nvmlVgpuInstanceGetPlacementId")]
    pub fn get_get_placement_id(&self) -> Result<VgpuPlacementId, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetPlacementId
                .as_ref(),
        )?;
        let mut raw_placement_id: nvmlVgpuPlacementId_t;
        unsafe {
            raw_placement_id = std::mem::zeroed();
            nvml_try(sym(self.instance, &mut raw_placement_id))?;
        }
        Ok(raw_placement_id.into())
    }

    /// Retrieve the currently used runtime state size of the vGPU instance
    ///
    /// This size represents the maximum in-memory data size utilized by a vGPU instance during
    /// standard operation. This measurement is exclusive of frame buffer (FB) data size assigned
    /// to the vGPU instance.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetRuntimeStateSize")]
    pub fn get_get_runtime_state_size(&self) -> Result<VgpuRuntimeState, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetRuntimeStateSize
                .as_ref(),
        )?;
        let mut raw_state: nvmlVgpuRuntimeState_t;
        unsafe {
            raw_state = std::mem::zeroed();
            nvml_try(sym(self.instance, &mut raw_state))?;
        }
        Ok(raw_state.into())
    }

    /// Retrieve the UUID of a vGPU instance.
    ///
    /// The UUID is a globally unique identifier associated with the vGPU, and is returned as a 5-part hexadecimal string
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetUUID")]
    pub fn get_uuid(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(self.device.nvml().lib.nvmlVgpuInstanceGetUUID.as_ref())?;
        let mut buffer: [c_char; NVML_DEVICE_UUID_BUFFER_SIZE as usize] =
            [0; NVML_DEVICE_UUID_BUFFER_SIZE as usize];

        unsafe {
            nvml_try(sym(
                self.instance,
                buffer.as_mut_ptr(),
                NVML_DEVICE_UUID_BUFFER_SIZE,
            ))?;
            let raw_id = CStr::from_ptr(buffer.as_ptr());
            Ok(raw_id.to_str()?.to_string())
        }
    }

    /// Retrieve the NVIDIA driver version installed in the VM associated with a vGPU.
    ///
    /// The version is returned as an alphanumeric string in the caller-supplied buffer version.
    /// This may be called at any time for a vGPU instance.
    ///
    /// The guest VM driver version is returned as "Not Available" if no NVIDIA driver is installed
    /// in the VM, or the VM has not yet booted to the point where the NVIDIA driver is loaded and
    /// initialized.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Kepler or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceGetVmDriverVersion")]
    pub fn get_driver_version(&self) -> Result<String, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetVmDriverVersion
                .as_ref(),
        )?;
        let mut buffer: [c_char; NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE as usize] =
            [0; NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE as usize];

        unsafe {
            nvml_try(sym(
                self.instance,
                buffer.as_mut_ptr(),
                NVML_SYSTEM_NVML_VERSION_BUFFER_SIZE,
            ))?;
            let raw_id = CStr::from_ptr(buffer.as_ptr());
            Ok(raw_id.to_str()?.to_string())
        }
    }
    /// Set the encoder capacity of a vGPU instance, as a percentage of maximum encoder capacity with valid values in the range 0-100.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `NotFound`, if the vGPU does not match a valid active vGPU instance on the system
    ///
    /// # Platform Support
    ///
    /// For Maxwell or newer fully supported devices.
    #[doc(alias = "nvmlVgpuInstanceSetEncoderCapacity")]
    pub fn set_encoder_capacity(&self, capacity: u32) -> Result<(), NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceSetEncoderCapacity
                .as_ref(),
        )?;

        unsafe {
            nvml_try(sym(self.instance, capacity))?;
        }
        Ok(())
    }

    fn get_encoder_session_count(&self) -> Result<u32, NvmlError> {
        let sym = nvml_sym(
            self.device
                .nvml()
                .lib
                .nvmlVgpuInstanceGetEncoderSessions
                .as_ref(),
        )?;
        let mut count = 0;
        unsafe {
            nvml_try_count(sym(self.instance, &mut count, std::ptr::null_mut()))?;
        };
        Ok(count)
    }
}

impl<'dev> std::fmt::Debug for VgpuInstance<'dev> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VgpuInstance")
            .field("instance", &self.instance)
            .finish_non_exhaustive()
    }
}
