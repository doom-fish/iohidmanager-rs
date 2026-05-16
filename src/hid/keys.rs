use crate::bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyDefinition {
    pub symbol: &'static str,
    pub value: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NumericDefinition {
    pub symbol: &'static str,
    pub value: u64,
}

pub const DEVICE_KEY: &str = "IOHIDDevice";
pub const TRANSPORT_KEY: &str = "Transport";
pub const VENDOR_ID_KEY: &str = "VendorID";
pub const PRODUCT_ID_KEY: &str = "ProductID";
pub const PRODUCT_KEY: &str = "Product";
pub const MANUFACTURER_KEY: &str = "Manufacturer";
pub const SERIAL_NUMBER_KEY: &str = "SerialNumber";
pub const DEVICE_USAGE_PAIRS_KEY: &str = "DeviceUsagePairs";
pub const DEVICE_USAGE_PAGE_KEY: &str = "DeviceUsagePage";
pub const DEVICE_USAGE_KEY: &str = "DeviceUsage";
pub const PRIMARY_USAGE_PAGE_KEY: &str = "PrimaryUsagePage";
pub const PRIMARY_USAGE_KEY: &str = "PrimaryUsage";
pub const REPORT_DESCRIPTOR_KEY: &str = "ReportDescriptor";
pub const ELEMENT_USAGE_PAGE_KEY: &str = "UsagePage";
pub const ELEMENT_USAGE_KEY: &str = "Usage";
pub const ELEMENT_REPORT_ID_KEY: &str = "ReportID";
pub const ELEMENT_COOKIE_MIN_KEY: &str = "ElementCookieMin";
pub const ELEMENT_COOKIE_MAX_KEY: &str = "ElementCookieMax";
pub const ELEMENT_USAGE_MIN_KEY: &str = "UsageMin";
pub const ELEMENT_USAGE_MAX_KEY: &str = "UsageMax";
pub const TRANSPORT_USB_VALUE: &str = "USB";
pub const TRANSPORT_BLUETOOTH_VALUE: &str = "Bluetooth";
pub const DEVICE_SUSPEND_KEY: &str = "IOHIDDeviceSuspend";
pub const MAX_REPORT_BUFFER_COUNT_KEY: &str = "MaxReportBufferCount";
pub const REPORT_BUFFER_ENTRY_SIZE_KEY: &str = "ReportBufferEntrySize";
pub const KEYBOARD_LAYOUT_VALUE_KEY: &str = "HIDKeyboardLayoutValue";
pub const POINTER_ACCELERATION_ALGORITHM_KEY: &str = "HIDPointerAccelerationAlgorithm";
pub const SCROLL_ACCELERATION_ALGORITHM_KEY: &str = "HIDScrollAccelerationAlgorithm";

pub const OPTIONS_TYPE_NONE: u32 = 0x00;
pub const OPTIONS_TYPE_SEIZE_DEVICE: u32 = 0x01;
pub const OPTIONS_TYPE_MASK_PRIVATE: u32 = 0x00ff_0000;
pub const QUEUE_OPTIONS_TYPE_NONE: u32 = 0x00;
pub const QUEUE_OPTIONS_TYPE_ENQUEUE_ALL: u32 = 0x01;
pub const STANDARD_TYPE_ANSI: u32 = 0x0;
pub const STANDARD_TYPE_ISO: u32 = 0x1;
pub const STANDARD_TYPE_JIS: u32 = 0x2;
pub const STANDARD_TYPE_UNSPECIFIED: u32 = 0xffff_ffff;
pub const KEYBOARD_LAYOUT_TYPE_UNKNOWN: u32 = 0x0;
pub const KEYBOARD_LAYOUT_TYPE_101: u32 = 0x1;
pub const KEYBOARD_LAYOUT_TYPE_103: u32 = 0x2;
pub const KEYBOARD_LAYOUT_TYPE_102: u32 = 0x3;
pub const KEYBOARD_LAYOUT_TYPE_104: u32 = 0x4;
pub const KEYBOARD_LAYOUT_TYPE_106: u32 = 0x5;
pub const KEYBOARD_LAYOUT_TYPE_VENDOR: u32 = 0x6;
pub const ACCELERATION_ALGORITHM_TABLE: u64 = 0;
pub const ACCELERATION_ALGORITHM_PARAMETRIC: u64 = 1;
pub const ACCELERATION_ALGORITHM_DEFAULT: u64 = 2;

include!("generated_keys.rs");

#[must_use]
pub fn string_key(symbol: &str) -> Option<&'static str> {
    ALL_STRING_KEYS
        .iter()
        .find(|definition| definition.symbol == symbol)
        .map(|definition| definition.value)
}

#[must_use]
pub fn numeric_constant(symbol: &str) -> Option<u64> {
    ALL_NUMERIC_CONSTANTS
        .iter()
        .find(|definition| definition.symbol == symbol)
        .map(|definition| definition.value)
}

#[must_use]
pub fn bridge_queue_enqueue_all() -> u32 {
    unsafe { bridge::iohidmanager_swift_keys_queue_enqueue_all() }
}

#[must_use]
pub fn bridge_standard_type_ansi() -> u32 {
    unsafe { bridge::iohidmanager_swift_keys_standard_type_ansi() }
}
