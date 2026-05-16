use iohidmanager::prelude::*;

#[test]
fn manager_matching_configuration_smoke() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    manager.set_input_value_matching(None).expect("value matching");
    manager
        .set_input_value_matching_multiple(&[])
        .expect("value matching multiple");
    let _ = manager.devices();
}
