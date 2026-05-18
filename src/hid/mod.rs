//! High-level `IOKit` HID wrappers.

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::fmt;

use crate::error::HidError;
use crate::ffi_impl as ffi;

/// Device helpers wrapping `IOHIDDevice*` APIs.
pub mod device;
/// Element helpers wrapping `IOHIDElement*` APIs.
pub mod element;
/// Event-system keys and option constants mirrored from `IOHIDEventSystemKeys.h`.
pub mod event_system;
/// Property keys and option constants mirrored from `IOHIDKeys.h`.
pub mod keys;
/// Manager helpers wrapping `IOHIDManager*` callbacks and option flags.
pub mod manager;
/// Queue helpers wrapping `IOHIDQueue*` APIs.
pub mod queue;
/// Service plug-in UUIDs and COM-style interfaces mirrored from `IOHIDDevicePlugIn.h`.
pub mod service_plugin;
/// Transaction helpers wrapping `IOHIDTransaction*` APIs.
pub mod transaction;
/// Usage-page and usage constants mirrored from `IOHIDUsageTables.h`.
pub mod usage;
/// Value helpers wrapping `IOHIDValue*` APIs.
pub mod value;

pub use device::DeviceRemovalSubscription;
pub use manager::{
    HidManagerOptions, ManagerDeviceSubscription, ManagerReportSubscription,
    ManagerValueSubscription,
};
pub use queue::{HidQueue, HidQueueOptions, QueueValueAvailableSubscription};
pub use transaction::{HidTransaction, HidTransactionDirection, HidTransactionOptions};

/// Aliases `IOHIDDeviceGetValueOptions`.
pub type IOHIDDeviceGetValueOptions = ffi::IOHIDDeviceGetValueOptions;
/// Mirrors `kIOHIDDeviceGetValueWithUpdate`.
pub const DEVICE_GET_VALUE_WITH_UPDATE: IOHIDDeviceGetValueOptions =
    ffi::kIOHIDDeviceGetValueWithUpdate;
/// Mirrors `kIOHIDDeviceGetValueWithoutUpdate`.
pub const DEVICE_GET_VALUE_WITHOUT_UPDATE: IOHIDDeviceGetValueOptions =
    ffi::kIOHIDDeviceGetValueWithoutUpdate;

/// Aliases `IOHIDElementCommitDirection`.
pub type IOHIDElementCommitDirection = ffi::IOHIDElementCommitDirection;
/// Mirrors `kIOHIDElementCommitDirectionIn`.
pub const ELEMENT_COMMIT_DIRECTION_IN: IOHIDElementCommitDirection =
    ffi::kIOHIDElementCommitDirectionIn;
/// Mirrors `kIOHIDElementCommitDirectionOut`.
pub const ELEMENT_COMMIT_DIRECTION_OUT: IOHIDElementCommitDirection =
    ffi::kIOHIDElementCommitDirectionOut;

/// Aliases `IOHIDElementCookie`.
pub type IOHIDElementCookie = ffi::IOHIDElementCookie;
/// Aliases `IOHIDElementFlags`.
pub type IOHIDElementFlags = ffi::IOHIDElementFlags;
/// Mirrors `kIOHIDElementFlagsConstantMask`.
pub const ELEMENT_FLAGS_CONSTANT_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsConstantMask;
/// Mirrors `kIOHIDElementFlagsVariableMask`.
pub const ELEMENT_FLAGS_VARIABLE_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsVariableMask;
/// Mirrors `kIOHIDElementFlagsRelativeMask`.
pub const ELEMENT_FLAGS_RELATIVE_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsRelativeMask;
/// Mirrors `kIOHIDElementFlagsWrapMask`.
pub const ELEMENT_FLAGS_WRAP_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsWrapMask;
/// Mirrors `kIOHIDElementFlagsNonLinearMask`.
pub const ELEMENT_FLAGS_NON_LINEAR_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsNonLinearMask;
/// Mirrors `kIOHIDElementFlagsNoPreferredMask`.
pub const ELEMENT_FLAGS_NO_PREFERRED_MASK: IOHIDElementFlags =
    ffi::kIOHIDElementFlagsNoPreferredMask;
/// Mirrors `kIOHIDElementFlagsNullStateMask`.
pub const ELEMENT_FLAGS_NULL_STATE_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsNullStateMask;
/// Mirrors `kIOHIDElementFlagsVolativeMask`.
pub const ELEMENT_FLAGS_VOLATIVE_MASK: IOHIDElementFlags = ffi::kIOHIDElementFlagsVolativeMask;
/// Mirrors `kIOHIDElementFlagsBufferedByteMask`.
pub const ELEMENT_FLAGS_BUFFERED_BYTE_MASK: IOHIDElementFlags =
    ffi::kIOHIDElementFlagsBufferedByteMask;

/// Aliases `IOHIDValueOptions`.
pub type IOHIDValueOptions = ffi::IOHIDValueOptions;
/// Mirrors `kIOHIDValueOptionsFlagRelativeSimple`.
pub const VALUE_OPTIONS_FLAG_RELATIVE_SIMPLE: IOHIDValueOptions =
    ffi::kIOHIDValueOptionsFlagRelativeSimple;
/// Mirrors `kIOHIDValueOptionsFlagPrevious`.
pub const VALUE_OPTIONS_FLAG_PREVIOUS: IOHIDValueOptions = ffi::kIOHIDValueOptionsFlagPrevious;
/// Mirrors `kIOHIDValueOptionsUpdateElementValues`.
pub const VALUE_OPTIONS_UPDATE_ELEMENT_VALUES: IOHIDValueOptions =
    ffi::kIOHIDValueOptionsUpdateElementValues;

/// Aliases `IOHIDCompletionAction`.
pub type IOHIDCompletionAction = ffi::IOHIDCompletionAction;
/// Aliases `IOHIDCompletion`.
pub type IOHIDCompletion = ffi::IOHIDCompletion;
/// Aliases `HIDReportCommandType`.
pub type HIDReportCommandType = ffi::HIDReportCommandType;
/// Mirrors `kIOHIDReportCommandSetReport`.
pub const REPORT_COMMAND_SET_REPORT: HIDReportCommandType = ffi::kIOHIDReportCommandSetReport;
/// Mirrors `kIOHIDReportCommandGetReport`.
pub const REPORT_COMMAND_GET_REPORT: HIDReportCommandType = ffi::kIOHIDReportCommandGetReport;
/// Mirrors `kIOHIDReportOptionNotInterrupt`.
pub const REPORT_OPTION_NOT_INTERRUPT: ffi::IOOptionBits = ffi::kIOHIDReportOptionNotInterrupt;
/// Mirrors `kIOHIDReportOptionVariableSize`.
pub const REPORT_OPTION_VARIABLE_SIZE: ffi::IOOptionBits = ffi::kIOHIDReportOptionVariableSize;
/// Mirrors `kIOHIDDeviceDefaultAsyncRequestTimeout`.
pub const DEVICE_DEFAULT_ASYNC_REQUEST_TIMEOUT: u64 = ffi::kIOHIDDeviceDefaultAsyncRequestTimeout;
/// Mirrors `kIOHIDDeviceMinAsyncRequestTimeout`.
pub const DEVICE_MIN_ASYNC_REQUEST_TIMEOUT: u64 = ffi::kIOHIDDeviceMinAsyncRequestTimeout;
/// Mirrors `kIOHIDDeviceMaxAsyncRequestTimeout`.
pub const DEVICE_MAX_ASYNC_REQUEST_TIMEOUT: u64 = ffi::kIOHIDDeviceMaxAsyncRequestTimeout;

/// Aliases `IOHIDManagerOptions`.
pub type IOHIDManagerOptions = ffi::IOHIDManagerOptions;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
/// Describes a `(usage_page, usage)` pair used by `IOHIDManagerSetDeviceMatching`.
pub enum HidUsage {
    /// Matches `kHIDUsage_GD_Keyboard` on `kHIDPage_GenericDesktop`.
    Keyboard,
    /// Matches `kHIDUsage_GD_Mouse` on `kHIDPage_GenericDesktop`.
    Mouse,
    /// Matches `kHIDUsage_GD_Joystick` on `kHIDPage_GenericDesktop`.
    Joystick,
    /// Matches `kHIDUsage_GD_GamePad` on `kHIDPage_GenericDesktop`.
    GamePad,
    /// Matches an arbitrary `(usage_page, usage)` pair.
    Custom(u32, u32),
}

impl HidUsage {
    #[must_use]
    /// Returns the `(usage_page, usage)` pair consumed by `IOHIDManagerSetDeviceMatching`.
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
/// Builds matching dictionaries for `IOHIDManagerSetDeviceMatching`.
pub struct DeviceMatch {
    entries: Vec<(String, MatchValue)>,
}

impl DeviceMatch {
    #[must_use]
    /// Creates an empty device-matching dictionary for `IOHIDManagerSetDeviceMatching`.
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    /// Creates a device-matching dictionary from a `HidUsage` pair.
    pub fn usage(usage: HidUsage) -> Self {
        Self::new().with_usage(usage)
    }

    #[must_use]
    /// Adds `kIOHIDDeviceUsagePageKey` and `kIOHIDDeviceUsageKey`.
    pub fn with_usage(mut self, usage: HidUsage) -> Self {
        let (page, usage) = usage.as_pair();
        self.insert(ffi::kIOHIDDeviceUsagePageKey, MatchValue::U32(page));
        self.insert(ffi::kIOHIDDeviceUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    /// Adds `kIOHIDVendorIDKey`.
    pub fn with_vendor_id(mut self, vendor_id: u32) -> Self {
        self.insert(ffi::kIOHIDVendorIDKey, MatchValue::U32(vendor_id));
        self
    }

    #[must_use]
    /// Adds `kIOHIDProductIDKey`.
    pub fn with_product_id(mut self, product_id: u32) -> Self {
        self.insert(ffi::kIOHIDProductIDKey, MatchValue::U32(product_id));
        self
    }

    #[must_use]
    /// Adds `kIOHIDLocationIDKey`.
    pub fn with_location_id(mut self, location_id: u32) -> Self {
        self.insert(ffi::kIOHIDLocationIDKey, MatchValue::U32(location_id));
        self
    }

    #[must_use]
    /// Adds `kIOHIDTransportKey`.
    pub fn with_transport(mut self, transport: impl Into<String>) -> Self {
        self.insert(
            ffi::kIOHIDTransportKey,
            MatchValue::String(transport.into()),
        );
        self
    }

    #[must_use]
    /// Adds an arbitrary integer entry to the matching dictionary.
    pub fn with_u32(mut self, key: &str, value: u32) -> Self {
        self.insert(key, MatchValue::U32(value));
        self
    }

    #[must_use]
    /// Adds an arbitrary string entry to the matching dictionary.
    pub fn with_string(mut self, key: &str, value: impl Into<String>) -> Self {
        self.insert(key, MatchValue::String(value.into()));
        self
    }

    #[must_use]
    /// Returns `true` when no entries will be passed to `IOHIDManagerSetDeviceMatching`.
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
/// Builds matching dictionaries for `IOHIDDeviceSetInputValueMatching` and `IOHIDManagerSetInputValueMatching`.
pub struct ElementMatch {
    entries: Vec<(String, MatchValue)>,
}

impl ElementMatch {
    #[must_use]
    /// Creates an empty element-matching dictionary for `IOHIDDeviceSetInputValueMatching`.
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    /// Creates an element-matching dictionary from a `HidUsage` pair.
    pub fn usage(usage: HidUsage) -> Self {
        Self::new().with_usage(usage)
    }

    #[must_use]
    /// Adds `kIOHIDElementUsagePageKey` and `kIOHIDElementUsageKey`.
    pub fn with_usage(mut self, usage: HidUsage) -> Self {
        let (page, usage) = usage.as_pair();
        self.insert(ffi::kIOHIDElementUsagePageKey, MatchValue::U32(page));
        self.insert(ffi::kIOHIDElementUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementUsagePageKey`.
    pub fn with_usage_page(mut self, usage_page: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsagePageKey, MatchValue::U32(usage_page));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementUsageKey`.
    pub fn with_usage_id(mut self, usage: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageKey, MatchValue::U32(usage));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementUsageMinKey`.
    pub fn with_usage_min(mut self, usage_min: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageMinKey, MatchValue::U32(usage_min));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementUsageMaxKey`.
    pub fn with_usage_max(mut self, usage_max: u32) -> Self {
        self.insert(ffi::kIOHIDElementUsageMaxKey, MatchValue::U32(usage_max));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementCookieMinKey`.
    pub fn with_cookie_min(mut self, cookie_min: u32) -> Self {
        self.insert(ffi::kIOHIDElementCookieMinKey, MatchValue::U32(cookie_min));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementCookieMaxKey`.
    pub fn with_cookie_max(mut self, cookie_max: u32) -> Self {
        self.insert(ffi::kIOHIDElementCookieMaxKey, MatchValue::U32(cookie_max));
        self
    }

    #[must_use]
    /// Adds `kIOHIDElementReportIDKey`.
    pub fn with_report_id(mut self, report_id: u32) -> Self {
        self.insert(ffi::kIOHIDElementReportIDKey, MatchValue::U32(report_id));
        self
    }

    #[must_use]
    /// Adds an arbitrary integer entry to the element-matching dictionary.
    pub fn with_u32(mut self, key: &str, value: u32) -> Self {
        self.insert(key, MatchValue::U32(value));
        self
    }

    #[must_use]
    /// Adds an arbitrary string entry to the element-matching dictionary.
    pub fn with_string(mut self, key: &str, value: impl Into<String>) -> Self {
        self.insert(key, MatchValue::String(value.into()));
        self
    }

    #[must_use]
    /// Returns `true` when no entries will be passed to `IOHIDDeviceSetInputValueMatching`.
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
/// Wraps `IOHIDReportType`.
pub enum HidReportType {
    /// Mirrors `kIOHIDReportTypeInput`.
    Input,
    /// Mirrors `kIOHIDReportTypeOutput`.
    Output,
    /// Mirrors `kIOHIDReportTypeFeature`.
    Feature,
    /// Preserves an unrecognized raw `IOHIDReportType` value.
    Unknown(u32),
}

impl HidReportType {
    #[must_use]
    /// Returns the raw `IOHIDReportType` constant.
    pub const fn as_raw(self) -> u32 {
        match self {
            Self::Input => ffi::kIOHIDReportTypeInput,
            Self::Output => ffi::kIOHIDReportTypeOutput,
            Self::Feature => ffi::kIOHIDReportTypeFeature,
            Self::Unknown(raw) => raw,
        }
    }

    #[must_use]
    /// Wraps a raw `IOHIDReportType` constant.
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
/// Wraps the scale constants consumed by `IOHIDValueGetScaledValue`.
pub enum HidValueScale {
    /// Mirrors `kIOHIDValueScaleTypeCalibrated`.
    Calibrated,
    /// Mirrors `kIOHIDValueScaleTypePhysical`.
    Physical,
    /// Mirrors `kIOHIDValueScaleTypeExponent`.
    Exponent,
}

impl HidValueScale {
    #[must_use]
    /// Returns the raw scale constant consumed by `IOHIDValueGetScaledValue`.
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
/// Wraps the raw element-type values returned by `IOHIDElementGetType`.
pub enum HidElementType {
    /// Represents an input-misc element from `IOHIDElementGetType`.
    InputMisc,
    /// Represents an input-button element from `IOHIDElementGetType`.
    InputButton,
    /// Represents an input-axis element from `IOHIDElementGetType`.
    InputAxis,
    /// Represents an input-scan-code element from `IOHIDElementGetType`.
    InputScanCodes,
    /// Represents an input-null element from `IOHIDElementGetType`.
    InputNull,
    /// Represents an output element from `IOHIDElementGetType`.
    Output,
    /// Represents a feature element from `IOHIDElementGetType`.
    Feature,
    /// Represents a collection element from `IOHIDElementGetType`.
    Collection,
    /// Preserves an unrecognized raw element-type value.
    Unknown(i32),
}

impl HidElementType {
    #[must_use]
    /// Wraps the raw value returned by `IOHIDElementGetType`.
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
/// Wraps the raw collection-type values returned by `IOHIDElementGetCollectionType`.
pub enum HidCollectionType {
    /// Represents a physical collection from `IOHIDElementGetCollectionType`.
    Physical,
    /// Represents an application collection from `IOHIDElementGetCollectionType`.
    Application,
    /// Represents a logical collection from `IOHIDElementGetCollectionType`.
    Logical,
    /// Represents a report collection from `IOHIDElementGetCollectionType`.
    Report,
    /// Represents a named-array collection from `IOHIDElementGetCollectionType`.
    NamedArray,
    /// Represents a usage-switch collection from `IOHIDElementGetCollectionType`.
    UsageSwitch,
    /// Represents a usage-modifier collection from `IOHIDElementGetCollectionType`.
    UsageModifier,
    /// Preserves an unrecognized raw collection-type value.
    Unknown(i32),
}

impl HidCollectionType {
    #[must_use]
    /// Wraps the raw value returned by `IOHIDElementGetCollectionType`.
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
/// Carries one callback payload from `IOHIDDeviceRegisterInputReportCallback` or `IOHIDManagerRegisterInputReportCallback`.
pub struct HidInputReport {
    /// Report kind from `IOHIDReportType`.
    pub report_type: HidReportType,
    /// Report ID supplied by the callback.
    pub report_id: u32,
    /// Report bytes supplied by the callback.
    pub bytes: Vec<u8>,
    /// Timestamp from `IOHID*RegisterInputReportWithTimeStampCallback`, or `0` when unavailable.
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Collects common `IOHIDDeviceGetProperty` values.
pub struct HidDeviceInfo {
    /// Value read from `kIOHIDVendorIDKey`.
    pub vendor_id: Option<u32>,
    /// Value read from `kIOHIDProductIDKey`.
    pub product_id: Option<u32>,
    /// Value read from `kIOHIDProductKey`.
    pub product: Option<String>,
    /// Value read from `kIOHIDManufacturerKey`.
    pub manufacturer: Option<String>,
    /// Value read from `kIOHIDSerialNumberKey`.
    pub serial_number: Option<String>,
    /// Value read from `kIOHIDTransportKey`.
    pub transport: Option<String>,
    /// Value read from `kIOHIDPrimaryUsagePageKey` or `kIOHIDDeviceUsagePageKey`.
    pub usage_page: Option<u32>,
    /// Value read from `kIOHIDPrimaryUsageKey` or `kIOHIDDeviceUsageKey`.
    pub usage: Option<u32>,
    /// Value read from `kIOHIDLocationIDKey`.
    pub location_id: Option<u32>,
}

/// Owns an `IOHIDManagerRef` returned by `IOHIDManagerCreate`.
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
                ffi::CFRelease(self.raw);
            }
            self.raw = ptr::null();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidManager {
    #[must_use]
    /// Wraps `IOHIDManagerGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDManagerGetTypeID() }
    }

    /// Wraps `IOHIDManagerCreate` with `kIOHIDManagerOptionNone`.
    pub fn new() -> Result<Self, HidError> {
        Self::with_options(HidManagerOptions::NONE)
    }

    /// Wraps `IOHIDManagerSetDeviceMatching`.
    pub fn set_device_matching(&self, usage: Option<HidUsage>) -> Result<(), HidError> {
        usage.map_or_else(
            || {
                unsafe { ffi::IOHIDManagerSetDeviceMatching(self.raw, ptr::null()) };
                Ok(())
            },
            |usage| self.set_device_matching_dict(Some(&DeviceMatch::usage(usage))),
        )
    }

    /// Wraps `IOHIDManagerSetDeviceMatching` with a custom dictionary.
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

    /// Wraps `IOHIDManagerSetDeviceMatchingMultiple`.
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

    /// Wraps `IOHIDManagerSaveToPropertyDomain`.
    pub fn save_to_property_domain(
        &self,
        application_id: &str,
        user_name: &str,
        host_name: &str,
        options: IOHIDManagerOptions,
    ) -> Result<(), HidError> {
        let app = make_cfstring(application_id)?;
        let user = match make_cfstring(user_name) {
            Ok(user) => user,
            Err(err) => {
                unsafe { ffi::CFRelease(app.cast()) };
                return Err(err);
            }
        };
        let host = match make_cfstring(host_name) {
            Ok(host) => host,
            Err(err) => {
                unsafe {
                    ffi::CFRelease(app.cast());
                    ffi::CFRelease(user.cast());
                }
                return Err(err);
            }
        };
        unsafe {
            ffi::IOHIDManagerSaveToPropertyDomain(self.raw, app, user, host, options);
            ffi::CFRelease(app.cast());
            ffi::CFRelease(user.cast());
            ffi::CFRelease(host.cast());
        }
        Ok(())
    }

    #[must_use]
    /// Reads `IOHIDManagerGetProperty` as a `u32`.
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDManagerGetProperty` as a `String`.
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDManagerGetProperty` as `CFData` bytes.
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDManagerGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    /// Wraps `IOHIDManagerSetProperty` for numeric values.
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

    /// Wraps `IOHIDManagerSetProperty` for string values.
    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
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

    /// Wraps `IOHIDManagerSetProperty` for `CFData` bytes.
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
    /// Wraps `IOHIDManagerCopyDevices` and snapshots each device's properties.
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
            .map(|device| read_device_info(device.cast()))
            .collect();
        unsafe { ffi::CFRelease(set.cast()) };
        devices
    }

    #[must_use]
    /// Wraps `IOHIDManagerCopyDevices` and retains live `IOHIDDeviceRef` handles.
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
                HidDevice { raw: device.cast() }
            })
            .collect();
        unsafe { ffi::CFRelease(set.cast()) };
        devices
    }

    #[must_use]
    /// Mirrors `IOHIDManagerRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDManagerRef {
        self.raw
    }
}

/// Owns an `IOHIDDeviceRef` returned by `IOHIDManagerCopyDevices` or `IOHIDDeviceCreate`.
pub struct HidDevice {
    raw: ffi::IOHIDDeviceRef,
}

unsafe impl Send for HidDevice {}
unsafe impl Sync for HidDevice {}

impl Clone for HidDevice {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
        Self { raw: self.raw }
    }
}

impl Drop for HidDevice {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw) };
            self.raw = ptr::null();
        }
    }
}

#[allow(clippy::missing_errors_doc, clippy::type_complexity)]
impl HidDevice {
    #[must_use]
    /// Wraps `IOHIDDeviceGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDDeviceGetTypeID() }
    }

    #[must_use]
    /// Snapshots common `IOHIDDeviceGetProperty` values.
    pub fn info(&self) -> HidDeviceInfo {
        read_device_info(self.raw)
    }

    #[must_use]
    /// Wraps `IOHIDDeviceGetService`.
    pub fn service(&self) -> ffi::io_service_t {
        unsafe { ffi::IOHIDDeviceGetService(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDDeviceConformsTo`.
    pub fn conforms_to(&self, usage: HidUsage) -> bool {
        let (page, usage) = usage.as_pair();
        unsafe { ffi::IOHIDDeviceConformsTo(self.raw, page, usage) }
    }

    #[must_use]
    /// Reads `IOHIDDeviceGetProperty` as a `u32`.
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDDeviceGetProperty` as a `String`.
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDDeviceGetProperty` as `CFData` bytes.
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDDeviceGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads the `kIOHIDReportDescriptorKey` property.
    pub fn report_descriptor(&self) -> Option<Vec<u8>> {
        self.property_data(ffi::kIOHIDReportDescriptorKey)
    }

    /// Wraps `IOHIDDeviceSetProperty` for numeric values.
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

    /// Wraps `IOHIDDeviceSetProperty` for string values.
    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
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

    /// Wraps `IOHIDDeviceSetProperty` for `CFData` bytes.
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

    /// Wraps `IOHIDDeviceSetInputValueMatching`.
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

    /// Wraps `IOHIDDeviceSetInputValueMatchingMultiple`.
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

    /// Wraps `IOHIDDeviceRegisterInputReportCallback`.
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
            ffi::CFRetain(self.raw);
        }
        Ok(ReportSubscription {
            device: self.raw,
            run_loop,
            buffer_ptr,
            buffer_len: max_report_length,
            context: context_ptr,
        })
    }

    /// Wraps `IOHIDDeviceRegisterInputReportWithTimeStampCallback`.
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
            ffi::CFRetain(self.raw);
        }
        Ok(TimestampedReportSubscription {
            device: self.raw,
            run_loop,
            buffer_ptr,
            buffer_len: max_report_length,
            context: context_ptr,
        })
    }

    /// Wraps `IOHIDDeviceRegisterInputValueCallback`.
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
            ffi::CFRetain(self.raw);
        }
        Ok(ValueSubscription {
            device: self.raw,
            run_loop,
            context: context_ptr,
        })
    }

    #[must_use]
    /// Wraps `IOHIDDeviceCopyMatchingElements` with no filter.
    pub fn elements(&self) -> Vec<HidElement> {
        self.matching_elements(None)
    }

    #[must_use]
    /// Wraps `IOHIDDeviceCopyMatchingElements`.
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

    /// Wraps `IOHIDDeviceGetValue`.
    pub fn get_value(&self, element: &HidElement) -> Result<HidValue, HidError> {
        let mut value = ptr::null();
        let status = unsafe { ffi::IOHIDDeviceGetValue(self.raw, element.raw, &mut value) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceGetValue", status));
        }
        clone_value_ref(value).ok_or(HidError::OperationFailed("IOHIDDeviceGetValue"))
    }

    /// Wraps `IOHIDDeviceGetValueWithOptions`.
    pub fn get_value_with_options(
        &self,
        element: &HidElement,
        options: IOHIDDeviceGetValueOptions,
    ) -> Result<HidValue, HidError> {
        let mut value = ptr::null();
        let status = unsafe {
            ffi::IOHIDDeviceGetValueWithOptions(self.raw, element.raw, &mut value, options)
        };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceGetValueWithOptions", status));
        }
        clone_value_ref(value).ok_or(HidError::OperationFailed("IOHIDDeviceGetValueWithOptions"))
    }

    /// Wraps `IOHIDDeviceSetValue`.
    pub fn set_value(&self, element: &HidElement, value: &HidValue) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDDeviceSetValue(self.raw, element.raw, value.raw) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceSetValue", status))
        }
    }

    /// Wraps `IOHIDDeviceGetReport`.
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
        let report_id = ffi::CFIndex::from(report_id);
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

    /// Wraps `IOHIDDeviceSetReport`.
    pub fn set_report(
        &self,
        report_type: HidReportType,
        report_id: u32,
        report: &[u8],
    ) -> Result<(), HidError> {
        let report_id = ffi::CFIndex::from(report_id);
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
    /// Mirrors `IOHIDDeviceRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDDeviceRef {
        self.raw
    }

    /// Create a `HidDevice` from a raw ref that we do **not** already own;
    /// `CFRetain` is called so the resulting wrapper releases on drop.
    #[cfg(feature = "async")]
    pub(crate) fn from_raw_retained(raw: ffi::IOHIDDeviceRef) -> Self {
        debug_assert!(!raw.is_null());
        unsafe { ffi::CFRetain(raw) };
        Self { raw }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps an `IOHIDElementRef`.
pub struct HidElement {
    raw: ffi::IOHIDElementRef,
}

unsafe impl Send for HidElement {}
unsafe impl Sync for HidElement {}

#[allow(clippy::missing_errors_doc)]
impl HidElement {
    #[must_use]
    /// Wraps `IOHIDElementGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDElementGetTypeID() }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetType`.
    pub fn element_type(&self) -> i32 {
        unsafe { ffi::IOHIDElementGetType(self.raw) }
    }

    #[must_use]
    /// Classifies the raw value from `IOHIDElementGetType`.
    pub fn element_kind(&self) -> HidElementType {
        HidElementType::from_raw(self.element_type())
    }

    #[must_use]
    /// Wraps `IOHIDElementGetCollectionType` for collection elements.
    pub fn collection_type(&self) -> Option<HidCollectionType> {
        if self.element_kind() != HidElementType::Collection {
            return None;
        }
        Some(HidCollectionType::from_raw(unsafe {
            ffi::IOHIDElementGetCollectionType(self.raw)
        }))
    }

    #[must_use]
    /// Wraps `IOHIDElementGetUsage`.
    pub fn usage(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUsage(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetUsagePage`.
    pub fn usage_page(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUsagePage(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetCookie`.
    pub fn cookie(&self) -> IOHIDElementCookie {
        unsafe { ffi::IOHIDElementGetCookie(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementIsVirtual`.
    pub fn is_virtual(&self) -> bool {
        unsafe { ffi::IOHIDElementIsVirtual(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementIsRelative`.
    pub fn is_relative(&self) -> bool {
        unsafe { ffi::IOHIDElementIsRelative(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementIsWrapping`.
    pub fn is_wrapping(&self) -> bool {
        unsafe { ffi::IOHIDElementIsWrapping(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementIsArray`.
    pub fn is_array(&self) -> bool {
        unsafe { ffi::IOHIDElementIsArray(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementIsNonLinear`.
    pub fn is_non_linear(&self) -> bool {
        unsafe { ffi::IOHIDElementIsNonLinear(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementHasPreferredState`.
    pub fn has_preferred_state(&self) -> bool {
        unsafe { ffi::IOHIDElementHasPreferredState(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementHasNullState`.
    pub fn has_null_state(&self) -> bool {
        unsafe { ffi::IOHIDElementHasNullState(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetName`.
    pub fn name(&self) -> Option<String> {
        read_cf_string(unsafe { ffi::IOHIDElementGetName(self.raw).cast() })
    }

    #[must_use]
    /// Wraps `IOHIDElementGetReportID`.
    pub fn report_id(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportID(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetReportSize`.
    pub fn report_size_bits(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportSize(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetReportCount`.
    pub fn report_count(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetReportCount(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetUnit`.
    pub fn unit(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUnit(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetUnitExponent`.
    pub fn unit_exponent(&self) -> u32 {
        unsafe { ffi::IOHIDElementGetUnitExponent(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetLogicalMin`.
    pub fn logical_min(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetLogicalMin(self.raw) as i64 }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetLogicalMax`.
    pub fn logical_max(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetLogicalMax(self.raw) as i64 }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetPhysicalMin`.
    pub fn physical_min(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetPhysicalMin(self.raw) as i64 }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetPhysicalMax`.
    pub fn physical_max(&self) -> i64 {
        unsafe { ffi::IOHIDElementGetPhysicalMax(self.raw) as i64 }
    }

    #[must_use]
    /// Wraps `IOHIDElementGetParent`.
    pub fn parent(&self) -> Option<Self> {
        let parent = unsafe { ffi::IOHIDElementGetParent(self.raw) };
        (!parent.is_null()).then_some(Self { raw: parent })
    }

    #[must_use]
    /// Wraps `IOHIDElementGetChildren`.
    pub fn children(&self) -> Vec<Self> {
        elements_from_array(unsafe { ffi::IOHIDElementGetChildren(self.raw) }, false)
    }

    #[must_use]
    /// Wraps `IOHIDElementCopyAttached`.
    pub fn attached(&self) -> Vec<Self> {
        elements_from_array(unsafe { ffi::IOHIDElementCopyAttached(self.raw) }, true)
    }

    #[must_use]
    /// Wraps `IOHIDElementGetDevice`.
    pub fn device(&self) -> Option<HidDevice> {
        let device = unsafe { ffi::IOHIDElementGetDevice(self.raw) };
        if device.is_null() {
            None
        } else {
            unsafe { ffi::CFRetain(device) };
            Some(HidDevice { raw: device })
        }
    }

    #[must_use]
    /// Reads `IOHIDElementGetProperty` as a `u32`.
    pub fn property_u32(&self, key: &str) -> Option<u32> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_u32(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDElementGetProperty` as a `String`.
    pub fn property_string(&self, key: &str) -> Option<String> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_string(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    #[must_use]
    /// Reads `IOHIDElementGetProperty` as `CFData` bytes.
    pub fn property_data(&self, key: &str) -> Option<Vec<u8>> {
        with_cfstring(key, |key_cf| unsafe {
            read_cf_data(ffi::IOHIDElementGetProperty(self.raw, key_cf))
        })
        .ok()
        .flatten()
    }

    /// Wraps `IOHIDElementSetProperty` for numeric values.
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

    /// Wraps `IOHIDElementSetProperty` for string values.
    pub fn set_property_string(&self, key: &str, value: &str) -> Result<(), HidError> {
        let value_cf = make_cfstring(value)?;
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

    /// Wraps `IOHIDElementSetProperty` for `CFData` bytes.
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
    /// Mirrors `IOHIDElementRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDElementRef {
        self.raw
    }
}

/// Owns an `IOHIDValueRef`.
pub struct HidValue {
    raw: ffi::IOHIDValueRef,
}

unsafe impl Send for HidValue {}
unsafe impl Sync for HidValue {}

impl Clone for HidValue {
    fn clone(&self) -> Self {
        unsafe { ffi::CFRetain(self.raw) };
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
            unsafe { ffi::CFRelease(self.raw) };
            self.raw = ptr::null();
        }
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidValue {
    #[must_use]
    /// Wraps `IOHIDValueGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDValueGetTypeID() }
    }

    /// Wraps `IOHIDValueCreateWithIntegerValue`.
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

    /// Wraps `IOHIDValueCreateWithBytes`.
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
    /// Wraps `IOHIDValueGetElement`.
    pub fn element(&self) -> HidElement {
        HidElement {
            raw: unsafe { ffi::IOHIDValueGetElement(self.raw) },
        }
    }

    #[must_use]
    /// Wraps `IOHIDValueGetTimeStamp`.
    pub fn timestamp(&self) -> u64 {
        unsafe { ffi::IOHIDValueGetTimeStamp(self.raw) }
    }

    #[must_use]
    /// Wraps `IOHIDValueGetLength`.
    pub fn len(&self) -> usize {
        usize::try_from(unsafe { ffi::IOHIDValueGetLength(self.raw) }).unwrap_or(0)
    }

    #[must_use]
    /// Returns `true` when `IOHIDValueGetLength` is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    /// Wraps `IOHIDValueGetBytePtr`.
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
    /// Wraps `IOHIDValueGetIntegerValue`.
    pub fn integer_value(&self) -> i64 {
        unsafe { ffi::IOHIDValueGetIntegerValue(self.raw) as i64 }
    }

    #[must_use]
    /// Wraps `IOHIDValueGetScaledValue`.
    pub fn scaled_value(&self, scale: HidValueScale) -> f64 {
        unsafe { ffi::IOHIDValueGetScaledValue(self.raw, scale.as_raw()) }
    }

    #[must_use]
    /// Mirrors `IOHIDValueRef`.
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

/// Owns a registration from `IOHIDDeviceRegisterInputReportCallback`.
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
            ffi::CFRelease(self.device);
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
            let _ = Box::from_raw(self.buffer_ptr);
        }
        self.device = ptr::null();
        self.context = ptr::null_mut();
    }
}

/// Owns a registration from `IOHIDDeviceRegisterInputReportWithTimeStampCallback`.
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
            ffi::CFRelease(self.device);
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
            let _ = Box::from_raw(self.buffer_ptr);
        }
        self.device = ptr::null();
        self.context = ptr::null_mut();
    }
}

/// Owns a registration from `IOHIDDeviceRegisterInputValueCallback`.
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
            ffi::CFRelease(self.device);
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.device = ptr::null();
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

pub(crate) fn clone_value_ref(raw: ffi::IOHIDValueRef) -> Option<HidValue> {
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
                        ffi::CFRelease(key_cf.cast());
                        ffi::CFRelease(dict.cast());
                    }
                    return Err(err);
                }
            },
            MatchValue::String(value) => match make_cfstring(value) {
                Ok(value_cf) => value_cf.cast(),
                Err(err) => {
                    unsafe {
                        ffi::CFRelease(key_cf.cast());
                        ffi::CFRelease(dict.cast());
                    }
                    return Err(err);
                }
            },
        };
        unsafe {
            ffi::CFDictionarySetValue(dict, key_cf.cast(), value_cf.cast());
            ffi::CFRelease(key_cf.cast());
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
            elements.push(HidElement { raw: raw.cast() });
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
    unsafe { ffi::CFRelease(value_cf.cast()) };
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
