mod common;

use iohidmanager::prelude::*;

#[test]
fn value_no_copy_constructor_smoke() {
    let Some(manager) = common::live_manager() else {
        return;
    };
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let Some(element) = device.elements().into_iter().next() else {
        return;
    };
    let bytes = [1_u8];
    let value = unsafe { HidValue::from_bytes_no_copy(&element, 0, &bytes) }.expect("value");
    assert_eq!(value.timestamp(), 0);
}
