#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

impl HidElement {
    /// Attach another element to this element.
    pub fn attach(&self, other: &Self) {
        unsafe {
            ffi::IOHIDElementAttach(self.raw, other.raw);
        }
    }

    /// Detach a previously attached element from this element.
    pub fn detach(&self, other: &Self) {
        unsafe {
            ffi::IOHIDElementDetach(self.raw, other.raw);
        }
    }
}
