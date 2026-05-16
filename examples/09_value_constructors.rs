use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("no HID device available");
        return Ok(());
    };
    let Some(element) = device.elements().into_iter().next() else {
        println!("device has no HID elements");
        return Ok(());
    };

    let integer = HidValue::from_integer(element, 0, 1)?;
    let bytes = HidValue::from_bytes(element, 0, &[1])?;
    let no_copy_bytes = [1_u8];
    let no_copy = unsafe { HidValue::from_bytes_no_copy(element, 0, &no_copy_bytes) }?;
    println!(
        "constructed values: integer={} bytes={} no-copy={}",
        integer.integer_value(),
        bytes.bytes().len(),
        no_copy.bytes().len(),
    );
    Ok(())
}
