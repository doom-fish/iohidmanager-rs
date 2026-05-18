use crate::{bridge, ffi_impl as ffi};

/// Aliases `IOHIDKeyboardEventOptions`.
pub type IOHIDKeyboardEventOptions = ffi::IOHIDKeyboardEventOptions;
/// Aliases `IOHIDPointerEventOptions`.
pub type IOHIDPointerEventOptions = ffi::IOHIDPointerEventOptions;
/// Aliases `IOHIDScrollEventOptions`.
pub type IOHIDScrollEventOptions = ffi::IOHIDScrollEventOptions;
/// Aliases `IOHIDServiceSensorControlOptions`.
pub type IOHIDServiceSensorControlOptions = ffi::IOHIDServiceSensorControlOptions;

/// Exposes the `HIDPointerAcceleration` event-system key.
pub const POINTER_ACCELERATION_KEY: &str = "HIDPointerAcceleration";
/// Exposes the `HIDTrackpadScrollAcceleration` event-system key.
pub const TRACKPAD_SCROLL_ACCELERATION_KEY: &str = "HIDTrackpadScrollAcceleration";
/// Exposes the `HIDTrackpadAcceleration` event-system key.
pub const TRACKPAD_ACCELERATION_TYPE_KEY: &str = "HIDTrackpadAcceleration";
/// Exposes the `HIDPointerAccelerationType` event-system key.
pub const POINTER_ACCELERATION_TYPE_KEY: &str = "HIDPointerAccelerationType";
/// Exposes the `HIDAccelCurves` event-system key.
pub const ACCEL_PARAMETRIC_CURVES_KEY: &str = "HIDAccelCurves";
/// Exposes the `HIDMouseScrollAcceleration` event-system key.
pub const MOUSE_SCROLL_ACCELERATION_KEY: &str = "HIDMouseScrollAcceleration";
/// Exposes the `HIDMouseAcceleration` event-system key.
pub const MOUSE_ACCELERATION_TYPE_KEY: &str = "HIDMouseAcceleration";
/// Exposes the `HIDScrollAcceleration` event-system key.
pub const SCROLL_ACCELERATION_KEY: &str = "HIDScrollAcceleration";
/// Exposes the `HIDScrollAccelerationType` event-system key.
pub const SCROLL_ACCELERATION_TYPE_KEY: &str = "HIDScrollAccelerationType";
/// Exposes the `HIDScrollAccelCurves` event-system key.
pub const SCROLL_ACCEL_PARAMETRIC_CURVES_KEY: &str = "HIDScrollAccelCurves";
/// Exposes the `HIDScrollResolution` event-system key.
pub const SCROLL_RESOLUTION_KEY: &str = "HIDScrollResolution";
/// Exposes the `DropAccelPropertyEvents` event-system key.
pub const DROP_ACCEL_PROPERTY_EVENTS_KEY: &str = "DropAccelPropertyEvents";
/// Exposes the `HIDScrollResolutionX` event-system key.
pub const SCROLL_RESOLUTION_X_KEY: &str = "HIDScrollResolutionX";
/// Exposes the `HIDScrollResolutionY` event-system key.
pub const SCROLL_RESOLUTION_Y_KEY: &str = "HIDScrollResolutionY";
/// Exposes the `HIDScrollResolutionZ` event-system key.
pub const SCROLL_RESOLUTION_Z_KEY: &str = "HIDScrollResolutionZ";
/// Exposes the `DigitizerTipThreshold` event-system key.
pub const DIGITIZER_TIP_THRESHOLD_KEY: &str = "DigitizerTipThreshold";
/// Exposes the `SurfaceDimensions` event-system key.
pub const SURFACE_DIMENSIONS_KEY: &str = "SurfaceDimensions";
/// Exposes the `Width` event-system key.
pub const WIDTH_KEY: &str = "Width";
/// Exposes the `Height` event-system key.
pub const HEIGHT_KEY: &str = "Height";
/// Exposes the `IOHIDEventDriverHandlesReport` event-system key.
pub const EVENT_DRIVER_HANDLES_REPORT_KEY: &str = "IOHIDEventDriverHandlesReport";
/// Exposes the `IOHIDSetAcceleration` event-system key.
pub const SERVICE_ACCELERATION_PROPERTIES_KEY: &str = "IOHIDSetAcceleration";
/// Exposes the `HIDPointerAccelerationMultiplier` event-system key.
pub const POINTER_ACCELERATION_MULTIPLIER_KEY: &str = "HIDPointerAccelerationMultiplier";
/// Exposes the `HIDPointerReportRate` event-system key.
pub const POINTER_REPORT_RATE_KEY: &str = "HIDPointerReportRate";
/// Exposes the `HIDScrollReportRate` event-system key.
pub const SCROLL_REPORT_RATE_KEY: &str = "HIDScrollReportRate";
/// Exposes the `HIDSupportsPointerAcceleration` event-system key.
pub const POINTER_ACCELERATION_SUPPORT_KEY: &str = "HIDSupportsPointerAcceleration";
/// Exposes the `HIDSupportsScrollAcceleration` event-system key.
pub const SCROLL_ACCELERATION_SUPPORT_KEY: &str = "HIDSupportsScrollAcceleration";
/// Exposes the `HIDUseLinearScalingMouseAcceleration` event-system key.
pub const USE_LINEAR_SCALING_MOUSE_ACCELERATION_KEY: &str = "HIDUseLinearScalingMouseAcceleration";
/// Exposes the `HIDDefaultSensorControlOptions` event-system key.
pub const SENSOR_CONTROL_OPTIONS_KEY: &str = "HIDDefaultSensorControlOptions";

/// Mirrors `kIOHIDKeyboardEventOptionsNoKeyRepeat`.
pub const KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT: IOHIDKeyboardEventOptions =
    ffi::kIOHIDKeyboardEventOptionsNoKeyRepeat;
/// Mirrors `kIOHIDPointerEventOptionsNoAcceleration`.
pub const POINTER_EVENT_OPTIONS_NO_ACCELERATION: IOHIDPointerEventOptions =
    ffi::kIOHIDPointerEventOptionsNoAcceleration;
/// Mirrors `kIOHIDScrollEventOptionsNoAcceleration`.
pub const SCROLL_EVENT_OPTIONS_NO_ACCELERATION: IOHIDScrollEventOptions =
    ffi::kIOHIDScrollEventOptionsNoAcceleration;
/// Mirrors `kIOHIDServiceSensorControlDecimation`.
pub const SENSOR_CONTROL_DECIMATION: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlDecimation;
/// Mirrors `kIOHIDServiceSensorControlAggregation`.
pub const SENSOR_CONTROL_AGGREGATION: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlAggregation;
/// Mirrors `kIOHIDServiceSensorControlDispatchControl`.
pub const SENSOR_CONTROL_DISPATCH_CONTROL: IOHIDServiceSensorControlOptions =
    ffi::kIOHIDServiceSensorControlDispatchControl;

/// Collects event-system keys mirrored from `IOHIDEventSystemKeys.h`.
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
/// Returns the Swift-bridge mirror of `kIOHIDKeyboardEventOptionsNoKeyRepeat`.
pub fn bridge_keyboard_no_key_repeat() -> u32 {
    unsafe { bridge::iohidmanager_swift_event_keyboard_no_repeat() }
}

#[must_use]
/// Returns the Swift-bridge mirror of `kIOHIDServiceSensorControlDecimation`.
pub fn bridge_sensor_control_decimation() -> u32 {
    unsafe { bridge::iohidmanager_swift_event_sensor_control_decimation() }
}
