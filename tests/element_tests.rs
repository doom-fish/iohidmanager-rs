use iohidmanager::prelude::*;

#[test]
fn element_attach_detach_smoke() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let elements = device.elements();
    if elements.len() < 2 {
        return;
    }
    elements[0].attach(&elements[1]);
    elements[0].detach(&elements[1]);
}
