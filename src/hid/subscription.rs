use core::ffi::c_void;
use std::sync::atomic::{AtomicU64, Ordering};

use doom_fish_utils::callback_context::CallbackContext;

use super::event_loop;
use crate::ffi_impl as ffi;

pub type Teardown = Box<dyn FnOnce() + Send + Sync>;

pub struct Registration<T: Send + Sync + 'static> {
    context: CallbackContext<T>,
    teardown: Option<Teardown>,
}

impl<T: Send + Sync + 'static> Registration<T> {
    pub fn new(context: CallbackContext<T>, teardown: Teardown) -> Self {
        Self {
            context,
            teardown: Some(teardown),
        }
    }
}

impl<T: Send + Sync + 'static> Drop for Registration<T> {
    fn drop(&mut self) {
        self.context.deactivate();
        if let Some(teardown) = self.teardown.take() {
            event_loop::defer(teardown);
        }
    }
}

pub struct ForeignRef {
    ptr: *mut c_void,
    release: unsafe extern "C" fn(*mut c_void),
}

unsafe impl Send for ForeignRef {}
unsafe impl Sync for ForeignRef {}

impl ForeignRef {
    pub fn new<T: Send + Sync + 'static>(context: &CallbackContext<T>) -> Self {
        Self {
            ptr: context.retained_ptr(),
            release: CallbackContext::<T>::RELEASE,
        }
    }

    pub const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }
}

impl Drop for ForeignRef {
    fn drop(&mut self) {
        unsafe { (self.release)(self.ptr) };
    }
}

pub struct Retained(ffi::CFTypeRef);

unsafe impl Send for Retained {}
unsafe impl Sync for Retained {}

impl Retained {
    pub fn from_create(raw: ffi::CFTypeRef) -> Option<Self> {
        (!raw.is_null()).then_some(Self(raw))
    }

    pub fn retain(raw: ffi::CFTypeRef) -> Self {
        unsafe { ffi::CFRetain(raw) };
        Self(raw)
    }

    pub const fn get(&self) -> ffi::CFTypeRef {
        self.0
    }
}

impl Drop for Retained {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.0) };
    }
}

pub struct ReportBuffer(*mut [u8]);

unsafe impl Send for ReportBuffer {}
unsafe impl Sync for ReportBuffer {}

impl ReportBuffer {
    pub fn new(len: usize) -> Self {
        Self(Box::into_raw(vec![0_u8; len].into_boxed_slice()))
    }

    pub const fn as_mut_ptr(&self) -> *mut u8 {
        self.0.cast()
    }

    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl Drop for ReportBuffer {
    fn drop(&mut self) {
        drop(unsafe { Box::from_raw(self.0) });
    }
}

pub fn next_id() -> u64 {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    NEXT.fetch_add(1, Ordering::Relaxed)
}
