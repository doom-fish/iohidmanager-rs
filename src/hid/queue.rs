use core::ffi::c_void;
use core::ptr;
use std::time::Duration;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;
use doom_fish_utils::panic_safe::catch_user_panic;

#[allow(clippy::type_complexity)]
struct QueueValueContext {
    callback: *mut Box<dyn Fn() + Send + Sync + 'static>,
}

unsafe extern "C" fn queue_value_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    if context.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let callback = unsafe { &*(*context.cast::<QueueValueContext>()).callback };
    catch_user_panic("queue_value_trampoline", || {
        callback();
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps the option bits consumed by `IOHIDQueueCreate`.
pub struct HidQueueOptions(u32);

impl HidQueueOptions {
    /// Mirrors `kIOHIDQueueOptionsTypeNone`.
    pub const NONE: Self = Self(ffi::kIOHIDQueueOptionsTypeNone);
    /// Mirrors `kIOHIDQueueOptionsTypeEnqueueAll`.
    pub const ENQUEUE_ALL: Self = Self(ffi::kIOHIDQueueOptionsTypeEnqueueAll);

    #[must_use]
    /// Returns the raw option bits passed to `IOHIDQueueCreate`.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Owns an `IOHIDQueueRef` returned by `IOHIDQueueCreate` and the device it reads from.
pub struct HidQueue {
    raw: ffi::IOHIDQueueRef,
    device: HidDevice,
}

unsafe impl Send for HidQueue {}

impl Drop for HidQueue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

/// Owns a registration from `IOHIDQueueRegisterValueAvailableCallback`.
pub struct QueueValueAvailableSubscription {
    queue: ffi::IOHIDQueueRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut QueueValueContext,
}

unsafe impl Send for QueueValueAvailableSubscription {}

impl Drop for QueueValueAvailableSubscription {
    fn drop(&mut self) {
        if self.queue.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            ffi::IOHIDQueueRegisterValueAvailableCallback(self.queue, None, ptr::null_mut());
            ffi::IOHIDQueueUnscheduleFromRunLoop(
                self.queue,
                self.run_loop,
                ffi::kCFRunLoopDefaultMode,
            );
            ffi::CFRelease(self.queue.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.queue = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidQueue {
    #[must_use]
    /// Wraps `IOHIDQueueGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDQueueGetTypeID() }
    }

    /// Wraps `IOHIDQueueCreate` with `kIOHIDQueueOptionsTypeNone`.
    pub fn new(device: &HidDevice, depth: usize) -> Result<Self, HidError> {
        Self::with_options(device, depth, HidQueueOptions::NONE)
    }

    /// Wraps `IOHIDQueueCreate`.
    pub fn with_options(
        device: &HidDevice,
        depth: usize,
        options: HidQueueOptions,
    ) -> Result<Self, HidError> {
        let depth = ffi::CFIndex::try_from(depth)
            .map_err(|_| HidError::InvalidArgument("depth does not fit CFIndex".to_owned()))?;
        let raw = unsafe {
            ffi::IOHIDQueueCreate(ffi::kCFAllocatorDefault, device.raw, depth, options.bits())
        };
        if raw.is_null() {
            Err(HidError::OperationFailed("IOHIDQueueCreate"))
        } else {
            Ok(Self {
                raw,
                device: device.clone(),
            })
        }
    }

    #[must_use]
    /// Returns the device the queue was created for (`IOHIDQueueGetDevice`).
    pub fn device(&self) -> Option<HidDevice> {
        Some(self.device.clone())
    }

    #[must_use]
    /// Wraps `IOHIDQueueGetDepth`.
    pub fn depth(&self) -> usize {
        usize::try_from(unsafe { ffi::IOHIDQueueGetDepth(self.raw) }).unwrap_or(0)
    }

    /// Wraps `IOHIDQueueSetDepth`.
    pub fn set_depth(&self, depth: usize) -> Result<(), HidError> {
        let depth = ffi::CFIndex::try_from(depth)
            .map_err(|_| HidError::InvalidArgument("depth does not fit CFIndex".to_owned()))?;
        unsafe { ffi::IOHIDQueueSetDepth(self.raw, depth) };
        Ok(())
    }

    /// Wraps `IOHIDQueueAddElement`.
    pub fn add_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueAddElement(self.raw, element.raw) };
    }

    /// Wraps `IOHIDQueueRemoveElement`.
    pub fn remove_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueRemoveElement(self.raw, element.raw) };
    }

    #[must_use]
    /// Wraps `IOHIDQueueContainsElement`.
    pub fn contains_element(&self, element: &HidElement) -> bool {
        unsafe { ffi::IOHIDQueueContainsElement(self.raw, element.raw) }
    }

    /// Wraps `IOHIDQueueStart`.
    pub fn start(&self) {
        unsafe { ffi::IOHIDQueueStart(self.raw) };
    }

    /// Wraps `IOHIDQueueStop`.
    pub fn stop(&self) {
        unsafe { ffi::IOHIDQueueStop(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDQueueActivate`.
    pub unsafe fn activate(&self) {
        unsafe { ffi::IOHIDQueueActivate(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDQueueCancel`.
    pub unsafe fn cancel(&self) {
        unsafe { ffi::IOHIDQueueCancel(self.raw) };
    }

    #[must_use]
    /// Wraps `IOHIDQueueScheduleWithRunLoop` for the current run loop.
    pub fn schedule_current_run_loop(&self) -> ffi::CFRunLoopRef {
        let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
        unsafe {
            ffi::IOHIDQueueScheduleWithRunLoop(self.raw, run_loop, ffi::kCFRunLoopDefaultMode);
        }
        run_loop
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    /// Wraps `IOHIDQueueUnscheduleFromRunLoop`.
    pub fn unschedule_from_run_loop(&self, run_loop: ffi::CFRunLoopRef) {
        unsafe {
            ffi::IOHIDQueueUnscheduleFromRunLoop(self.raw, run_loop, ffi::kCFRunLoopDefaultMode);
        }
    }

    #[must_use]
    /// Wraps `IOHIDQueueCopyNextValue`.
    pub fn copy_next_value(&self) -> Option<HidValue> {
        HidValue::from_raw_owned(
            unsafe { ffi::IOHIDQueueCopyNextValue(self.raw) },
            Some(&self.device),
        )
    }

    #[must_use]
    /// Wraps `IOHIDQueueCopyNextValueWithTimeout`.
    ///
    /// Timeouts longer than `u32::MAX` milliseconds are clamped to that limit.
    pub fn copy_next_value_with_timeout(&self, timeout: Duration) -> Option<HidValue> {
        HidValue::from_raw_owned(
            unsafe {
                ffi::IOHIDQueueCopyNextValueWithTimeout(self.raw, queue_timeout_seconds(timeout))
            },
            Some(&self.device),
        )
    }

    /// Wraps `IOHIDQueueRegisterValueAvailableCallback`.
    pub fn on_value_available<F>(
        &self,
        callback: F,
    ) -> Result<QueueValueAvailableSubscription, HidError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let run_loop = self.schedule_current_run_loop();
        let callback: Box<dyn Fn() + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(QueueValueContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDQueueRegisterValueAvailableCallback(
                self.raw,
                Some(queue_value_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(QueueValueAvailableSubscription {
            queue: self.raw,
            run_loop,
            context: context_ptr,
        })
    }

    #[must_use]
    /// Mirrors `IOHIDQueueRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDQueueRef {
        self.raw
    }
}

pub(crate) fn queue_timeout_seconds(timeout: Duration) -> ffi::CFTimeInterval {
    let millis = timeout.as_millis().min(u128::from(u32::MAX));
    let millis = u32::try_from(millis).unwrap_or(u32::MAX);
    f64::from(millis) / 1000.0
}
