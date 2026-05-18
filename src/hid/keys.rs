use crate::{bridge, ffi_impl as ffi};

/// Aliases `IOHIDAccelerationAlgorithmType`.
pub type IOHIDAccelerationAlgorithmType = ffi::IOHIDAccelerationAlgorithmType;
/// Aliases `IOHIDKeyboardPhysicalLayoutType`.
pub type IOHIDKeyboardPhysicalLayoutType = ffi::IOHIDKeyboardPhysicalLayoutType;
/// Aliases `IOHIDOptionsType`.
pub type IOHIDOptionsType = ffi::IOHIDOptionsType;
/// Aliases `IOHIDStandardType`.
pub type IOHIDStandardType = ffi::IOHIDStandardType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Describes one string constant mirrored from `IOHIDKeys.h`.
pub struct KeyDefinition {
    /// Header symbol name such as `kIOHIDVendorIDKey`.
    pub symbol: &'static str,
    /// String value exposed by the framework.
    pub value: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Describes one numeric constant mirrored from `IOHIDKeys.h`.
pub struct NumericDefinition {
    /// Header symbol name such as `kIOHIDOptionsTypeNone`.
    pub symbol: &'static str,
    /// Raw numeric value exposed by the framework.
    pub value: u64,
}

/// Exposes the `IOHIDDevice` key mirrored from `IOHIDKeys.h`.
pub const DEVICE_KEY: &str = "IOHIDDevice";
/// Exposes the `Transport` key mirrored from `IOHIDKeys.h`.
pub const TRANSPORT_KEY: &str = "Transport";
/// Exposes the `VendorID` key mirrored from `IOHIDKeys.h`.
pub const VENDOR_ID_KEY: &str = "VendorID";
/// Exposes the `ProductID` key mirrored from `IOHIDKeys.h`.
pub const PRODUCT_ID_KEY: &str = "ProductID";
/// Exposes the `Product` key mirrored from `IOHIDKeys.h`.
pub const PRODUCT_KEY: &str = "Product";
/// Exposes the `Manufacturer` key mirrored from `IOHIDKeys.h`.
pub const MANUFACTURER_KEY: &str = "Manufacturer";
/// Exposes the `SerialNumber` key mirrored from `IOHIDKeys.h`.
pub const SERIAL_NUMBER_KEY: &str = "SerialNumber";
/// Exposes the `DeviceUsagePairs` key mirrored from `IOHIDKeys.h`.
pub const DEVICE_USAGE_PAIRS_KEY: &str = "DeviceUsagePairs";
/// Exposes the `DeviceUsagePage` key mirrored from `IOHIDKeys.h`.
pub const DEVICE_USAGE_PAGE_KEY: &str = "DeviceUsagePage";
/// Exposes the `DeviceUsage` key mirrored from `IOHIDKeys.h`.
pub const DEVICE_USAGE_KEY: &str = "DeviceUsage";
/// Exposes the `PrimaryUsagePage` key mirrored from `IOHIDKeys.h`.
pub const PRIMARY_USAGE_PAGE_KEY: &str = "PrimaryUsagePage";
/// Exposes the `PrimaryUsage` key mirrored from `IOHIDKeys.h`.
pub const PRIMARY_USAGE_KEY: &str = "PrimaryUsage";
/// Exposes the `ReportDescriptor` key mirrored from `IOHIDKeys.h`.
pub const REPORT_DESCRIPTOR_KEY: &str = "ReportDescriptor";
/// Exposes the `UsagePage` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_USAGE_PAGE_KEY: &str = "UsagePage";
/// Exposes the `Usage` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_USAGE_KEY: &str = "Usage";
/// Exposes the `ReportID` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_REPORT_ID_KEY: &str = "ReportID";
/// Exposes the `ElementCookieMin` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_COOKIE_MIN_KEY: &str = "ElementCookieMin";
/// Exposes the `ElementCookieMax` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_COOKIE_MAX_KEY: &str = "ElementCookieMax";
/// Exposes the `UsageMin` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_USAGE_MIN_KEY: &str = "UsageMin";
/// Exposes the `UsageMax` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_USAGE_MAX_KEY: &str = "UsageMax";
/// Exposes the `VendorSpecific` key mirrored from `IOHIDKeys.h`.
pub const ELEMENT_VENDOR_SPECIFIC_KEY: &str = "VendorSpecific";
/// Exposes the `USB` value mirrored from `IOHIDKeys.h`.
pub const TRANSPORT_USB_VALUE: &str = "USB";
/// Exposes the `Bluetooth` value mirrored from `IOHIDKeys.h`.
pub const TRANSPORT_BLUETOOTH_VALUE: &str = "Bluetooth";
/// Exposes the `IOHIDDeviceSuspend` key mirrored from `IOHIDKeys.h`.
pub const DEVICE_SUSPEND_KEY: &str = "IOHIDDeviceSuspend";
/// Exposes the `MaxReportBufferCount` key mirrored from `IOHIDKeys.h`.
pub const MAX_REPORT_BUFFER_COUNT_KEY: &str = "MaxReportBufferCount";
/// Exposes the `ReportBufferEntrySize` key mirrored from `IOHIDKeys.h`.
pub const REPORT_BUFFER_ENTRY_SIZE_KEY: &str = "ReportBufferEntrySize";
/// Exposes the `HIDKeyboardLayoutValue` key mirrored from `IOHIDKeys.h`.
pub const KEYBOARD_LAYOUT_VALUE_KEY: &str = "HIDKeyboardLayoutValue";
/// Exposes the `HIDPointerAccelerationAlgorithm` key mirrored from `IOHIDKeys.h`.
pub const POINTER_ACCELERATION_ALGORITHM_KEY: &str = "HIDPointerAccelerationAlgorithm";
/// Exposes the `HIDScrollAccelerationAlgorithm` key mirrored from `IOHIDKeys.h`.
pub const SCROLL_ACCELERATION_ALGORITHM_KEY: &str = "HIDScrollAccelerationAlgorithm";
/// Mirrors `kIOHIDSystemButtonPressedDuringDarkBoot`.
pub const SYSTEM_BUTTON_PRESSED_DURING_DARK_BOOT: u64 = 3_758_325_767;

/// Mirrors `kIOHIDOptionsTypeNone`.
pub const OPTIONS_TYPE_NONE: IOHIDOptionsType = ffi::kIOHIDOptionsTypeNone;
/// Mirrors `kIOHIDOptionsTypeSeizeDevice`.
pub const OPTIONS_TYPE_SEIZE_DEVICE: IOHIDOptionsType = ffi::kIOHIDOptionsTypeSeizeDevice;
/// Mirrors `kIOHIDOptionsTypeMaskPrivate`.
pub const OPTIONS_TYPE_MASK_PRIVATE: IOHIDOptionsType = 0x00ff_0000;
/// Mirrors `kIOHIDQueueOptionsTypeNone`.
pub const QUEUE_OPTIONS_TYPE_NONE: IOHIDOptionsType = ffi::kIOHIDQueueOptionsTypeNone;
/// Mirrors `kIOHIDQueueOptionsTypeEnqueueAll`.
pub const QUEUE_OPTIONS_TYPE_ENQUEUE_ALL: IOHIDOptionsType = ffi::kIOHIDQueueOptionsTypeEnqueueAll;
/// Mirrors `kIOHIDStandardTypeANSI`.
pub const STANDARD_TYPE_ANSI: IOHIDStandardType = 0x0;
/// Mirrors `kIOHIDStandardTypeISO`.
pub const STANDARD_TYPE_ISO: IOHIDStandardType = 0x1;
/// Mirrors `kIOHIDStandardTypeJIS`.
pub const STANDARD_TYPE_JIS: IOHIDStandardType = 0x2;
/// Mirrors `kIOHIDStandardTypeUnspecified`.
pub const STANDARD_TYPE_UNSPECIFIED: IOHIDStandardType = 0xffff_ffff;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutTypeUnknown`.
pub const KEYBOARD_LAYOUT_TYPE_UNKNOWN: IOHIDKeyboardPhysicalLayoutType = 0x0;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutType101`.
pub const KEYBOARD_LAYOUT_TYPE_101: IOHIDKeyboardPhysicalLayoutType = 0x1;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutType103`.
pub const KEYBOARD_LAYOUT_TYPE_103: IOHIDKeyboardPhysicalLayoutType = 0x2;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutType102`.
pub const KEYBOARD_LAYOUT_TYPE_102: IOHIDKeyboardPhysicalLayoutType = 0x3;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutType104`.
pub const KEYBOARD_LAYOUT_TYPE_104: IOHIDKeyboardPhysicalLayoutType = 0x4;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutType106`.
pub const KEYBOARD_LAYOUT_TYPE_106: IOHIDKeyboardPhysicalLayoutType = 0x5;
/// Mirrors `kIOHIDKeyboardPhysicalLayoutTypeVendor`.
pub const KEYBOARD_LAYOUT_TYPE_VENDOR: IOHIDKeyboardPhysicalLayoutType = 0x6;
/// Mirrors `kIOHIDAccelerationAlgorithmTypeTable`.
pub const ACCELERATION_ALGORITHM_TABLE: IOHIDAccelerationAlgorithmType = 0;
/// Mirrors `kIOHIDAccelerationAlgorithmTypeParametric`.
pub const ACCELERATION_ALGORITHM_PARAMETRIC: IOHIDAccelerationAlgorithmType = 1;
/// Mirrors `kIOHIDAccelerationAlgorithmTypeDefault`.
pub const ACCELERATION_ALGORITHM_DEFAULT: IOHIDAccelerationAlgorithmType = 2;

include!("generated_keys.rs");

#[must_use]
/// Looks up a string constant mirrored from `IOHIDKeys.h` by symbol name.
pub fn string_key(symbol: &str) -> Option<&'static str> {
    ALL_STRING_KEYS
        .iter()
        .find(|definition| definition.symbol == symbol)
        .map(|definition| definition.value)
}

#[must_use]
/// Looks up a numeric constant mirrored from `IOHIDKeys.h` by symbol name.
pub fn numeric_constant(symbol: &str) -> Option<u64> {
    ALL_NUMERIC_CONSTANTS
        .iter()
        .find(|definition| definition.symbol == symbol)
        .map(|definition| definition.value)
}

#[must_use]
/// Returns the Swift-bridge mirror of `kIOHIDQueueOptionsTypeEnqueueAll`.
pub fn bridge_queue_enqueue_all() -> u32 {
    unsafe { bridge::iohidmanager_swift_keys_queue_enqueue_all() }
}

#[must_use]
/// Returns the Swift-bridge mirror of `kIOHIDStandardTypeANSI`.
pub fn bridge_standard_type_ansi() -> u32 {
    unsafe { bridge::iohidmanager_swift_keys_standard_type_ansi() }
}
