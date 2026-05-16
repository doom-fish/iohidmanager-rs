use iohidmanager::service_plugin::{ServicePlugInUuid, ALL_SERVICE_PLUGIN_UUIDS};

#[test]
fn service_plugin_bridge_matches_header_values() {
    for uuid in ALL_SERVICE_PLUGIN_UUIDS {
        assert_eq!(uuid.bridge_bytes().expect("bridge bytes"), uuid.bytes());
    }
    assert_ne!(ServicePlugInUuid::DeviceFactory.bytes(), ServicePlugInUuid::DeviceType.bytes());
}
