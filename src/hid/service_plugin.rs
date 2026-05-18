use crate::bridge;

pub use crate::ffi_impl::{
    CFUUIDBytes, IOHIDDeviceDeviceInterface, IOHIDDeviceQueueInterface,
    IOHIDDeviceTimeStampedDeviceInterface, IOHIDDeviceTransactionInterface, IOHIDIUnknownAddRef,
    IOHIDIUnknownQueryInterface, IOHIDIUnknownRelease, IUnknownVTable,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
/// Enumerates service plug-in UUIDs mirrored from `IOHIDDevicePlugIn.h`.
pub enum ServicePlugInUuid {
    /// Mirrors the device-factory UUID from `IOHIDDevicePlugIn.h`.
    DeviceFactory = 0,
    /// Mirrors the device-type UUID from `IOHIDDevicePlugIn.h`.
    DeviceType = 1,
    /// Mirrors the device-interface UUID from `IOHIDDevicePlugIn.h`.
    DeviceInterface = 2,
    /// Mirrors the v2 device-interface UUID from `IOHIDDevicePlugIn.h`.
    DeviceInterfaceV2 = 3,
    /// Mirrors the queue-interface UUID from `IOHIDDevicePlugIn.h`.
    QueueInterface = 4,
    /// Mirrors the transaction-interface UUID from `IOHIDDevicePlugIn.h`.
    TransactionInterface = 5,
}

impl ServicePlugInUuid {
    #[must_use]
    /// Returns the UUID bytes mirrored from `IOHIDDevicePlugIn.h`.
    pub const fn bytes(self) -> [u8; 16] {
        match self {
            Self::DeviceFactory => [
                0x13, 0xAA, 0x9C, 0x44, 0x6F, 0x1B, 0x11, 0xD4, 0x90, 0x7C, 0x00, 0x05, 0x02, 0x8F,
                0x18, 0xD5,
            ],
            Self::DeviceType => [
                0x7D, 0xDE, 0xEC, 0xA8, 0xA7, 0xB4, 0x11, 0xDA, 0x8A, 0x0E, 0x00, 0x14, 0x51, 0x97,
                0x58, 0xEF,
            ],
            Self::DeviceInterface => [
                0x47, 0x4B, 0xDC, 0x8E, 0x9F, 0x4A, 0x11, 0xDA, 0xB3, 0x66, 0x00, 0x0D, 0x93, 0x6D,
                0x06, 0xD2,
            ],
            Self::DeviceInterfaceV2 => [
                0xB4, 0x73, 0x25, 0x6C, 0x6A, 0x72, 0x4E, 0x04, 0xB6, 0x94, 0xC4, 0x00, 0x1D, 0x20,
                0x20, 0x20,
            ],
            Self::QueueInterface => [
                0x2E, 0xC7, 0x8B, 0xDB, 0x9F, 0x4E, 0x11, 0xDA, 0xB6, 0x5C, 0x00, 0x0D, 0x93, 0x6D,
                0x06, 0xD2,
            ],
            Self::TransactionInterface => [
                0x1F, 0x2E, 0x78, 0xFA, 0x9F, 0xFA, 0x11, 0xDA, 0x90, 0xB4, 0x00, 0x0D, 0x93, 0x6D,
                0x06, 0xD2,
            ],
        }
    }

    #[must_use]
    /// Fetches the UUID bytes from the Swift bridge mirror.
    pub fn bridge_bytes(self) -> Option<[u8; 16]> {
        let mut bytes = [0_u8; 16];
        let ok = unsafe {
            bridge::iohidmanager_swift_service_plugin_uuid(self as u32, bytes.as_mut_ptr())
        };
        ok.then_some(bytes)
    }

    #[must_use]
    /// Formats the service plug-in UUID in hyphenated form.
    pub fn hyphenated(self) -> String {
        let bytes = self.bytes();
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        )
    }
}

/// Collects the service plug-in UUIDs mirrored from `IOHIDDevicePlugIn.h`.
pub const ALL_SERVICE_PLUGIN_UUIDS: &[ServicePlugInUuid] = &[
    ServicePlugInUuid::DeviceFactory,
    ServicePlugInUuid::DeviceType,
    ServicePlugInUuid::DeviceInterface,
    ServicePlugInUuid::DeviceInterfaceV2,
    ServicePlugInUuid::QueueInterface,
    ServicePlugInUuid::TransactionInterface,
];
