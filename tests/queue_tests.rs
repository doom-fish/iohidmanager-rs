use iohidmanager::prelude::*;

#[test]
fn queue_create_and_depth_roundtrip() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let queue = HidQueue::new(&device, 4).expect("queue");
    queue.set_depth(8).expect("set depth");
    assert_eq!(queue.depth(), 8);
}
