#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

#[allow(clippy::missing_errors_doc)]
impl HidElement {
    /// Attach another element of the same device to this element.
    pub fn attach(&self, other: &Self) -> Result<(), HidError> {
        let same_device = match (&self.device, &other.device) {
            (Some(device), Some(other_device)) => device.raw == other_device.raw,
            (None, None) => true,
            _ => false,
        };
        if !same_device {
            return Err(HidError::InvalidArgument(
                "attached elements must belong to the same device".to_owned(),
            ));
        }
        unsafe {
            ffi::IOHIDElementAttach(self.raw, other.raw);
        }
        Ok(())
    }

    /// Detach a previously attached element from this element.
    pub fn detach(&self, other: &Self) {
        unsafe {
            ffi::IOHIDElementDetach(self.raw, other.raw);
        }
    }
}
