use crate::ffi::bindings::*;
use bitflags::bitflags;
#[cfg(feature = "serde")]
use serde_derive::{Deserialize, Serialize};

bitflags! {
    /**
    Event types that you can request to be notified about.

    Types can be combined with the Bitwise Or operator `|` when passed to
    `Device.register_events()`.
    */
    // Checked against local
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct EventTypes: u64 {
        /// A corrected texture memory error is not an ECC error, so it does not
        /// generate a single bit event.
        const SINGLE_BIT_ECC_ERROR  = nvmlEventTypeSingleBitEccError as u64;
        /// An uncorrected texture memory error is not an ECC error, so it does not
        /// generate a double bit event.
        const DOUBLE_BIT_ECC_ERROR  = nvmlEventTypeDoubleBitEccError as u64;
        /**
        Power state change event.

        On the Fermi architecture, a PState change is an indicator that the GPU
        is throttling down due to no work being executed on the GPU, power
        capping, or thermal capping. In a typical situation, Fermi-based
        GPUs should stay in performance state zero for the duration of the
        execution of a compute process.
        */
        const PSTATE_CHANGE         = nvmlEventTypePState as u64;
        const CRITICAL_XID_ERROR    = nvmlEventTypeXidCriticalError as u64;
        /// Only supports the Kepler architecture.
        const CLOCK_CHANGE          = nvmlEventTypeClock as u64;
        /// Power source change event (battery vs. AC power).
        const POWER_SOURCE_CHANGE   = nvmlEventTypePowerSourceChange as u64;
        /// MIG configuration changes.
        /// Placeholder for unknown event type bits introduced in newer NVML versions.
        const UNKNOWN_0x80   = 0x80u64;
        const UNKNOWN_0x100  = 0x100u64;
        const UNKNOWN_0x200  = 0x200u64;
        const UNKNOWN_0x400  = 0x400u64;
        const UNKNOWN_0x800  = 0x800u64;
        const UNKNOWN_0x1000 = 0x1000u64;
        const UNKNOWN_0x2000 = 0x2000u64;
        const UNKNOWN_0x4000 = 0x4000u64;
        const UNKNOWN_0x8000 = 0x8000u64;
    }
}
