//! High-level `IOKit` HID wrappers.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::fmt;

use crate::error::HidError;
use crate::ffi_impl as ffi;

pub mod device;
pub mod element;
pub mod event_system;
pub mod keys;
pub mod manager;
pub mod queue;
pub mod service_plugin;
pub mod transaction;
pub mod usage;
pub mod value;

pub use device::DeviceRemovalSubscription;
pub use manager::{
    ManagerDeviceSubscription, ManagerReportSubscription, ManagerValueSubscription,
};
pub use queue::{HidQueue, HidQueueOptions, QueueValueAvailableSubscription};
pub use transaction::{HidTransaction, HidTransactionDirection, HidTransactionOptions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HidUsage {
    Keyboard,
    Mouse,
    Joystick,
    GamePad,
    Custom(u32, u32),
}

impl HidUsage {
    #[must_use]
    pub const fn as_pair(self) -> (u32, u32) {
        match self {
            Self::Keyboard => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Keyboard),
            Self::Mouse => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Mouse),
            Self::Joystick => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_Joystick),
            Self::GamePad => (ffi::kHIDPage_GenericDesktop, ffi::kHIDUsage_GD_GamePad),
            Self::Custom(page, usage) => (page, usage),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum MatchValue {
    U32(u32),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeviceMatch {
    entries: Vec<(String, MatchValue)>,
}

impl DeviceMatch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn usage(usage: HidUsage) -> Self {
        Self::new().with_usage(usage)
    }

    #[must_use]
    pub fn with_usage(mut self, usage: HidUsage) -> Self {
        let (page, usage) = usage.as_pair();
        self.insert(ffi::kIOHIDDeviceUsagePageKey, MatchValue::U32(page));
        self.insert(ffi::kIOHIDDeviceUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    pub fn with_vendor_id(mut self, vendor_id: u32) -> Self {
        self.insert(ffi::kIOHIDVendorIDKey, MatchValue::U32(vendor_id));
        self
    }

    #[must_use]
    pub fn with_product_id(mut self, product_id: u32) -> Self {
        self.insert(ffi::kIOHIDProductIDKey, MatchValue::U32(product_id));
        self
    }

    #[must_use]
    pub fn with_location_id(mut self, location_id: u32) -> Self {
        self.insert(ffi::kIOHIDLocationIDKey, MatchValue::U32(location_id));
        self
    }

    #[must_use]
    pub fn with_transport(mut self, transport: impl Into<String>) -> Self {
        self.insert(
            ffi::kIOHIDTransportKey,
            MatchValue::String(transport.into()),
        );
        self
    }

    #[must_use]
    pub fn with_u32(mut self, key: &str, value: u32) -> Self {
        self.insert(key, MatchValue::U32(value));
        self
    }

    #[must_use]
    pub fn with_string(mut self, key: &str, value: impl Into<String>) -> Self {
        self.insert(key, MatchValue::String(value.into()));
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn insert(&mut self, key: &str, value: MatchValue) {
        if let Some((_, existing)) = self
            .entries
            .iter_mut()
            .find(|(existing_key, _)| existing_key == key)
        {
            *existing = value;
            return;
        }
        self.entries.push((key.to_owned(), value));
    }

    fn to_cf_dictionary(&self) -> Result<ffi::CFDictionaryRef, HidError> {
        build_cf_dictionary(&self.entries)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ElementMatch {
    entries: Vec<(String, MatchValue)>,
}

impl ElementMatch {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn usage(usage: HidUsage) -> Self {
        Self::new().with_usage(usage)
    }

    #[must_use]
    pub fn with_usage(mut self, usage: HidUsage) -> Self {
        let (page, usage) = usage.as_pair();
        self.insert(ffi::kIOHIDElementUsagePageKey, MatchValue::U32(page));
        self.insert(ffi::kIOHIDElementUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    pub fn with_usage_page(mut self, usage_page: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsagePageKey, MatchValue::U32(usage_page));
        self
    }

    #[must_use]
    pub fn with_usage_id(mut self, usage: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    pub fn with_usage_min(mut self, usage_min: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageMinKey, MatchValue::U32(usage_min));
        self
    }

    #[must_use]
    pub fn with_usage_max(mut self, usage_max: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageMaxKey, MatchValue::U32(usage_max));
        self
    }

    #[must_use]
    pub fn with_cookie_min(mut self, cookie_min: u32) -> Self {
        self.insert(ffi::kIOHIDElementCookieMinKey, MatchValue::U32(cookie_min));
        self
    }

    #[must_use]
    pub fn with_cookie_max(mut self, cookie_max: u32) -> Self {
        self.insert(ffi::kIOHIDElementCookieMaxKey, MatchValue::U32(cookie_max));
        self
    }

    #[must_use]
    pub fn with_report_id(mut self, report_id: u32) -> Self {
        self.insert(ffi::kIOHIDElementReportIDKey, MatchValue::U32(report_id));
        self
    }

    #[must_use]
    pub fn with_u32(mut self, key: &str, value: u32) -> Self {
        self.insert(key, MatchValue::U32(value));
        self
    }

    #[must_use]
    pub fn with_string(mut self, key: &str, value: impl Into<String>) -> Self {
        self.insert(key, MatchValue::String(value.into()));
        self
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn insert(&mut self, key: &str, value: MatchValue) {
        if let Some((_, existing)) = self
            .entries
            .iter_mut()
            .find(|(existing_key, _)| existing_key == key)
        {
            *existing = value;
            return;
        }
        self.entries.push((key.to_owned(), value));
    }

    fn to_cf_dictionary(&self) -> Result<ffi::CFDictionaryRef, HidError> {
        build_cf_dictionary(&self.entries)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HidReportType {
    Input,
    Output,
    Feature,
    Unknown(u32),
}

impl HidReportType {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::Input => ffi::kIOHIDReportTypeInput,
            Self::Output => ffi::kIOHIDReportTypeOutput,
            Self::Feature => ffi::kIOHIDReportTypeFeature,
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        match raw {
            ffi::kIOHIDReportTypeInput => Self::Input,
            ffi::kIOHIDReportTypeOutput => Self::Output,
            ffi::kIOHIDReportTypeFeature => Self::Feature,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HidValueScale {
    Calibrated,
    Physical,
    Exponent,
}

impl HidValueScale {
    #[must_use]
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::Calibrated => ffi::kIOHIDValueScaleTypeCalibrated,
            Self::Physical => ffi::kIOHIDValueScaleTypePhysical,
            Self::Exponent => ffi::kIOHIDValueScaleTypeExponent,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HidElementType {
    InputMisc,
    InputButton,
    InputAxis,
    InputScanCodes,
    InputNull,
    Output,
    Feature,
    Collection,
    Unknown(i32),
}

impl HidElementType {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            1 => Self::InputMisc,
            2 => Self::InputButton,
            3 => Self::InputAxis,
            4 => Self::InputScanCodes,
            5 => Self::InputNull,
            129 => Self::Output,
            257 => Self::Feature,
            513 => Self::Collection,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HidCollectionType {
    Physical,
    Application,
    Logical,
    Report,
    NamedArray,
    UsageSwitch,
    UsageModifier,
    Unknown(i32),
}

impl HidCollectionType {
    #[must_use]
    pub const fn from_raw(raw: i32) -> Self {
        match raw {
            0 => Self::Physical,
            1 => Self::Application,
            2 => Self::Logical,
            3 => Self::Report,
            4 => Self::NamedArray,
            5 => Self::UsageSwitch,
            6 => Self::UsageModifier,
            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidInputReport {
    pub report_type: HidReportType,
    pub report_id: u32,
    pub bytes: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HidDeviceInfo {
    pub vendor_id: Option<u32>,
    pub product_id: Option<u32>,
    pub product: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub transport: Option<String>,
    pub usage_page: Option<u32>,
    pub usage: Option<u32>,
    pub location_id: Option<u32>,
}

pub struct HidManager {
    raw: ffi::IOHIDManagerRef,
}

unsafe impl Send for HidManager {}
unsafe impl Sync for HidManager {}

impl Drop for HidManager {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe {
                let _ = ffi::IOHIDManagerClose(self.raw, ffi::kIOHIDOptionsTypeNone);
                ffi::CFRelease(self.raw.cast_const());
            }
            self.raw = ptr::null_mut();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidManager {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDManagerGetTypeID() }
    }

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

    pub fn set_device_matching(&self, usage: Option<HidUsage>) -> Result<(), HidError> {
        usage.map_or_else(
            || {
                unsafe { ffi::IOHIDManagerSetDeviceMatching(self.raw, ptr::null()) };
                Ok(())
            },
            |usage| self.set_device_matching_dict(Some(&DeviceMatch::usage(usage))),
        )
    }

    pub fn set_device_matching_dict(&self, matching: Option<&DeviceMatch>) -> Result<(), HidError> {
        match matching {
            None => unsafe {
                ffi::IOHIDManagerSetDeviceMatching(self.raw, ptr::null());
                Ok(())
            },
            Some(matching) => {
                let dict = matching.to_cf_dictionary()?;
                unsafe {
                    ffi::IOHIDManagerSetDeviceMatching(self.raw, dict);
                    ffi::CFRelease(dict.cast());
                }
                Ok(())
            }
        }
    }

    pub fn set_device_matching_multiple(&self, matches: &[DeviceMatch]) -> Result<(), HidError> {
        if matches.is_empty() {
            unsafe { ffi::IOHIDManagerSetDeviceMatchingMultiple(self.raw, ptr::null()) };
            return Ok(());
        }
        let array = build_cf_dictionary_array(matches, DeviceMatch::to_cf_dictionary)?;
        unsafe {
            ffi::IOHIDManagerSetDeviceMatchingMultiple(self.raw, array);
            ffi::CFRelease(array.cast());
        }
        Ok(())
    }

    pub fn save_to_property_domain(
        &self,
        application_id: &str,
        user_name: &str,
        host_name: &str,
        options: ffi::IOOptionBits,
    ) -> Result<(), HidError> {
        let app = make_cfstring(application_id)?;
        let user = match make_cfstring(user_name) {
            Ok(user) => user,
            Err(err) => {
                unsafe { ffi::CFRelease(app) };
                return Err(err);
            }
        };
        let host = match make_cfstring(host_name) {
            Ok(host) => host,
            Err(err) => {
                unsafe {
                    ffi::CFRelease(app);
                    ffi::CFRelease(user);
                }
                return Err(err);
            }
        };
        unsafe {
            ffi::IOHIDManagerSaveToPropertyDomain(self.raw, app, user, host, options);
            ffi::CFRelease(app);
            ffi::CFRelease(user);
            ffi::CFRelease(host);
        }
        Ok(())
    }

    #[must_use]
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    pub fn set_property_u32(&self, key: &str, value: u32) -> Result<(), HidError> {
        let value_cf = make_cfnumber_u32(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDManagerSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDManagerSetProperty"))
        }
    }

    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDManagerSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDManagerSetProperty"))
        }
    }

    pub fn set_property_data(&self, key: &str, value: &[u8]) -> Result<(), HidError> {
        let value_cf = make_cfdata(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDManagerSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDManagerSetProperty"))
        }
    }

    #[must_use]
    pub fn devices(&self) -> Vec<HidDeviceInfo> {
        let set = unsafe { ffi::IOHIDManagerCopyDevices(self.raw) };
        if set.is_null() {
            return Vec::new();
        }
        let count = unsafe { ffi::CFSetGetCount(set) };
        let count = usize::try_from(count).unwrap_or(0);
        let mut buffer = vec![ptr::null(); count];
        unsafe { ffi::CFSetGetValues(set, buffer.as_mut_ptr()) };
        let devices = buffer
            .into_iter()
            .filter(|device| !device.is_null())
            .map(|device| read_device_info(device.cast_mut()))
            .collect();
        unsafe { ffi::CFRelease(set.cast()) };
        devices
    }

    #[must_use]
    pub fn live_devices(&self) -> Vec<HidDevice> {
        let set = unsafe { ffi::IOHIDManagerCopyDevices(self.raw) };
        if set.is_null() {
            return Vec::new();
        }
        let count = unsafe { ffi::CFSetGetCount(set) };
        let count = usize::try_from(count).unwrap_or(0);
        let mut buffer = vec![ptr::null(); count];
        unsafe { ffi::CFSetGetValues(set, buffer.as_mut_ptr()) };
        let devices = buffer
            .into_iter()
            .filter(|device| !device.is_null())
            .map(|device| {
                unsafe { ffi::CFRetain(device) };
                HidDevice {
                    raw: device.cast_mut(),
                }
            })
            .collect();
        unsafe { ffi::CFRelease(set.cast()) };
        devices
    }

    #[must_use]
    pub const fn as_ptr(&self) -> ffi::IOHIDManagerRef {
        self.raw
    }
}

pub struct HidDevice {
    raw: ffi::IOHIDDeviceRef,
}

unsafe impl Send for HidDevice {}
unsafe impl Sync for HidDevice {}

impl Clone for HidDevice {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw.cast_const()) };
        Self { raw: self.raw }
    }
}

impl Drop for HidDevice {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

#[allow(clippy::missing_errors_doc, clippy::type_complexity)]
impl HidDevice {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDDeviceGetTypeID() }
    }

    #[must_use]
    pub fn info(&self) -> HidDeviceInfo {
        read_device_info(self.raw)
    }

    #[must_use]
    pub fn service(&self) -> ffi::io_service_t {
        unsafe { ffi::IOHIDDeviceGetService(self.raw) }
    }

    #[must_use]
    pub fn conforms_to(&self, usage: HidUsage) -> bool {
        let (page, usage) = usage.as_pair();
        unsafe { ffi::IOHIDDeviceConformsTo(self.raw, page, usage) }
    }

    #[must_use]
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn report_descriptor(&self) -> Option<Vec<u8>> {
        self.property_data(ffi::kIOHIDReportDescriptorKey)
    }

    pub fn set_property_u32(&self, key: &str, value: u32) -> Result<(), HidError> {
        let value_cf = make_cfnumber_u32(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDDeviceSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDDeviceSetProperty"))
        }
    }

    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDDeviceSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDDeviceSetProperty"))
        }
    }

    pub fn set_property_data(&self, key: &str, value: &[u8]) -> Result<(), HidError> {
        let value_cf = make_cfdata(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDDeviceSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDDeviceSetProperty"))
        }
    }

    pub fn set_input_value_matching(
        &self,
        matching: Option<&ElementMatch>,
    ) -> Result<(), HidError> {
        match matching {
            None => unsafe {
                ffi::IOHIDDeviceSetInputValueMatching(self.raw, ptr::null());
                Ok(())
            },
            Some(matching) => {
                let dict = matching.to_cf_dictionary()?;
                unsafe {
                    ffi::IOHIDDeviceSetInputValueMatching(self.raw, dict);
                    ffi::CFRelease(dict.cast());
                }
                Ok(())
            }
        }
    }

    pub fn set_input_value_matching_multiple(
        &self,
        matches: &[ElementMatch],
    ) -> Result<(), HidError> {
        if matches.is_empty() {
            unsafe { ffi::IOHIDDeviceSetInputValueMatchingMultiple(self.raw, ptr::null()) };
            return Ok(());
        }
        let array = build_cf_dictionary_array(matches, ElementMatch::to_cf_dictionary)?;
        unsafe {
            ffi::IOHIDDeviceSetInputValueMatchingMultiple(self.raw, array);
            ffi::CFRelease(array.cast());
        }
        Ok(())
    }

    pub fn on_input_report<F>(
        &self,
        max_report_length: usize,
        callback: F,
    ) -> Result<ReportSubscription, HidError>
    where
        F: Fn(&[u8]) + Send + Sync + 'static,
    {
        if max_report_length == 0 {
            return Err(HidError::InvalidArgument(
                "max_report_length must be greater than zero".to_owned(),
            ));
        }
        let run_loop = open_and_schedule_device(self.raw)?;
        let report_length = ffi::CFIndex::try_from(max_report_length).map_err(|_| {
            HidError::InvalidArgument("max_report_length does not fit CFIndex".to_owned())
        })?;
        let buffer = vec![0_u8; max_report_length].into_boxed_slice();
        let buffer_ptr = Box::into_raw(buffer);
        let callback: Box<dyn Fn(&[u8]) + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context = Box::new(ReportContext {
            buffer_len: max_report_length,
            callback: callback_ptr,
        });
        let context_ptr = Box::into_raw(context);
        unsafe {
            ffi::IOHIDDeviceRegisterInputReportCallback(
                self.raw,
                (*buffer_ptr).as_mut_ptr(),
                report_length,
                Some(report_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ReportSubscription {
            device: self.raw,
            run_loop,
            buffer_ptr,
            buffer_len: max_report_length,
            context: context_ptr,
        })
    }

    pub fn on_input_report_with_timestamp<F>(
        &self,
        max_report_length: usize,
        callback: F,
    ) -> Result<TimestampedReportSubscription, HidError>
    where
        F: Fn(HidInputReport) + Send + Sync + 'static,
    {
        if max_report_length == 0 {
            return Err(HidError::InvalidArgument(
                "max_report_length must be greater than zero".to_owned(),
            ));
        }
        let run_loop = open_and_schedule_device(self.raw)?;
        let report_length = ffi::CFIndex::try_from(max_report_length).map_err(|_| {
            HidError::InvalidArgument("max_report_length does not fit CFIndex".to_owned())
        })?;
        let buffer = vec![0_u8; max_report_length].into_boxed_slice();
        let buffer_ptr = Box::into_raw(buffer);
        let callback: Box<dyn Fn(HidInputReport) + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context = Box::new(TimestampedReportContext {
            buffer_len: max_report_length,
            callback: callback_ptr,
        });
        let context_ptr = Box::into_raw(context);
        unsafe {
            ffi::IOHIDDeviceRegisterInputReportWithTimeStampCallback(
                self.raw,
                (*buffer_ptr).as_mut_ptr(),
                report_length,
                Some(timestamped_report_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(TimestampedReportSubscription {
            device: self.raw,
            run_loop,
            buffer_ptr,
            buffer_len: max_report_length,
            context: context_ptr,
        })
    }

    pub fn on_input_value<F>(&self, callback: F) -> Result<ValueSubscription, HidError>
    where
        F: Fn(&HidValue) + Send + Sync + 'static,
    {
        let run_loop = open_and_schedule_device(self.raw)?;
        let callback: Box<dyn Fn(&HidValue) + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context = Box::new(ValueContext {
            callback: callback_ptr,
        });
        let context_ptr = Box::into_raw(context);
        unsafe {
            ffi::IOHIDDeviceRegisterInputValueCallback(
                self.raw,
                Some(value_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ValueSubscription {
            device: self.raw,
            run_loop,
            context: context_ptr,
        })
    }

    #[must_use]
    pub fn elements(&self) -> Vec<HidElement> {
        self.matching_elements(None)
    }

    #[must_use]
    pub fn matching_elements(&self, matching: Option<&ElementMatch>) -> Vec<HidElement> {
        let dict = matching.and_then(|matching| matching.to_cf_dictionary().ok());
        let elements = unsafe {
            ffi::IOHIDDeviceCopyMatchingElements(
                self.raw,
                dict.unwrap_or(ptr::null()),
                ffi::kIOHIDOptionsTypeNone,
            )
        };
        if let Some(dict) = dict {
            unsafe { ffi::CFRelease(dict.cast()) };
        }
        elements_from_array(elements, true)
    }

    pub fn get_value(&self, element: &HidElement) -> Result<HidValue, HidError> {
        let mut value = ptr::null_mut();
        let status = unsafe { ffi::IOHIDDeviceGetValue(self.raw, element.raw, &mut value) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceGetValue", status));
        }
        clone_value_ref(value).ok_or(HidError::OperationFailed("IOHIDDeviceGetValue"))
    }

    pub fn get_value_with_options(
        &self,
        element: &HidElement,
        options: u32,
    ) -> Result<HidValue, HidError> {
        let mut value = ptr::null_mut();
        let status = unsafe {
            ffi::IOHIDDeviceGetValueWithOptions(self.raw, element.raw, &mut value, options)
        };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceGetValueWithOptions", status));
        }
        clone_value_ref(value).ok_or(HidError::OperationFailed("IOHIDDeviceGetValueWithOptions"))
    }

    pub fn set_value(&self, element: &HidElement, value: &HidValue) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDDeviceSetValue(self.raw, element.raw, value.raw) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceSetValue", status))
        }
    }

    pub fn get_report(
        &self,
        report_type: HidReportType,
        report_id: u32,
        max_report_length: usize,
    ) -> Result<Vec<u8>, HidError> {
        if max_report_length == 0 {
            return Err(HidError::InvalidArgument(
                "max_report_length must be greater than zero".to_owned(),
            ));
        }
        let report_id = ffi::CFIndex::try_from(report_id)
            .map_err(|_| HidError::InvalidArgument("report_id does not fit CFIndex".to_owned()))?;
        let mut buffer = vec![0_u8; max_report_length];
        let mut report_length = ffi::CFIndex::try_from(max_report_length).map_err(|_| {
            HidError::InvalidArgument("max_report_length does not fit CFIndex".to_owned())
        })?;
        let status = unsafe {
            ffi::IOHIDDeviceGetReport(
                self.raw,
                report_type.as_raw(),
                report_id,
                buffer.as_mut_ptr(),
                &mut report_length,
            )
        };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceGetReport", status));
        }
        buffer.truncate(usize::try_from(report_length).unwrap_or(0));
        Ok(buffer)
    }

    pub fn set_report(
        &self,
        report_type: HidReportType,
        report_id: u32,
        report: &[u8],
    ) -> Result<(), HidError> {
        let report_id = ffi::CFIndex::try_from(report_id)
            .map_err(|_| HidError::InvalidArgument("report_id does not fit CFIndex".to_owned()))?;
        let report_length = ffi::CFIndex::try_from(report.len()).map_err(|_| {
            HidError::InvalidArgument("report length does not fit CFIndex".to_owned())
        })?;
        let status = unsafe {
            ffi::IOHIDDeviceSetReport(
                self.raw,
                report_type.as_raw(),
                report_id,
                report.as_ptr(),
                report_length,
            )
        };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceSetReport", status))
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> ffi::IOHIDDeviceRef {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HidElement {
    raw: ffi::IOHIDElementRef,
}

unsafe impl Send for HidElement {}
unsafe impl Sync for HidElement {}

#[allow(clippy::missing_errors_doc)]
impl HidElement {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDElementGetTypeID() }
    }

    #[must_use]
    pub fn element_type(&self) -> i32 {
        unsafe { ffi::IOHIDElementGetType(self.raw) }
    }

    #[must_use]
    pub fn element_kind(&self) -> HidElementType {
        HidElementType::from_raw(self.element_type())
    }

    #[must_use]
    pub fn collection_type(&self) -> Option<HidCollectionType> {
        if self.element_kind() != HidElementType::Collection {
            return None;
        }
        Some(HidCollectionType::from_raw(unsafe {
            ffi::IOHIDElementGetCollectionType(self.raw)
        }))
    }

    #[must_use]
    pub fn usage(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUsage(self.raw) }
    }

    #[must_use]
    pub fn usage_page(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUsagePage(self.raw) }
    }

    #[must_use]
    pub fn cookie(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetCookie(self.raw) }
    }

    #[must_use]
    pub fn is_virtual(&self) -> bool {
        unsafe { ffi::IOHIDElementIsVirtual(self.raw) }
    }

    #[must_use]
    pub fn is_relative(&self) -> bool {
        unsafe { ffi::IOHIDElementIsRelative(self.raw) }
    }

    #[must_use]
    pub fn is_wrapping(&self) -> bool {
        unsafe { ffi::IOHIDElementIsWrapping(self.raw) }
    }

    #[must_use]
    pub fn is_array(&self) -> bool {
        unsafe { ffi::IOHIDElementIsArray(self.raw) }
    }

    #[must_use]
    pub fn is_non_linear(&self) -> bool {
        unsafe { ffi::IOHIDElementIsNonLinear(self.raw) }
    }

    #[must_use]
    pub fn has_preferred_state(&self) -> bool {
        unsafe { ffi::IOHIDElementHasPreferredState(self.raw) }
    }

    #[must_use]
    pub fn has_null_state(&self) -> bool {
        unsafe { ffi::IOHIDElementHasNullState(self.raw) }
    }

    #[must_use]
    pub fn name(&self) -> Option<String> {
        read_cf_string(unsafe { ffi::IOHIDElementGetName(self.raw).cast() })
    }

    #[must_use]
    pub fn report_id(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportID(self.raw) }
    }

    #[must_use]
    pub fn report_size_bits(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportSize(self.raw) }
    }

    #[must_use]
    pub fn report_count(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportCount(self.raw) }
    }

    #[must_use]
    pub fn unit(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUnit(self.raw) }
    }

    #[must_use]
    pub fn unit_exponent(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUnitExponent(self.raw) }
    }

    #[must_use]
    pub fn logical_min(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetLogicalMin(self.raw) as i64 }
    }

    #[must_use]
    pub fn logical_max(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetLogicalMax(self.raw) as i64 }
    }

    #[must_use]
    pub fn physical_min(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetPhysicalMin(self.raw) as i64 }
    }

    #[must_use]
    pub fn physical_max(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetPhysicalMax(self.raw) as i64 }
    }

    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        let parent = unsafe { ffi::IOHIDElementGetParent(self.raw) };
        (!parent.is_null()).then_some(Self { raw: parent })
    }

    #[must_use]
    pub fn children(&self) -> Vec<Self> {
        elements_from_array(unsafe { ffi::IOHIDElementGetChildren(self.raw) }, false)
    }

    #[must_use]
    pub fn attached(&self) -> Vec<Self> {
        elements_from_array(unsafe { ffi::IOHIDElementCopyAttached(self.raw) }, true)
    }

    #[must_use]
    pub fn device(&self) -> Option<HidDevice> {
        let device = unsafe { ffi::IOHIDElementGetDevice(self.raw) };
        if device.is_null() {
            None
        } else {
            unsafe { ffi::CFRetain(device.cast_const()) };
            Some(HidDevice { raw: device })
        }
    }

    #[must_use]
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    pub fn set_property_u32(&self, key: &str, value: u32) -> Result<(), HidError> {
        let value_cf = make_cfnumber_u32(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDElementSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDElementSetProperty"))
        }
    }

    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDElementSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDElementSetProperty"))
        }
    }

    pub fn set_property_data(&self, key: &str, value: &[u8]) -> Result<(), HidError> {
        let value_cf = make_cfdata(value)?;
        let ok = with_cfstring(key, |key_cf| unsafe {
            ffi::IOHIDElementSetProperty(self.raw, key_cf, value_cf.cast())
        })?;
        unsafe { ffi::CFRelease(value_cf.cast()) };
        if ok {
            Ok(())
        } else {
            Err(HidError::OperationFailed("IOHIDElementSetProperty"))
        }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> ffi::IOHIDElementRef {
        self.raw
    }
}

pub struct HidValue {
    raw: ffi::IOHIDValueRef,
}

unsafe impl Send for HidValue {}
unsafe impl Sync for HidValue {}

impl Clone for HidValue {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw.cast_const()) };
        Self { raw: self.raw }
    }
}

impl fmt::Debug for HidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HidValue")
            .field("timestamp", &self.timestamp())
            .field("len", &self.len())
            .field("integer_value", &self.integer_value())
            .field("usage_page", &self.element().usage_page())
            .field("usage", &self.element().usage())
            .finish()
    }
}

impl Drop for HidValue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidValue {
    #[must_use]
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDValueGetTypeID() }
    }

    pub fn from_integer(element: HidElement, timestamp: u64, value: i64) -> Result<Self, HidError> {
        let value = ffi::CFIndex::try_from(value).map_err(|_| {
            HidError::InvalidArgument("integer value does not fit CFIndex".to_owned())
        })?;
        let raw = unsafe {
            ffi::IOHIDValueCreateWithIntegerValue(
                ffi::kCFAllocatorDefault,
                element.raw,
                timestamp,
                value,
            )
        };
        if raw.is_null() {
            Err(HidError::OperationFailed(
                "IOHIDValueCreateWithIntegerValue",
            ))
        } else {
            Ok(Self { raw })
        }
    }

    pub fn from_bytes(element: HidElement, timestamp: u64, bytes: &[u8]) -> Result<Self, HidError> {
        let length = ffi::CFIndex::try_from(bytes.len()).map_err(|_| {
            HidError::InvalidArgument("byte slice length does not fit CFIndex".to_owned())
        })?;
        let raw = unsafe {
            ffi::IOHIDValueCreateWithBytes(
                ffi::kCFAllocatorDefault,
                element.raw,
                timestamp,
                bytes.as_ptr(),
                length,
            )
        };
        if raw.is_null() {
            Err(HidError::OperationFailed("IOHIDValueCreateWithBytes"))
        } else {
            Ok(Self { raw })
        }
    }

    #[must_use]
    pub fn element(&self) -> HidElement {
        HidElement {
            raw: unsafe { ffi::IOHIDValueGetElement(self.raw) },
        }
    }

    #[must_use]
    pub fn timestamp(&self) -> u64 {
        unsafe { ffi::IOHIDValueGetTimeStamp(self.raw) }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        usize::try_from(unsafe { ffi::IOHIDValueGetLength(self.raw) }).unwrap_or(0)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn bytes(&self) -> Vec<u8> {
        let length = self.len();
        if length == 0 {
            return Vec::new();
        }
        let bytes = unsafe { ffi::IOHIDValueGetBytePtr(self.raw) };
        if bytes.is_null() {
            return Vec::new();
        }
        unsafe { core::slice::from_raw_parts(bytes, length) }.to_vec()
    }

    #[must_use]
    pub fn integer_value(&self) -> i64 {
        unsafe { ffi::IOHIDValueGetIntegerValue(self.raw) as i64 }
    }

    #[must_use]
    pub fn scaled_value(&self, scale: HidValueScale) -> f64 {
        unsafe { ffi::IOHIDValueGetScaledValue(self.raw, scale.as_raw()) }
    }

    #[must_use]
    pub const fn as_ptr(&self) -> ffi::IOHIDValueRef {
        self.raw
    }
}

#[allow(clippy::type_complexity)]
struct ReportContext {
    buffer_len: usize,
    callback: *mut Box<dyn Fn(&[u8]) + Send + Sync + 'static>,
}

#[allow(clippy::type_complexity)]
struct TimestampedReportContext {
    buffer_len: usize,
    callback: *mut Box<dyn Fn(HidInputReport) + Send + Sync + 'static>,
}

#[allow(clippy::type_complexity)]
struct ValueContext {
    callback: *mut Box<dyn Fn(&HidValue) + Send + Sync + 'static>,
}

unsafe extern "C" fn report_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    _report_type: ffi::IOHIDReportType,
    _report_id: u32,
    report: *mut u8,
    report_length: ffi::CFIndex,
) {
    if context.is_null() || report.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let context = unsafe { &*context.cast::<ReportContext>() };
    let length = usize::try_from(report_length)
        .unwrap_or(0)
        .min(context.buffer_len);
    let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) };
    let callback = unsafe { &*context.callback };
    callback(bytes);
}

unsafe extern "C" fn timestamped_report_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    report_type: ffi::IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: ffi::CFIndex,
    timestamp: u64,
) {
    if context.is_null() || report.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let context = unsafe { &*context.cast::<TimestampedReportContext>() };
    let length = usize::try_from(report_length)
        .unwrap_or(0)
        .min(context.buffer_len);
    let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
    let callback = unsafe { &*context.callback };
    callback(HidInputReport {
        report_type: HidReportType::from_raw(report_type),
        report_id,
        bytes,
        timestamp,
    });
}

unsafe extern "C" fn value_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    value: ffi::IOHIDValueRef,
) {
    if context.is_null() || value.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let Some(value) = clone_value_ref(value) else {
        return;
    };
    let context = unsafe { &*context.cast::<ValueContext>() };
    let callback = unsafe { &*context.callback };
    callback(&value);
}

pub struct ReportSubscription {
    device: ffi::IOHIDDeviceRef,
    run_loop: ffi::CFRunLoopRef,
    buffer_ptr: *mut [u8],
    buffer_len: usize,
    context: *mut ReportContext,
}

unsafe impl Send for ReportSubscription {}

impl Drop for ReportSubscription {
    fn drop(&mut self) {
        if self.device.is_null() || self.buffer_ptr.is_null() || self.context.is_null() {
            return;
        }
        let report_length = ffi::CFIndex::try_from(self.buffer_len).unwrap_or(0);
        unsafe {
            ffi::IOHIDDeviceRegisterInputReportCallback(
                self.device,
                (*self.buffer_ptr).as_mut_ptr(),
                report_length,
                None,
                ptr::null_mut(),
            );
            close_and_unschedule_device(self.device, self.run_loop);
            ffi::CFRelease(self.device.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
            let _ = Box::from_raw(self.buffer_ptr);
        }
        self.device = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

pub struct TimestampedReportSubscription {
    device: ffi::IOHIDDeviceRef,
    run_loop: ffi::CFRunLoopRef,
    buffer_ptr: *mut [u8],
    buffer_len: usize,
    context: *mut TimestampedReportContext,
}

unsafe impl Send for TimestampedReportSubscription {}

impl Drop for TimestampedReportSubscription {
    fn drop(&mut self) {
        if self.device.is_null() || self.buffer_ptr.is_null() || self.context.is_null() {
            return;
        }
        let report_length = ffi::CFIndex::try_from(self.buffer_len).unwrap_or(0);
        unsafe {
            ffi::IOHIDDeviceRegisterInputReportWithTimeStampCallback(
                self.device,
                (*self.buffer_ptr).as_mut_ptr(),
                report_length,
                None,
                ptr::null_mut(),
            );
            close_and_unschedule_device(self.device, self.run_loop);
            ffi::CFRelease(self.device.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
            let _ = Box::from_raw(self.buffer_ptr);
        }
        self.device = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

pub struct ValueSubscription {
    device: ffi::IOHIDDeviceRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut ValueContext,
}

unsafe impl Send for ValueSubscription {}

impl Drop for ValueSubscription {
    fn drop(&mut self) {
        if self.device.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            ffi::IOHIDDeviceRegisterInputValueCallback(self.device, None, ptr::null_mut());
            close_and_unschedule_device(self.device, self.run_loop);
            ffi::CFRelease(self.device.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.device = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

fn open_and_schedule_device(device: ffi::IOHIDDeviceRef) -> Result<ffi::CFRunLoopRef, HidError> {
    let status = unsafe { ffi::IOHIDDeviceOpen(device, ffi::kIOHIDOptionsTypeNone) };
    if status != ffi::kIOReturnSuccess {
        return Err(HidError::DeviceOpenFailed(status));
    }
    let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
    unsafe {
        ffi::IOHIDDeviceScheduleWithRunLoop(device, run_loop, ffi::kCFRunLoopDefaultMode);
    }
    Ok(run_loop)
}

unsafe fn close_and_unschedule_device(device: ffi::IOHIDDeviceRef, run_loop: ffi::CFRunLoopRef) {
    ffi::IOHIDDeviceUnscheduleFromRunLoop(device, run_loop, ffi::kCFRunLoopDefaultMode);
    let _ = ffi::IOHIDDeviceClose(device, ffi::kIOHIDOptionsTypeNone);
}

fn clone_value_ref(raw: ffi::IOHIDValueRef) -> Option<HidValue> {
    if raw.is_null() {
        return None;
    }
    let element = unsafe { ffi::IOHIDValueGetElement(raw) };
    if element.is_null() {
        return None;
    }
    let timestamp = unsafe { ffi::IOHIDValueGetTimeStamp(raw) };
    let length = unsafe { ffi::IOHIDValueGetLength(raw) };
    if length > 0 {
        let bytes = unsafe { ffi::IOHIDValueGetBytePtr(raw) };
        if !bytes.is_null() {
            let copied = unsafe {
                ffi::IOHIDValueCreateWithBytes(
                    ffi::kCFAllocatorDefault,
                    element,
                    timestamp,
                    bytes,
                    length,
                )
            };
            if !copied.is_null() {
                return Some(HidValue { raw: copied });
            }
        }
    }
    let integer = unsafe { ffi::IOHIDValueGetIntegerValue(raw) };
    let copied = unsafe {
        ffi::IOHIDValueCreateWithIntegerValue(ffi::kCFAllocatorDefault, element, timestamp, integer)
    };
    (!copied.is_null()).then_some(HidValue { raw: copied })
}

fn build_cf_dictionary(entries: &[(String, MatchValue)]) -> Result<ffi::CFDictionaryRef, HidError> {
    let capacity = ffi::CFIndex::try_from(entries.len()).map_err(|_| {
        HidError::InvalidArgument("dictionary size does not fit CFIndex".to_owned())
    })?;
    let dict = unsafe {
        ffi::CFDictionaryCreateMutable(
            ffi::kCFAllocatorDefault,
            capacity,
            &raw const ffi::kCFTypeDictionaryKeyCallBacks,
            &raw const ffi::kCFTypeDictionaryValueCallBacks,
        )
    };
    if dict.is_null() {
        return Err(HidError::OperationFailed("CFDictionaryCreateMutable"));
    }

    for (key, value) in entries {
        let key_cf = match make_cfstring(key) {
            Ok(key_cf) => key_cf,
            Err(err) => {
                unsafe { ffi::CFRelease(dict.cast()) };
                return Err(err);
            }
        };
        let value_cf: ffi::CFTypeRef = match value {
            MatchValue::U32(value) => match make_cfnumber_u32(*value) {
                Ok(value_cf) => value_cf.cast(),
                Err(err) => {
                    unsafe {
                        ffi::CFRelease(key_cf);
                        ffi::CFRelease(dict.cast());
                    }
                    return Err(err);
                }
            },
            MatchValue::String(value) => match make_cfstring(value) {
                Ok(value_cf) => value_cf.cast(),
                Err(err) => {
                    unsafe {
                        ffi::CFRelease(key_cf);
                        ffi::CFRelease(dict.cast());
                    }
                    return Err(err);
                }
            },
        };
        unsafe {
            ffi::CFDictionarySetValue(dict, key_cf.cast(), value_cf.cast());
            ffi::CFRelease(key_cf);
            ffi::CFRelease(value_cf);
        }
    }

    Ok(dict.cast())
}

fn build_cf_dictionary_array<T, F>(items: &[T], build: F) -> Result<ffi::CFArrayRef, HidError>
where
    F: Fn(&T) -> Result<ffi::CFDictionaryRef, HidError>,
{
    let capacity = ffi::CFIndex::try_from(items.len())
        .map_err(|_| HidError::InvalidArgument("array size does not fit CFIndex".to_owned()))?;
    let array = unsafe {
        ffi::CFArrayCreateMutable(
            ffi::kCFAllocatorDefault,
            capacity,
            &raw const ffi::kCFTypeArrayCallBacks,
        )
    };
    if array.is_null() {
        return Err(HidError::OperationFailed("CFArrayCreateMutable"));
    }
    for item in items {
        let dict = match build(item) {
            Ok(dict) => dict,
            Err(err) => {
                unsafe { ffi::CFRelease(array.cast()) };
                return Err(err);
            }
        };
        unsafe {
            ffi::CFArrayAppendValue(array, dict.cast());
            ffi::CFRelease(dict.cast());
        }
    }
    Ok(array.cast())
}

fn elements_from_array(array: ffi::CFArrayRef, release_after: bool) -> Vec<HidElement> {
    if array.is_null() {
        return Vec::new();
    }
    let count = unsafe { ffi::CFArrayGetCount(array) };
    let count = usize::try_from(count).unwrap_or(0);
    let mut elements = Vec::with_capacity(count);
    for index in 0..count {
        let raw = unsafe {
            ffi::CFArrayGetValueAtIndex(array, ffi::CFIndex::try_from(index).unwrap_or(0))
        };
        if !raw.is_null() {
            elements.push(HidElement {
                raw: raw.cast_mut(),
            });
        }
    }
    if release_after {
        unsafe { ffi::CFRelease(array.cast()) };
    }
    elements
}

fn read_device_info(device: ffi::IOHIDDeviceRef) -> HidDeviceInfo {
    HidDeviceInfo {
        vendor_id: read_device_property_u32(device, ffi::kIOHIDVendorIDKey),
        product_id: read_device_property_u32(device, ffi::kIOHIDProductIDKey),
        product: read_device_property_string(device, ffi::kIOHIDProductKey),
        manufacturer: read_device_property_string(device, ffi::kIOHIDManufacturerKey),
        serial_number: read_device_property_string(device, ffi::kIOHIDSerialNumberKey),
        transport: read_device_property_string(device, ffi::kIOHIDTransportKey),
        usage_page: read_device_property_u32(device, ffi::kIOHIDPrimaryUsagePageKey),
        usage: read_device_property_u32(device, ffi::kIOHIDPrimaryUsageKey),
        location_id: read_device_property_u32(device, ffi::kIOHIDLocationIDKey),
    }
}

fn read_device_property_u32(device: ffi::IOHIDDeviceRef, key: &str) -> Option<u32> {
    with_cfstring(key, |key_cf| unsafe {
        read_cf_u32(ffi::IOHIDDeviceGetProperty(device, key_cf))
    })
    .ok()
    .flatten()
}

fn read_device_property_string(device: ffi::IOHIDDeviceRef, key: &str) -> Option<String> {
    with_cfstring(key, |key_cf| unsafe {
        read_cf_string(ffi::IOHIDDeviceGetProperty(device, key_cf))
    })
    .ok()
    .flatten()
}

fn read_cf_u32(value: ffi::CFTypeRef) -> Option<u32> {
    if value.is_null() || unsafe { ffi::CFGetTypeID(value) } != unsafe { ffi::CFNumberGetTypeID() }
    {
        return None;
    }
    let mut number = 0_i64;
    let ok = unsafe {
        ffi::CFNumberGetValue(
            value.cast(),
            ffi::kCFNumberSInt64Type,
            ptr::from_mut(&mut number).cast(),
        )
    };
    ok.then(|| u32::try_from(number).ok()).flatten()
}

fn read_cf_string(value: ffi::CFTypeRef) -> Option<String> {
    if value.is_null() || unsafe { ffi::CFGetTypeID(value) } != unsafe { ffi::CFStringGetTypeID() }
    {
        return None;
    }
    cfstring_to_string(value.cast())
}

fn read_cf_data(value: ffi::CFTypeRef) -> Option<Vec<u8>> {
    if value.is_null() || unsafe { ffi::CFGetTypeID(value) } != unsafe { ffi::CFDataGetTypeID() } {
        return None;
    }
    let length = usize::try_from(unsafe { ffi::CFDataGetLength(value.cast()) }).unwrap_or(0);
    if length == 0 {
        return Some(Vec::new());
    }
    let bytes = unsafe { ffi::CFDataGetBytePtr(value.cast()) };
    if bytes.is_null() {
        return None;
    }
    Some(unsafe { core::slice::from_raw_parts(bytes, length) }.to_vec())
}

fn cfstring_to_string(value: ffi::CFStringRef) -> Option<String> {
    if value.is_null() {
        return None;
    }
    let length = unsafe { ffi::CFStringGetLength(value) };
    let capacity = usize::try_from((length * 4) + 1).unwrap_or(0);
    let mut buffer = vec![0_u8; capacity];
    let ok = unsafe {
        ffi::CFStringGetCString(
            value,
            buffer.as_mut_ptr().cast::<c_char>(),
            ffi::CFIndex::try_from(buffer.len()).unwrap_or(0),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if !ok {
        return None;
    }
    if let Some(end) = buffer.iter().position(|byte| *byte == 0) {
        buffer.truncate(end);
    }
    String::from_utf8(buffer).ok()
}

fn with_cfstring<T, F>(value: &str, f: F) -> Result<T, HidError>
where
    F: FnOnce(ffi::CFStringRef) -> T,
{
    let value_cf = make_cfstring(value)?;
    let result = f(value_cf);
    unsafe { ffi::CFRelease(value_cf) };
    Ok(result)
}

fn make_cfstring(value: &str) -> Result<ffi::CFStringRef, HidError> {
    let value = CString::new(value).map_err(|err| HidError::InvalidArgument(err.to_string()))?;
    let string = unsafe {
        ffi::CFStringCreateWithCString(
            ffi::kCFAllocatorDefault,
            value.as_ptr(),
            ffi::kCFStringEncodingUTF8,
        )
    };
    if string.is_null() {
        Err(HidError::OperationFailed("CFStringCreateWithCString"))
    } else {
        Ok(string)
    }
}

fn make_cfnumber_u32(value: u32) -> Result<ffi::CFNumberRef, HidError> {
    let value = i64::from(value);
    let number = unsafe {
        ffi::CFNumberCreate(
            ffi::kCFAllocatorDefault,
            ffi::kCFNumberSInt64Type,
            ptr::from_ref(&value).cast(),
        )
    };
    if number.is_null() {
        Err(HidError::OperationFailed("CFNumberCreate"))
    } else {
        Ok(number)
    }
}

fn make_cfdata(value: &[u8]) -> Result<ffi::CFDataRef, HidError> {
    let length = ffi::CFIndex::try_from(value.len()).map_err(|_| {
        HidError::InvalidArgument("byte slice length does not fit CFIndex".to_owned())
    })?;
    let data = unsafe { ffi::CFDataCreate(ffi::kCFAllocatorDefault, value.as_ptr(), length) };
    if data.is_null() {
        Err(HidError::OperationFailed("CFDataCreate"))
    } else {
        Ok(data)
    }
}
