use std::convert::TryFrom;
use std::ffi::CStr;

use crate::enums::gpm::GpmMetricId;
use crate::error::{nvml_try, NvmlError};
use crate::ffi::bindings::*;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

/// Descriptive information about a GPM metric.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GpmMetricInfo {
    pub short_name: String,
    pub long_name: String,
    pub unit: String,
}

impl GpmMetricInfo {
    /// Construct from the C inner type with null-pointer safety.
    ///
    /// # Safety
    ///
    /// The pointers in `info` must either be null or point to valid
    /// null-terminated C strings.
    pub(crate) unsafe fn from_c(info: &nvmlGpmMetric_t__bindgen_ty_1) -> Result<Self, NvmlError> {
        let short_name = if info.shortName.is_null() {
            String::new()
        } else {
            CStr::from_ptr(info.shortName).to_str()?.into()
        };

        let long_name = if info.longName.is_null() {
            String::new()
        } else {
            CStr::from_ptr(info.longName).to_str()?.into()
        };

        let unit = if info.unit.is_null() {
            String::new()
        } else {
            CStr::from_ptr(info.unit).to_str()?.into()
        };

        Ok(Self {
            short_name,
            long_name,
            unit,
        })
    }
}

/// The result of a single GPM metric query.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GpmMetricResult {
    pub metric_id: GpmMetricId,
    pub value: f64,
    pub metric_info: GpmMetricInfo,
}

impl GpmMetricResult {
    /// Construct from the C type, checking the per-metric return code.
    ///
    /// # Safety
    ///
    /// The string pointers within `metric.metricInfo` must be valid.
    pub(crate) unsafe fn try_from_c(metric: &nvmlGpmMetric_t) -> Result<Self, NvmlError> {
        nvml_try(metric.nvmlReturn)?;

        let metric_id = GpmMetricId::try_from(metric.metricId)?;
        let metric_info = GpmMetricInfo::from_c(&metric.metricInfo)?;

        Ok(Self {
            metric_id,
            value: metric.value,
            metric_info,
        })
    }
}
