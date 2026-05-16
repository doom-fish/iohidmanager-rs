use iohidmanager::prelude::*;

#[test]
fn value_no_copy_constructor_smoke() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let Some(element) = device.elements().into_iter().next() else {
        return;
    };
    let bytes = [1_u8];
    let value = unsafe { HidValue::from_bytes_no_copy(element, 0, &bytes) }.expect("value");
    assert!(value.timestamp() == 0);
}
