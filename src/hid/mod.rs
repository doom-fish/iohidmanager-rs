//! High-level `HidManager` + `HidDeviceInfo` types.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;

use crate::error::HidError;
use crate::ffi;

/// Common HID Usage Page + Usage pairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HidUsage {
    /// Generic Desktop / Keyboard
    Keyboard,
    /// Generic Desktop / Mouse
    Mouse,
    /// Generic Desktop / Joystick
    Joystick,
    /// Generic Desktop / Game Pad
    GamePad,
    /// Custom (page, usage) pair.
    Custom(u32, u32),
}

impl HidUsage {
    /// Return the (`usage_page`, usage) pair Apple expects.
    #[must_use]
    pub const fn as_pair(self) -> (u32, u32) {
        match self {
            Self::Keyboard => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Keyboard),
            Self::Mouse => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Mouse),
            Self::Joystick => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Joystick),
            Self::GamePad => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_GamePad),
            Self::Custom(p, u) => (p, u),
        }
    }
}

/// Snapshot of a connected HID device's properties.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidDeviceInfo {
    pub vendor_id: Option<u32>,
    pub product_id: Option<u32>,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub transport: Option<String>,
    /// Usage page (e.g. 0x01 = Generic Desktop).
    pub usage_page: Option<u32>,
    /// Primary usage (e.g. 0x06 = Keyboard).
    pub usage: Option<u32>,
    pub location_id: Option<u32>,
}

/// Wraps `IOHIDManagerRef`. Drops the manager on scope exit.
pub struct HidManager {
    raw: ffi::IOHIDManagerRef,
}

unsafe impl Send for HidManager {}
unsafe impl Sync for HidManager {}

impl Drop for HidManager {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                ffi::IOHIDManagerClose(self.raw, ffi::kIOHIDOptionsTypeNone);
                ffi::CFRelease(self.raw.cast_const());
            }
            self.raw = ptr::null_mut();
        }
    }
}

impl HidManager {
    /// Create + open a new HID manager.
    ///
    /// # Errors
    ///
    /// Returns [`HidError::ManagerCreateFailed`] if `IOHIDManagerCreate`
    /// returns NULL, or [`HidError::ManagerOpenFailed`] with the raw
    /// `IOReturn` if open fails.
    pub fn new() -> Result<Self, HidError> {
        let raw = unsafe {
            ffi::IOHIDManagerCreate(ffi::kCFAllocatorDefault, ffi::kIOHIDOptionsTypeNone)
        };
        if raw.is_null() {
            return Err(HidError::ManagerCreateFailed);
        }
        let status = unsafe { ffi::IOHIDManagerOpen(raw, ffi::kIOHIDOptionsTypeNone) };
        if status != ffi::kIOReturnSuccess {
            unsafe { ffi::CFRelease(raw.cast_const()) };
            return Err(HidError::ManagerOpenFailed(status));
        }
        Ok(Self { raw })
    }

    /// Restrict subsequent enumeration to devices matching the given
    /// (`usage_page`, usage) pair. Pass `None` for unrestricted matching.
    ///
    /// # Errors
    ///
    /// Returns [`HidError::InvalidArgument`] for invalid input.
    pub fn set_device_matching(&self, usage: Option<HidUsage>) -> Result<(), HidError> {
        match usage {
            None => {
                unsafe { ffi::IOHIDManagerSetDeviceMatching(self.raw, ptr::null()) };
                Ok(())
            }
            Some(u) => {
                let (page, usage) = u.as_pair();
                let dict = unsafe {
                    ffi::CFDictionaryCreateMutable(
                        ffi::kCFAllocatorDefault,
                        2,
                        &raw const ffi::kCFTypeDictionaryKeyCallBacks,
                        &raw const ffi::kCFTypeDictionaryValueCallBacks,
                    )
                };
                let key_page = make_cfstring(ffi::kIOHIDDeviceUsagePageKey)?;
                let key_usage = make_cfstring(ffi::kIOHIDDeviceUsageKey)?;
                let n_page = make_cfnumber_u32(page);
                let n_usage = make_cfnumber_u32(usage);
                unsafe {
                    ffi::CFDictionarySetValue(dict, key_page.cast(), n_page.cast());
                    ffi::CFDictionarySetValue(dict, key_usage.cast(), n_usage.cast());
                    ffi::IOHIDManagerSetDeviceMatching(self.raw, dict.cast_const());
                    ffi::CFRelease(dict.cast_const());
                    ffi::CFRelease(key_page);
                    ffi::CFRelease(key_usage);
                    ffi::CFRelease(n_page);
                    ffi::CFRelease(n_usage);
                }
                Ok(())
            }
        }
    }

    /// Enumerate every device currently matched.
    #[must_use]
    pub fn devices(&self) -> Vec<HidDeviceInfo> {
        let set = unsafe { ffi::IOHIDManagerCopyDevices(self.raw) };
        if set.is_null() {
            return Vec::new();
        }
        let count = unsafe { ffi::CFSetGetCount(set) };
        let n = usize::try_from(count).unwrap_or(0);
        let mut buffer: Vec<*const c_void> = vec![ptr::null(); n];
        unsafe { ffi::CFSetGetValues(set, buffer.as_mut_ptr()) };
        let out = buffer
            .into_iter()
            .filter_map(|p| {
                if p.is_null() {
                    None
                } else {
                    Some(read_device_info(p.cast_mut()))
                }
            })
            .collect();
        unsafe { ffi::CFRelease(set) };
        out
    }
}

fn read_device_info(device: ffi::IOHIDDeviceRef) -> HidDeviceInfo {
    HidDeviceInfo {
        vendor_id: read_u32(device, ffi::kIOHIDVendorIDKey),
        product_id: read_u32(device, ffi::kIOHIDProductIDKey),
        product: read_string(device, ffi::kIOHIDProductKey),
        manufacturer: read_string(device, ffi::kIOHIDManufacturerKey),
        serial_number: read_string(device, ffi::kIOHIDSerialNumberKey),
        transport: read_string(device, ffi::kIOHIDTransportKey),
        usage_page: read_u32(device, ffi::kIOHIDPrimaryUsagePageKey),
        usage: read_u32(device, ffi::kIOHIDPrimaryUsageKey),
        location_id: read_u32(device, ffi::kIOHIDLocationIDKey),
    }
}

fn read_u32(device: ffi::IOHIDDeviceRef, key: &str) -> Option<u32> {
    let key_cf = make_cfstring(key).ok()?;
    let val = unsafe { ffi::IOHIDDeviceGetProperty(device, key_cf) };
    unsafe { ffi::CFRelease(key_cf) };
    if val.is_null() {
        return None;
    }
    if unsafe { ffi::CFGetTypeID(val) } != unsafe { ffi::CFNumberGetTypeID() } {
        return None;
    }
    let mut v: i32 = 0;
    let ok = unsafe {
        ffi::CFNumberGetValue(
            val,
            ffi::kCFNumberSInt32Type,
            ptr::from_mut(&mut v).cast::<c_void>(),
        )
    };
    if ok {
        u32::try_from(v).ok()
    } else {
        None
    }
}

fn read_string(device: ffi::IOHIDDeviceRef, key: &str) -> Option<String> {
    let key_cf = make_cfstring(key).ok()?;
    let val = unsafe { ffi::IOHIDDeviceGetProperty(device, key_cf) };
    unsafe { ffi::CFRelease(key_cf) };
    if val.is_null() {
        return None;
    }
    if unsafe { ffi::CFGetTypeID(val) } != unsafe { ffi::CFStringGetTypeID() } {
        return None;
    }
    let len = unsafe { ffi::CFStringGetLength(val) };
    let cap = (len * 4) + 1;
    let mut buf = vec![0u8; usize::try_from(cap).unwrap_or(0)];
    let ok = unsafe {
        ffi::CFStringGetCString(val, buf.as_mut_ptr().cast::<c_char>(), cap, ffi::kCFStringEncodingUTF8)
    };
    if !ok {
        return None;
    }
    if let Some(end) = buf.iter().position(|&b| b == 0) {
        buf.truncate(end);
    }
    String::from_utf8(buf).ok()
}

fn make_cfstring(s: &str) -> Result<ffi::CFStringRef, HidError> {
    let c = CString::new(s).map_err(|e| HidError::InvalidArgument(e.to_string()))?;
    let cf = unsafe {
        ffi::CFStringCreateWithCString(ffi::kCFAllocatorDefault, c.as_ptr(), ffi::kCFStringEncodingUTF8)
    };
    if cf.is_null() {
        return Err(HidError::InvalidArgument(format!(
            "CFStringCreateWithCString failed for {s:?}"
        )));
    }
    Ok(cf)
}

fn make_cfnumber_u32(v: u32) -> ffi::CFNumberRef {
    let v_i32 = i32::try_from(v).unwrap_or(0);
    unsafe {
        ffi::CFNumberCreate(
            ffi::kCFAllocatorDefault,
            ffi::kCFNumberSInt32Type,
            (&raw const v_i32).cast::<c_void>(),
        )
    }
}
