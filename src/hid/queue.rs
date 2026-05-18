use core::ffi::c_void;
use core::ptr;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::{bridge, ffi_impl as ffi};

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
    callback();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HidQueueOptions(u32);

impl HidQueueOptions {
    pub const NONE: Self = Self(ffi::kIOHIDQueueOptionsTypeNone);
    pub const ENQUEUE_ALL: Self = Self(ffi::kIOHIDQueueOptionsTypeEnqueueAll);

    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }
}

pub struct HidQueue {
    raw: ffi::IOHIDQueueRef,
}

unsafe impl Send for HidQueue {}
unsafe impl Sync for HidQueue {}

impl Clone for HidQueue {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw.cast_const()) };
        Self { raw: self.raw }
    }
}

impl Drop for HidQueue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

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
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { bridge::iohidmanager_swift_queue_type_id() }
    }

    pub fn new(device: &HidDevice, depth: usize) -> Result<Self, HidError> {
        Self::with_options(device, depth, HidQueueOptions::NONE)
    }

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
            Ok(Self { raw })
        }
    }

    #[must_use]
    pub fn device(&self) -> Option<HidDevice> {
        let device = unsafe { ffi::IOHIDQueueGetDevice(self.raw) };
        if device.is_null() {
            None
        } else {
            unsafe { ffi::CFRetain(device) };
            Some(HidDevice { raw: device })
        }
    }

    #[must_use]
    pub fn depth(&self) -> usize {
        usize::try_from(unsafe { ffi::IOHIDQueueGetDepth(self.raw) }).unwrap_or(0)
    }

    pub fn set_depth(&self, depth: usize) -> Result<(), HidError> {
        let depth = ffi::CFIndex::try_from(depth)
            .map_err(|_| HidError::InvalidArgument("depth does not fit CFIndex".to_owned()))?;
        unsafe { ffi::IOHIDQueueSetDepth(self.raw, depth) };
        Ok(())
    }

    pub fn add_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueAddElement(self.raw, element.raw) };
    }

    pub fn remove_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueRemoveElement(self.raw, element.raw) };
    }

    #[must_use]
    pub fn contains_element(&self, element: &HidElement) -> bool {
        unsafe { ffi::IOHIDQueueContainsElement(self.raw, element.raw) }
    }

    pub fn start(&self) {
        unsafe { ffi::IOHIDQueueStart(self.raw) };
    }

    pub fn stop(&self) {
        unsafe { ffi::IOHIDQueueStop(self.raw) };
    }

    pub fn activate(&self) {
        unsafe { ffi::IOHIDQueueActivate(self.raw) };
    }

    pub fn cancel(&self) {
        unsafe { ffi::IOHIDQueueCancel(self.raw) };
    }

    #[must_use]
    pub fn schedule_current_run_loop(&self) -> ffi::CFRunLoopRef {
        let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
        unsafe {
            ffi::IOHIDQueueScheduleWithRunLoop(self.raw, run_loop, ffi::kCFRunLoopDefaultMode);
        }
        run_loop
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn unschedule_from_run_loop(&self, run_loop: ffi::CFRunLoopRef) {
        unsafe {
            ffi::IOHIDQueueUnscheduleFromRunLoop(self.raw, run_loop, ffi::kCFRunLoopDefaultMode);
        }
    }

    #[must_use]
    pub fn copy_next_value(&self) -> Option<HidValue> {
        clone_value_ref(unsafe { ffi::IOHIDQueueCopyNextValue(self.raw) })
    }

    #[must_use]
    pub fn copy_next_value_with_timeout(&self, timeout_ms: f64) -> Option<HidValue> {
        clone_value_ref(unsafe { ffi::IOHIDQueueCopyNextValueWithTimeout(self.raw, timeout_ms) })
    }

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
    pub const fn as_ptr(&self) -> ffi::IOHIDQueueRef {
        self.raw
    }
}
