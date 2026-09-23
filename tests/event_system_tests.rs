mod common;

use iohidmanager::event_system;

#[test]
fn event_system_constants_match_sdk_header() {
    let header = common::hid_header("IOHIDEventServiceTypes");
    assert_eq!(
        u64::from(event_system::KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT),
        common::enum_value(&header, "kIOHIDKeyboardEventOptionsNoKeyRepeat"),
    );
    assert_eq!(
        u64::from(event_system::SENSOR_CONTROL_DECIMATION),
        common::enum_value(&header, "kIOHIDServiceSensorControlDecimation"),
    );
    assert!(event_system::ALL_EVENT_SYSTEM_KEYS.len() >= 10);
}
