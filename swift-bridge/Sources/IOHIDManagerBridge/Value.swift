import Foundation
import IOKit.hid

@_cdecl("iohidmanager_swift_value_type_id")
public func iohidmanager_swift_value_type_id() -> UInt {
    UInt(IOHIDValueGetTypeID())
}

@_cdecl("iohidmanager_swift_value_create_with_bytes_no_copy")
public func iohidmanager_swift_value_create_with_bytes_no_copy(
    _ elementRaw: UnsafeMutableRawPointer?,
    _ timestamp: UInt64,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> UnsafeMutableRawPointer? {
    guard let element = hidObject(elementRaw, as: IOHIDElement.self) else {
        return nil
    }
    let source = bytes ?? UnsafePointer<UInt8>(bitPattern: 1)
    guard let source else {
        return nil
    }
    guard let value = IOHIDValueCreateWithBytesNoCopy(
        kCFAllocatorDefault,
        element,
        timestamp,
        source,
        length
    ) else {
        return nil
    }
    return retainOpaque(value)
}
