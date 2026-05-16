use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("no HID device available");
        return Ok(());
    };

    let elements = device.elements();
    if elements.len() < 2 {
        println!("not enough elements to demonstrate attachment");
        return Ok(());
    }

    elements[0].attach(&elements[1]);
    elements[0].detach(&elements[1]);
    println!("element attachment round-trip completed");
    Ok(())
}
