mod common;

use iohidmanager::service_plugin::{ServicePlugInUuid, ALL_SERVICE_PLUGIN_UUIDS};

fn sdk_uuid(header: &str, symbol: &str) -> [u8; 16] {
    let pattern =
        format!(r"#define\s+{symbol}\s+CFUUIDGetConstantUUIDWithBytes\(\s*NULL\s*,([^)]*)\)");
    let re = regex_lite::Regex::new(&pattern).expect("regex");
    let caps = re
        .captures(header)
        .unwrap_or_else(|| panic!("{symbol} not found in IOHIDDevicePlugIn.h"));
    let bytes: Vec<u8> = caps[1]
        .split(',')
        .map(|byte| byte.replace('\\', ""))
        .map(|byte| u8::try_from(common::parse_integer(&byte)).expect("byte"))
        .collect();
    bytes.try_into().expect("16 UUID bytes")
}

#[test]
fn service_plugin_uuids_match_sdk_header() {
    let header = common::hid_header("IOHIDDevicePlugIn");
    for uuid in ALL_SERVICE_PLUGIN_UUIDS {
        let symbol = match uuid {
            ServicePlugInUuid::DeviceFactory => "kIOHIDDeviceFactoryID",
            ServicePlugInUuid::DeviceType => "kIOHIDDeviceTypeID",
            ServicePlugInUuid::DeviceInterface => "kIOHIDDeviceDeviceInterfaceID",
            ServicePlugInUuid::DeviceInterfaceV2 => "kIOHIDDeviceDeviceInterfaceID2",
            ServicePlugInUuid::QueueInterface => "kIOHIDDeviceQueueInterfaceID",
            ServicePlugInUuid::TransactionInterface => "kIOHIDDeviceTransactionInterfaceID",
        };
        assert_eq!(uuid.bytes(), sdk_uuid(&header, symbol), "{symbol}");
    }
    assert_ne!(
        ServicePlugInUuid::DeviceFactory.bytes(),
        ServicePlugInUuid::DeviceType.bytes()
    );
}
