mod common;

use iohidmanager::prelude::*;

#[test]
fn manager_matching_configuration_smoke() {
    let manager = HidManager::new().expect("manager");
    manager
        .set_input_value_matching(None)
        .expect("value matching");
    manager
        .set_input_value_matching_multiple(&[])
        .expect("value matching multiple");
    assert_eq!(manager.devices(), [] as [iohidmanager::HidDeviceInfo; 0]);
    if !common::live_tests_enabled() {
        return;
    }
    common::match_non_keyboards(&manager);
    let _ = manager.devices();
}
