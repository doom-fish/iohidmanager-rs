import Foundation
import IOKit.hid

@_cdecl("iohidmanager_swift_usage_page_generic_desktop")
public func iohidmanager_swift_usage_page_generic_desktop() -> UInt32 {
    UInt32(kHIDPage_GenericDesktop)
}

@_cdecl("iohidmanager_swift_usage_generic_desktop_keyboard")
public func iohidmanager_swift_usage_generic_desktop_keyboard() -> UInt32 {
    UInt32(kHIDUsage_GD_Keyboard)
}

@_cdecl("iohidmanager_swift_usage_generic_desktop_mouse")
public func iohidmanager_swift_usage_generic_desktop_mouse() -> UInt32 {
    UInt32(kHIDUsage_GD_Mouse)
}
