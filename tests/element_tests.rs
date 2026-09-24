mod common;

#[test]
fn element_attach_detach_smoke() {
    let Some(manager) = common::live_manager() else {
        return;
    };
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let elements = device.elements();
    if elements.len() < 2 {
        return;
    }
    elements[0]
        .attach(&elements[1])
        .expect("same-device attach");
    assert!(elements[0].attached().contains(&elements[1]));
    elements[0].detach(&elements[1]);
    assert!(!elements[0].attached().contains(&elements[1]));
}
