use core::ptr;
use std::mem::{size_of, size_of_val};

use iohidmanager::{event_system, hid, keys, service_plugin, usage, HidManager, HidManagerOptions};

#[test]
fn manager_options_are_usable() {
    let _: hid::IOHIDManagerOptions = HidManagerOptions::INDEPENDENT_DEVICES.bits();
    let options =
        HidManagerOptions::USE_PERSISTENT_PROPERTIES | HidManagerOptions::DO_NOT_SAVE_PROPERTIES;
    assert_eq!(options.bits(), 0x5);

    let manager = HidManager::with_options(HidManagerOptions::NONE).expect("manager");
    manager.set_device_matching(None).expect("matching");
}

#[test]
fn device_type_aliases_and_constants_are_exposed() {
    let _: hid::IOHIDDeviceGetValueOptions = hid::DEVICE_GET_VALUE_WITH_UPDATE;
    let _: hid::IOHIDElementCommitDirection = hid::ELEMENT_COMMIT_DIRECTION_IN;
    let _: hid::IOHIDElementFlags = hid::ELEMENT_FLAGS_BUFFERED_BYTE_MASK;
    let _: hid::IOHIDValueOptions = hid::VALUE_OPTIONS_UPDATE_ELEMENT_VALUES;
    let _: hid::HIDReportCommandType = hid::REPORT_COMMAND_GET_REPORT;
    let _: Option<hid::IOHIDCompletionAction> = None;

    let completion = hid::IOHIDCompletion {
        target: ptr::null_mut(),
        action: None,
        parameter: ptr::null_mut(),
    };

    assert_eq!(hid::DEVICE_GET_VALUE_WITHOUT_UPDATE, 0x0004_0000);
    assert_eq!(hid::ELEMENT_FLAGS_BUFFERED_BYTE_MASK, 0x0100);
    assert_eq!(hid::REPORT_COMMAND_SET_REPORT, 0);
    assert_eq!(hid::REPORT_COMMAND_GET_REPORT, 1);
    assert_eq!(hid::DEVICE_DEFAULT_ASYNC_REQUEST_TIMEOUT, 1_000);
    assert_eq!(size_of::<hid::IOHIDCompletion>(), size_of_val(&completion));
}

#[test]
fn event_system_keys_and_option_types_are_exposed() {
    let _: event_system::IOHIDKeyboardEventOptions =
        event_system::KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT;
    let _: event_system::IOHIDPointerEventOptions =
        event_system::POINTER_EVENT_OPTIONS_NO_ACCELERATION;
    let _: event_system::IOHIDScrollEventOptions =
        event_system::SCROLL_EVENT_OPTIONS_NO_ACCELERATION;
    let _: event_system::IOHIDServiceSensorControlOptions = event_system::SENSOR_CONTROL_DECIMATION;

    assert_eq!(
        event_system::ALL_EVENT_SYSTEM_KEYS
            .iter()
            .find(|(symbol, _)| *symbol == "kHIDAccelParametricCurvesKey")
            .map(|(_, value)| *value),
        Some(event_system::ACCEL_PARAMETRIC_CURVES_KEY),
    );
    assert_eq!(
        event_system::ALL_EVENT_SYSTEM_KEYS
            .iter()
            .find(|(symbol, _)| *symbol == "kIOHIDUseLinearScalingMouseAccelerationKey")
            .map(|(_, value)| *value),
        Some(event_system::USE_LINEAR_SCALING_MOUSE_ACCELERATION_KEY),
    );
}

#[test]
fn key_usage_and_plugin_surfaces_are_exposed() {
    let _: keys::IOHIDAccelerationAlgorithmType = keys::ACCELERATION_ALGORITHM_DEFAULT;
    let _: keys::IOHIDKeyboardPhysicalLayoutType = keys::KEYBOARD_LAYOUT_TYPE_101;
    let _: keys::IOHIDOptionsType = keys::OPTIONS_TYPE_SEIZE_DEVICE;
    let _: keys::IOHIDStandardType = keys::STANDARD_TYPE_ANSI;

    assert_eq!(
        keys::string_key("kIOHIDElementVendorSpecificKey"),
        Some(keys::ELEMENT_VENDOR_SPECIFIC_KEY),
    );
    assert_eq!(
        keys::numeric_constant("kIOHIDSystemButtonPressedDuringDarkBoot"),
        Some(keys::SYSTEM_BUTTON_PRESSED_DURING_DARK_BOOT),
    );

    let system_menu = usage::constant("kHIDUsage_GD_SystemMenu").expect("system menu");
    let system_menu_select =
        usage::constant("kHIDUsage_GD_SystemMenuSelect").expect("system menu select");
    assert_eq!(system_menu.value, usage::USAGE_SYSTEM_MENU);
    assert_eq!(system_menu.value, system_menu_select.value);

    assert_eq!(size_of::<service_plugin::CFUUIDBytes>(), 16);
    assert_ne!(size_of::<service_plugin::IUnknownVTable>(), 0);
    assert_ne!(size_of::<service_plugin::IOHIDDeviceDeviceInterface>(), 0);
    assert_ne!(
        size_of::<service_plugin::IOHIDDeviceTimeStampedDeviceInterface>(),
        0
    );
    assert_ne!(size_of::<service_plugin::IOHIDDeviceQueueInterface>(), 0);
    assert_ne!(
        size_of::<service_plugin::IOHIDDeviceTransactionInterface>(),
        0
    );
}
