use crate::enums::gpm::GpmMetricId;
use crate::error::{nvml_sym, nvml_try, NvmlError};
use crate::ffi::bindings::*;
use crate::struct_wrappers::gpm::GpmMetricResult;
use crate::Nvml;

use std::mem;

/**
Handle to a GPM (GPU Performance Monitoring) sample.

GPM enables collecting fine-grained GPU performance metrics (SM occupancy,
tensor utilization, PCIe/NVLink bandwidth, etc.) on Hopper+ GPUs. Metrics
are computed by taking two time-separated samples and comparing them via
[`gpm_metrics_get`].

**Operations on a sample are not thread-safe.** It does not, therefore,
implement `Sync`.

You can obtain a `GpmSample` via [`crate::Device::gpm_sample()`] or
[`crate::Device::gpm_mig_sample()`].

Lifetimes are used to enforce that each `GpmSample` instance cannot be used
after the `Nvml` instance it was obtained from is dropped.
*/
#[derive(Debug)]
pub struct GpmSample<'nvml> {
    sample: nvmlGpmSample_t,
    nvml: &'nvml Nvml,
}

unsafe impl<'nvml> Send for GpmSample<'nvml> {}

impl<'nvml> GpmSample<'nvml> {
    /// Allocate a new GPM sample.
    ///
    /// # Errors
    ///
    /// * `Uninitialized`, if the library has not been successfully initialized
    /// * `Unknown`, on any unexpected error
    #[doc(alias = "nvmlGpmSampleAlloc")]
    pub(crate) fn alloc(nvml: &'nvml Nvml) -> Result<Self, NvmlError> {
        let sym = nvml_sym(nvml.lib.nvmlGpmSampleAlloc.as_ref())?;

        unsafe {
            let mut sample: nvmlGpmSample_t = mem::zeroed();
            nvml_try(sym(&mut sample))?;

            Ok(Self { sample, nvml })
        }
    }

    /**
    Use this to free the sample if you care about handling potential errors
    (*the `Drop` implementation ignores errors!*).

    # Errors

    * `Uninitialized`, if the library has not been successfully initialized
    * `Unknown`, on any unexpected error
    */
    #[doc(alias = "nvmlGpmSampleFree")]
    pub fn free(self) -> Result<(), NvmlError> {
        let sym = nvml_sym(self.nvml.lib.nvmlGpmSampleFree.as_ref())?;

        unsafe {
            nvml_try(sym(self.sample))?;
        }

        mem::forget(self);
        Ok(())
    }

    /// Get the raw sample handle.
    ///
    /// # Safety
    ///
    /// This is unsafe to prevent it from being used without care. In
    /// particular, you must avoid creating a new `GpmSample` from this handle
    /// and allowing both this `GpmSample` and the newly created one to drop
    /// (which would result in a double-free).
    pub unsafe fn handle(&self) -> nvmlGpmSample_t {
        self.sample
    }

    /// Get a reference to the `Nvml` instance this sample was created from.
    pub fn nvml(&self) -> &'nvml Nvml {
        self.nvml
    }
}

/// This `Drop` implementation ignores errors! Use the `.free()` method on
/// the `GpmSample` struct if you care about handling them.
impl<'nvml> Drop for GpmSample<'nvml> {
    #[doc(alias = "nvmlGpmSampleFree")]
    fn drop(&mut self) {
        unsafe {
            self.nvml.lib.nvmlGpmSampleFree(self.sample);
        }
    }
}

/**
Retrieve GPM metrics computed between two time-separated samples.

The two samples should have been previously populated via
[`crate::Device::gpm_sample()`] or [`crate::Device::gpm_mig_sample()`].

Returns a `Vec` with one entry per requested metric. Each entry is itself
a `Result`: the outer `Result` covers transport-level errors, while the
inner `Result` covers per-metric failures (e.g. a metric not supported on
the current GPU).

# Errors

* `Uninitialized`, if the library has not been successfully initialized
* `InvalidArg`, if any argument is invalid
* `NotSupported`, if GPM is not supported
* `Unknown`, on any unexpected error

# Panics

Panics if more than 98 metrics are requested (the maximum supported by NVML).

# Device Support

Supports Hopper and newer architectures.
*/
#[doc(alias = "nvmlGpmMetricsGet")]
pub fn gpm_metrics_get<'nvml>(
    nvml: &'nvml Nvml,
    sample1: &GpmSample<'nvml>,
    sample2: &GpmSample<'nvml>,
    metric_ids: &[GpmMetricId],
) -> Result<Vec<Result<GpmMetricResult, NvmlError>>, NvmlError> {
    assert!(
        metric_ids.len() <= nvmlGpmMetricId_t_NVML_GPM_METRIC_MAX as usize,
        "cannot request more than {} GPM metrics at once",
        nvmlGpmMetricId_t_NVML_GPM_METRIC_MAX
    );

    let sym = nvml_sym(nvml.lib.nvmlGpmMetricsGet.as_ref())?;

    unsafe {
        let mut request: nvmlGpmMetricsGet_t = mem::zeroed();
        request.version = NVML_GPM_METRICS_GET_VERSION;
        request.numMetrics = metric_ids.len() as u32;
        request.sample1 = sample1.sample;
        request.sample2 = sample2.sample;

        for (i, id) in metric_ids.iter().enumerate() {
            request.metrics[i].metricId = id.as_c();
        }

        nvml_try(sym(&mut request))?;

        let results = (0..metric_ids.len())
            .map(|i| GpmMetricResult::try_from_c(&request.metrics[i]))
            .collect();

        Ok(results)
    }
}
