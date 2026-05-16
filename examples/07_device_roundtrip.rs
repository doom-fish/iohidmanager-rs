use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("no HID device available");
        return Ok(());
    };

    let _removal = device.on_removal(|| {})?;
    let element_count = device.elements().len();
    let values = device.copy_value_multiple(&[])?;
    println!(
        "device {:?} has {} elements and {} multi-value entries",
        device.info(),
        element_count,
        values.len()
    );
    Ok(())
}
