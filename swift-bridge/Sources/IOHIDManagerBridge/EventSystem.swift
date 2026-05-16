import Foundation
import IOKit.hid

@_cdecl("iohidmanager_swift_event_keyboard_no_repeat")
public func iohidmanager_swift_event_keyboard_no_repeat() -> UInt32 {
    kIOHIDKeyboardEventOptionsNoKeyRepeat.rawValue
}

@_cdecl("iohidmanager_swift_event_sensor_control_decimation")
public func iohidmanager_swift_event_sensor_control_decimation() -> UInt32 {
    kIOHIDServiceSensorControlDecimation.rawValue
}
