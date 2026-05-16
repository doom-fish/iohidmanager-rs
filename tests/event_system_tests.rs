use iohidmanager::event_system;

#[test]
fn event_system_constants_match_bridge() {
    assert_eq!(
        event_system::bridge_keyboard_no_key_repeat(),
        event_system::KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT,
    );
    assert_eq!(
        event_system::bridge_sensor_control_decimation(),
        event_system::SENSOR_CONTROL_DECIMATION,
    );
    assert!(event_system::ALL_EVENT_SYSTEM_KEYS.len() >= 10);
}
