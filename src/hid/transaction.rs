use core::ptr;
use std::collections::HashMap;
use std::time::Duration;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps `IOHIDTransactionDirectionType`.
pub enum HidTransactionDirection {
    /// Mirrors `kIOHIDTransactionDirectionTypeInput`.
    Input,
    /// Mirrors `kIOHIDTransactionDirectionTypeOutput`.
    Output,
}

impl HidTransactionDirection {
    #[must_use]
    /// Mirrors `IOHIDTransactionDirectionType`.
    pub const fn as_raw(self) -> ffi::IOHIDTransactionDirectionType {
        match self {
            Self::Input => ffi::kIOHIDTransactionDirectionTypeInput,
            Self::Output => ffi::kIOHIDTransactionDirectionTypeOutput,
        }
    }

    #[must_use]
    /// Mirrors `IOHIDTransactionDirectionType`.
    pub const fn from_raw(raw: ffi::IOHIDTransactionDirectionType) -> Self {
        match raw {
            ffi::kIOHIDTransactionDirectionTypeOutput => Self::Output,
            _ => Self::Input,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps the option bits consumed by `IOHIDTransactionCreate`.
pub struct HidTransactionOptions(u32);

impl HidTransactionOptions {
    /// Mirrors `kIOHIDTransactionOptionsNone`.
    pub const NONE: Self = Self(ffi::kIOHIDTransactionOptionsNone);
    /// Mirrors `kIOHIDTransactionOptionsWeakDevice`.
    pub const WEAK_DEVICE: Self = Self(ffi::kIOHIDTransactionOptionsWeakDevice);

    #[must_use]
    /// Returns the raw option bits passed to `IOHIDTransactionCreate`.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Owns an `IOHIDTransactionRef` returned by `IOHIDTransactionCreate` and its device.
pub struct HidTransaction {
    raw: ffi::IOHIDTransactionRef,
    device: HidDevice,
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
    /// Wraps `IOHIDTransactionGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDTransactionGetTypeID() }
    }

    /// Wraps `IOHIDTransactionCreate` with `kIOHIDTransactionOptionsNone`.
    pub fn new(device: &HidDevice, direction: HidTransactionDirection) -> Result<Self, HidError> {
        Self::with_options(device, direction, HidTransactionOptions::NONE)
    }

    /// Wraps `IOHIDTransactionCreate`.
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
            Ok(Self {
                raw,
                device: device.clone(),
            })
        }
    }

    #[must_use]
    /// Returns the device the transaction was created for (`IOHIDTransactionGetDevice`).
    pub fn device(&self) -> Option<HidDevice> {
        Some(self.device.clone())
    }

    #[must_use]
    /// Wraps `IOHIDTransactionGetDirection`.
    pub fn direction(&self) -> HidTransactionDirection {
        HidTransactionDirection::from_raw(unsafe { ffi::IOHIDTransactionGetDirection(self.raw) })
    }

    /// Wraps `IOHIDTransactionSetDirection`.
    pub fn set_direction(&self, direction: HidTransactionDirection) {
        unsafe { ffi::IOHIDTransactionSetDirection(self.raw, direction.as_raw()) };
    }

    /// Wraps `IOHIDTransactionAddElement`.
    pub fn add_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDTransactionAddElement(self.raw, element.raw) };
    }

    /// Wraps `IOHIDTransactionRemoveElement`.
    pub fn remove_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDTransactionRemoveElement(self.raw, element.raw) };
    }

    #[must_use]
    /// Wraps `IOHIDTransactionContainsElement`.
    pub fn contains_element(&self, element: &HidElement) -> bool {
        unsafe { ffi::IOHIDTransactionContainsElement(self.raw, element.raw) }
    }

    /// Wraps `IOHIDTransactionScheduleWithRunLoop` for the current run loop.
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

    /// Wraps `IOHIDTransactionUnscheduleFromRunLoop` for the current run loop.
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

    /// Wraps `IOHIDTransactionSetValue`.
    pub fn set_value(&self, element: &HidElement, value: &HidValue, options: ffi::IOOptionBits) {
        unsafe { ffi::IOHIDTransactionSetValue(self.raw, element.raw, value.raw, options) };
    }

    #[must_use]
    /// Wraps `IOHIDTransactionGetValue`.
    pub fn get_value(&self, element: &HidElement, options: ffi::IOOptionBits) -> Option<HidValue> {
        clone_value_ref(
            unsafe { ffi::IOHIDTransactionGetValue(self.raw, element.raw, options) },
            Some(&self.device),
        )
    }

    /// Wraps `IOHIDTransactionCommit`.
    pub fn commit(&self) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDTransactionCommit(self.raw) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDTransactionCommit", status))
        }
    }

    /// Wraps `IOHIDTransactionCommitWithCallback`.
    pub fn commit_with_timeout(&self, timeout: Duration) -> Result<(), HidError> {
        if self.direction() != HidTransactionDirection::Output {
            return Err(HidError::InvalidArgument(
                "commit_with_timeout commits output transactions; use commit for input".to_owned(),
            ));
        }
        let timeout_ms = completion::request_timeout_ms(timeout)?;
        let service = self.device.registered_service()?;
        completion::complete_on_private_device(
            COMMIT_WITH_CALLBACK,
            service,
            timeout,
            |device, context| {
                let mirror = self.mirror_onto(device)?;
                let status = unsafe {
                    ffi::IOHIDTransactionCommitWithCallback(
                        mirror.get().cast_mut(),
                        timeout_ms,
                        Some(completion::commit_completed),
                        context,
                    )
                };
                if status == ffi::kIOReturnSuccess {
                    Ok(mirror)
                } else {
                    Err(HidError::IoReturn(COMMIT_WITH_CALLBACK, status))
                }
            },
        )
    }

    pub(super) fn mirror_onto(&self, device: ffi::IOHIDDeviceRef) -> Result<Retained, HidError> {
        let mirror = Retained::from_create(
            unsafe {
                ffi::IOHIDTransactionCreate(
                    ffi::kCFAllocatorDefault,
                    device,
                    ffi::kIOHIDTransactionDirectionTypeOutput,
                    ffi::kIOHIDTransactionOptionsNone,
                )
            }
            .cast_const(),
        )
        .ok_or(HidError::OperationFailed("IOHIDTransactionCreate"))?;
        let targets: HashMap<IOHIDElementCookie, HidElement> = elements_from_array(
            unsafe {
                ffi::IOHIDDeviceCopyMatchingElements(
                    device,
                    ptr::null(),
                    ffi::kIOHIDOptionsTypeNone,
                )
            },
            true,
            None,
        )
        .into_iter()
        .map(|element| (element.cookie(), element))
        .collect();
        for element in self.device.elements() {
            if !self.contains_element(&element) {
                continue;
            }
            let target = targets
                .get(&element.cookie())
                .ok_or(HidError::OperationFailed("IOHIDDeviceCopyMatchingElements"))?;
            unsafe { ffi::IOHIDTransactionAddElement(mirror.get().cast_mut(), target.raw) };
            for options in [
                ffi::kIOHIDTransactionOptionDefaultOutputValue,
                ffi::kIOHIDTransactionOptionsNone,
            ] {
                let value =
                    unsafe { ffi::IOHIDTransactionGetValue(self.raw, element.raw, options) };
                if value.is_null() {
                    continue;
                }
                let copy = Retained::from_create(copy_value_to(value, target.raw))
                    .ok_or(HidError::OperationFailed("IOHIDValueCreateWithBytes"))?;
                unsafe {
                    ffi::IOHIDTransactionSetValue(
                        mirror.get().cast_mut(),
                        target.raw,
                        copy.get(),
                        options,
                    );
                }
            }
        }
        Ok(mirror)
    }

    /// Wraps `IOHIDTransactionClear`.
    pub fn clear(&self) {
        unsafe { ffi::IOHIDTransactionClear(self.raw) };
    }

    #[must_use]
    /// Mirrors `IOHIDTransactionRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDTransactionRef {
        self.raw
    }
}

const COMMIT_WITH_CALLBACK: &str = "IOHIDTransactionCommitWithCallback";
