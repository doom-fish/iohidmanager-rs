import Foundation
import IOKit.hid

@inline(__always)
func hidObject<T>(_ raw: UnsafeMutableRawPointer?, as type: T.Type) -> T? {
    guard let raw else {
        return nil
    }
    return Unmanaged<AnyObject>.fromOpaque(raw).takeUnretainedValue() as? T
}

@inline(__always)
func retainOpaque<T: AnyObject>(_ object: T) -> UnsafeMutableRawPointer {
    UnsafeMutableRawPointer(Unmanaged.passRetained(object).toOpaque())
}
