import Foundation
import IOKit.hid

@_cdecl("iohidmanager_swift_keys_queue_enqueue_all")
public func iohidmanager_swift_keys_queue_enqueue_all() -> UInt32 {
    UInt32(kIOHIDQueueOptionsTypeEnqueueAll)
}

@_cdecl("iohidmanager_swift_keys_standard_type_ansi")
public func iohidmanager_swift_keys_standard_type_ansi() -> UInt32 {
    UInt32(kIOHIDStandardTypeANSI)
}
