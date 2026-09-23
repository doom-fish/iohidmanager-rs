#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

impl HidValue {
    /// Create a value that references the provided byte slice without asking
    /// the bridge to duplicate it first.
    ///
    /// # Safety
    ///
    /// `bytes` must remain valid and unchanged for at least as long as the
    /// returned `HidValue` may read from its backing storage.
    ///
    /// # Errors
    ///
    /// Returns `HidError::InvalidArgument` when the slice length does not fit
    /// `CFIndex`, or `HidError::OperationFailed` when
    /// `IOHIDValueCreateWithBytesNoCopy` returns NULL.
    pub unsafe fn from_bytes_no_copy(
        element: &HidElement,
        timestamp: u64,
        bytes: &[u8],
    ) -> Result<Self, HidError> {
        let length = ffi::CFIndex::try_from(bytes.len()).map_err(|_| {
            HidError::InvalidArgument("byte slice length does not fit CFIndex".to_owned())
        })?;
        let raw = unsafe {
            ffi::IOHIDValueCreateWithBytesNoCopy(
                ffi::kCFAllocatorDefault,
                element.raw,
                timestamp,
                bytes.as_ptr(),
                length,
            )
        };
        Self::from_raw_owned(raw, element.device.as_ref())
            .ok_or(HidError::OperationFailed("IOHIDValueCreateWithBytesNoCopy"))
    }
}
