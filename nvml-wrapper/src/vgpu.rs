use std::ffi::{c_uint, CStr};

use ffi::bindings::{
    nvmlVgpuCapability_t, nvmlVgpuTypeId_t, NVML_DEVICE_NAME_BUFFER_SIZE,
    NVML_GRID_LICENSE_BUFFER_SIZE,
};
use static_assertions::assert_impl_all;

use crate::{
    error::{nvml_sym, nvml_try, NvmlError},
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
    pub fn device(&self) -> &'dev Device {
        self.device
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

    /// # Errors

    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `InvalidArg`, if this `Device` is invalid
    /// * `Unknown`, on any unexpected error

    /// # Device Support

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
}
