import Foundation
import IOKit.hid

@_cdecl("iohidmanager_swift_element_type_id")
public func iohidmanager_swift_element_type_id() -> UInt {
    UInt(IOHIDElementGetTypeID())
}
