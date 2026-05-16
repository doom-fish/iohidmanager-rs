use crate::{bridge, ffi_impl as ffi};

pub type IOHIDKeyboardEventOptions = ffi::IOHIDKeyboardEventOptions;
pub type IOHIDPointerEventOptions = ffi::IOHIDPointerEventOptions;
pub type IOHIDScrollEventOptions = ffi::IOHIDScrollEventOptions;
pub type IOHIDServiceSensorControlOptions = ffi::IOHIDServiceSensorControlOptions;

pub const POINTER_ACCELERATION_KEY: &str = "HIDPointerAcceleration";
pub const TRACKPAD_SCROLL_ACCELERATION_KEY: &str = "HIDTrackpadScrollAcceleration";
pub const TRACKPAD_ACCELERATION_TYPE_KEY: &str = "HIDTrackpadAcceleration";
pub const POINTER_ACCELERATION_TYPE_KEY: &str = "HIDPointerAccelerationType";
pub const ACCEL_PARAMETRIC_CURVES_KEY: &str = "HIDAccelCurves";
pub const MOUSE_SCROLL_ACCELERATION_KEY: &str = "HIDMouseScrollAcceleration";
pub const MOUSE_ACCELERATION_TYPE_KEY: &str = "HIDMouseAcceleration";
pub const SCROLL_ACCELERATION_KEY: &str = "HIDScrollAcceleration";
pub const SCROLL_ACCELERATION_TYPE_KEY: &str = "HIDScrollAccelerationType";
pub const SCROLL_ACCEL_PARAMETRIC_CURVES_KEY: &str = "HIDScrollAccelCurves";
pub const SCROLL_RESOLUTION_KEY: &str = "HIDScrollResolution";
pub const DROP_ACCEL_PROPERTY_EVENTS_KEY: &str = "DropAccelPropertyEvents";
pub const SCROLL_RESOLUTION_X_KEY: &str = "HIDScrollResolutionX";
pub const SCROLL_RESOLUTION_Y_KEY: &str = "HIDScrollResolutionY";
pub const SCROLL_RESOLUTION_Z_KEY: &str = "HIDScrollResolutionZ";
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
pub const USE_LINEAR_SCALING_MOUSE_ACCELERATION_KEY: &str = "HIDUseLinearScalingMouseAcceleration";
pub const SENSOR_CONTROL_OPTIONS_KEY: &str = "HIDDefaultSensorControlOptions";

pub const KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT: IOHIDKeyboardEventOptions =
    ffi::kIOHIDKeyboardEventOptionsNoKeyRepeat;
pub const POINTER_EVENT_OPTIONS_NO_ACCELERATION: IOHIDPointerEventOptions =
    ffi::kIOHIDPointerEventOptionsNoAcceleration;
pub const SCROLL_EVENT_OPTIONS_NO_ACCELERATION: IOHIDScrollEventOptions =
    ffi::kIOHIDScrollEventOptionsNoAcceleration;
pub const SENSOR_CONTROL_DECIMATION: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlDecimation;
pub const SENSOR_CONTROL_AGGREGATION: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlAggregation;
pub const SENSOR_CONTROL_DISPATCH_CONTROL: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlDispatchControl;

pub const ALL_EVENT_SYSTEM_KEYS: &[(&str, &str)] = &[
    ("kIOHIDPointerAccelerationKey", POINTER_ACCELERATION_KEY),
    (
        "kIOHIDTrackpadScrollAccelerationKey",
        TRACKPAD_SCROLL_ACCELERATION_KEY,
    ),
    (
        "kIOHIDTrackpadAccelerationType",
        TRACKPAD_ACCELERATION_TYPE_KEY,
    ),
    (
        "kIOHIDPointerAccelerationTypeKey",
        POINTER_ACCELERATION_TYPE_KEY,
    ),
    ("kHIDAccelParametricCurvesKey", ACCEL_PARAMETRIC_CURVES_KEY),
    (
        "kIOHIDMouseScrollAccelerationKey",
        MOUSE_SCROLL_ACCELERATION_KEY,
    ),
    (
        "kIOHIDMouseAccelerationTypeKey",
        MOUSE_ACCELERATION_TYPE_KEY,
    ),
    ("kIOHIDScrollAccelerationKey", SCROLL_ACCELERATION_KEY),
    (
        "kIOHIDScrollAccelerationTypeKey",
        SCROLL_ACCELERATION_TYPE_KEY,
    ),
    (
        "kHIDScrollAccelParametricCurvesKey",
        SCROLL_ACCEL_PARAMETRIC_CURVES_KEY,
    ),
    ("kIOHIDScrollResolutionKey", SCROLL_RESOLUTION_KEY),
    (
        "kIOHIDDropAccelPropertyEventsKey",
        DROP_ACCEL_PROPERTY_EVENTS_KEY,
    ),
    ("kIOHIDScrollResolutionXKey", SCROLL_RESOLUTION_X_KEY),
    ("kIOHIDScrollResolutionYKey", SCROLL_RESOLUTION_Y_KEY),
    ("kIOHIDScrollResolutionZKey", SCROLL_RESOLUTION_Z_KEY),
    (
        "kIOHIDDigitizerTipThresholdKey",
        DIGITIZER_TIP_THRESHOLD_KEY,
    ),
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
        "kIOHIDUseLinearScalingMouseAccelerationKey",
        USE_LINEAR_SCALING_MOUSE_ACCELERATION_KEY,
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
