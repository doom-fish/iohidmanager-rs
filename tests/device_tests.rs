use iohidmanager::prelude::*;

#[test]
fn device_multi_value_roundtrip() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let devices = manager.live_devices();
    let Some(device) = devices.first() else {
        return;
    };
    let _removal = devices.iter().find_map(|candidate| candidate.on_removal(|| {}).ok());
    let values = device.copy_value_multiple(&[]).expect("copy multiple");
    assert!(values.is_empty());
}
