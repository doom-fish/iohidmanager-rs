use core::ptr;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::{bridge, ffi_impl as ffi};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HidTransactionDirection {
    Input,
    Output,
}

impl HidTransactionDirection {
    #[must_use]
    pub const fn as_raw(self) -> ffi::IOHIDTransactionDirectionType {
        match self {
            Self::Input => ffi::kIOHIDTransactionDirectionTypeInput,
            Self::Output => ffi::kIOHIDTransactionDirectionTypeOutput,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: ffi::IOHIDTransactionDirectionType) -> Self {
        match raw {
            ffi::kIOHIDTransactionDirectionTypeOutput => Self::Output,
            _ => Self::Input,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HidTransactionOptions(u32);

impl HidTransactionOptions {
    pub const NONE: Self = Self(ffi::kIOHIDTransactionOptionsNone);
    pub const WEAK_DEVICE: Self = Self(ffi::kIOHIDTransactionOptionsWeakDevice);

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

pub struct HidTransaction {
    raw: ffi::IOHIDTransactionRef,
}

unsafe impl Send for HidTransaction {}
unsafe impl Sync for HidTransaction {}

impl Clone for HidTransaction {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw.cast_const()) };
        Self { raw: self.raw }
    }
}

impl Drop for HidTransaction {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidTransaction {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { bridge::iohidmanager_swift_transaction_type_id() }
    }

    pub fn new(device: &HidDevice, direction: HidTransactionDirection) -> Result<Self, HidError> {
        Self::with_options(device, direction, HidTransactionOptions::NONE)
    }

    pub fn with_options(
        device: &HidDevice,
        direction: HidTransactionDirection,
        options: HidTransactionOptions,
    ) -> Result<Self, HidError> {
        let raw = unsafe {
            ffi::IOHIDTransactionCreate(
                ffi::kCFAllocatorDefault,
                device.raw,
                direction.as_raw(),
                options.bits(),
            )
        };
        if raw.is_null() {
            Err(HidError::OperationFailed("IOHIDTransactionCreate"))
        } else {
            Ok(Self { raw })
        }
    }

    #[must_use]
    pub fn device(&self) -> Option<HidDevice> {
        let device = unsafe { ffi::IOHIDTransactionGetDevice(self.raw) };
        if device.is_null() {
            None
        } else {
            unsafe { ffi::CFRetain(device) };
            Some(HidDevice { raw: device })
        }
    }

    #[must_use]
    pub fn direction(&self) -> HidTransactionDirection {
        HidTransactionDirection::from_raw(unsafe { ffi::IOHIDTransactionGetDirection(self.raw) })
    }

    pub fn set_direction(&self, direction: HidTransactionDirection) {
        unsafe { ffi::IOHIDTransactionSetDirection(self.raw, direction.as_raw()) };
    }

    pub fn add_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDTransactionAddElement(self.raw, element.raw) };
    }

    pub fn remove_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDTransactionRemoveElement(self.raw, element.raw) };
    }

    #[must_use]
    pub fn contains_element(&self, element: &HidElement) -> bool {
        unsafe { ffi::IOHIDTransactionContainsElement(self.raw, element.raw) }
    }

    pub fn schedule_current_run_loop(&self) {
        let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
        unsafe {
            ffi::IOHIDTransactionScheduleWithRunLoop(
                self.raw,
                run_loop,
                ffi::kCFRunLoopDefaultMode,
            );
        }
    }

    pub fn unschedule_current_run_loop(&self) {
        let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
        unsafe {
            ffi::IOHIDTransactionUnscheduleFromRunLoop(
                self.raw,
                run_loop,
                ffi::kCFRunLoopDefaultMode,
            );
        }
    }

    pub fn set_value(&self, element: &HidElement, value: &HidValue, options: ffi::IOOptionBits) {
        unsafe { ffi::IOHIDTransactionSetValue(self.raw, element.raw, value.raw, options) };
    }

    #[must_use]
    pub fn get_value(&self, element: &HidElement, options: ffi::IOOptionBits) -> Option<HidValue> {
        clone_value_ref(unsafe { ffi::IOHIDTransactionGetValue(self.raw, element.raw, options) })
    }

    pub fn commit(&self) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDTransactionCommit(self.raw) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDTransactionCommit", status))
        }
    }

    pub fn commit_with_timeout(&self, timeout_ms: f64) -> Result<(), HidError> {
        let status = unsafe {
            ffi::IOHIDTransactionCommitWithCallback(self.raw, timeout_ms, None, ptr::null_mut())
        };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn(
                "IOHIDTransactionCommitWithCallback",
                status,
            ))
        }
    }

    pub fn clear(&self) {
        unsafe { ffi::IOHIDTransactionClear(self.raw) };
    }

    #[must_use]
    pub const fn as_ptr(&self) -> ffi::IOHIDTransactionRef {
        self.raw
    }
}
