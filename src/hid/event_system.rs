use crate::bridge;

pub const POINTER_ACCELERATION_KEY: &str = "HIDPointerAcceleration";
pub const TRACKPAD_SCROLL_ACCELERATION_KEY: &str = "HIDTrackpadScrollAcceleration";
pub const TRACKPAD_ACCELERATION_TYPE_KEY: &str = "HIDTrackpadAcceleration";
pub const POINTER_ACCELERATION_TYPE_KEY: &str = "HIDPointerAccelerationType";
pub const MOUSE_SCROLL_ACCELERATION_KEY: &str = "HIDMouseScrollAcceleration";
pub const MOUSE_ACCELERATION_TYPE_KEY: &str = "HIDMouseAcceleration";
pub const SCROLL_ACCELERATION_KEY: &str = "HIDScrollAcceleration";
pub const SCROLL_ACCELERATION_TYPE_KEY: &str = "HIDScrollAccelerationType";
pub const DIGITIZER_TIP_THRESHOLD_KEY: &str = "DigitizerTipThreshold";
pub const SURFACE_DIMENSIONS_KEY: &str = "SurfaceDimensions";
pub const WIDTH_KEY: &str = "Width";
pub const HEIGHT_KEY: &str = "Height";
pub const EVENT_DRIVER_HANDLES_REPORT_KEY: &str = "IOHIDEventDriverHandlesReport";
pub const SERVICE_ACCELERATION_PROPERTIES_KEY: &str = "IOHIDSetAcceleration";
pub const POINTER_ACCELERATION_MULTIPLIER_KEY: &str = "HIDPointerAccelerationMultiplier";
pub const POINTER_REPORT_RATE_KEY: &str = "HIDPointerReportRate";
pub const SCROLL_REPORT_RATE_KEY: &str = "HIDScrollReportRate";
pub const POINTER_ACCELERATION_SUPPORT_KEY: &str = "HIDSupportsPointerAcceleration";
pub const SCROLL_ACCELERATION_SUPPORT_KEY: &str = "HIDSupportsScrollAcceleration";
pub const SENSOR_CONTROL_OPTIONS_KEY: &str = "HIDDefaultSensorControlOptions";

pub const KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT: u32 = 1 << 8;
pub const POINTER_EVENT_OPTIONS_NO_ACCELERATION: u32 = 1 << 8;
pub const SCROLL_EVENT_OPTIONS_NO_ACCELERATION: u32 = 1 << 8;
pub const SENSOR_CONTROL_DECIMATION: u32 = 1 << 0;
pub const SENSOR_CONTROL_AGGREGATION: u32 = 1 << 1;
pub const SENSOR_CONTROL_DISPATCH_CONTROL: u32 = 1 << 2;

pub const ALL_EVENT_SYSTEM_KEYS: &[(&str, &str)] = &[
    ("kIOHIDPointerAccelerationKey", POINTER_ACCELERATION_KEY),
    (
        "kIOHIDTrackpadScrollAccelerationKey",
        TRACKPAD_SCROLL_ACCELERATION_KEY,
    ),
    ("kIOHIDTrackpadAccelerationType", TRACKPAD_ACCELERATION_TYPE_KEY),
    (
        "kIOHIDPointerAccelerationTypeKey",
        POINTER_ACCELERATION_TYPE_KEY,
    ),
    ("kIOHIDMouseScrollAccelerationKey", MOUSE_SCROLL_ACCELERATION_KEY),
    ("kIOHIDMouseAccelerationTypeKey", MOUSE_ACCELERATION_TYPE_KEY),
    ("kIOHIDScrollAccelerationKey", SCROLL_ACCELERATION_KEY),
    (
        "kIOHIDScrollAccelerationTypeKey",
        SCROLL_ACCELERATION_TYPE_KEY,
    ),
    ("kIOHIDDigitizerTipThresholdKey", DIGITIZER_TIP_THRESHOLD_KEY),
    ("kIOHIDSurfaceDimensionsKey", SURFACE_DIMENSIONS_KEY),
    ("kIOHIDWidthKey", WIDTH_KEY),
    ("kIOHIDHeightKey", HEIGHT_KEY),
    (
        "kIOHIDEventDriverHandlesReport",
        EVENT_DRIVER_HANDLES_REPORT_KEY,
    ),
    (
        "kIOHIDServiceAccelerationProperties",
        SERVICE_ACCELERATION_PROPERTIES_KEY,
    ),
    (
        "kIOHIDPointerAccelerationMultiplierKey",
        POINTER_ACCELERATION_MULTIPLIER_KEY,
    ),
    ("kHIDPointerReportRateKey", POINTER_REPORT_RATE_KEY),
    ("kIOHIDScrollReportRateKey", SCROLL_REPORT_RATE_KEY),
    (
        "kIOHIDPointerAccelerationSupportKey",
        POINTER_ACCELERATION_SUPPORT_KEY,
    ),
    (
        "kIOHIDScrollAccelerationSupportKey",
        SCROLL_ACCELERATION_SUPPORT_KEY,
    ),
    (
        "kIOHIDEventServiceSensorControlOptionsKey",
        SENSOR_CONTROL_OPTIONS_KEY,
    ),
];

#[must_use]
pub fn bridge_keyboard_no_key_repeat() -> u32 {
    unsafe { bridge::iohidmanager_swift_event_keyboard_no_repeat() }
}

#[must_use]
pub fn bridge_sensor_control_decimation() -> u32 {
    unsafe { bridge::iohidmanager_swift_event_sensor_control_decimation() }
}
