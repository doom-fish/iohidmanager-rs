use iohidmanager::prelude::*;

#[test]
fn queue_create_and_depth_roundtrip() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let Some(queue) = manager
        .live_devices()
        .into_iter()
        .find_map(|device| HidQueue::new(&device, 4).ok())
    else {
        return;
    };
    queue.set_depth(8).expect("set depth");
    assert_eq!(queue.depth(), 8);
}
