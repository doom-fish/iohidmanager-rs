mod common;

#[test]
fn device_multi_value_roundtrip() {
    let Some(manager) = common::live_manager() else {
        return;
    };
    let devices = manager.live_devices();
    let Some(device) = devices.first() else {
        return;
    };
    let _removal = devices.iter().find_map(|candidate| candidate.on_removal(|| {}).ok());
    let values = device.copy_value_multiple(&[]).expect("copy multiple");
    assert!(values.is_empty());
}
