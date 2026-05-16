use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("no HID device available");
        return Ok(());
    };

    let queue = HidQueue::new(&device, 8)?;
    if let Some(element) = device.elements().into_iter().next() {
        queue.add_element(&element);
        println!("queue depth={} contains={}", queue.depth(), queue.contains_element(&element));
        queue.remove_element(&element);
    } else {
        println!("device has no HID elements");
    }
    Ok(())
}
