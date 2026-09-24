mod common;

use iohidmanager::prelude::*;

#[test]
fn queue_create_and_depth_roundtrip() {
    let Some(manager) = common::live_manager() else {
        return;
    };
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
