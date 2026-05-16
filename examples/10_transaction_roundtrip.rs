use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("no HID device available");
        return Ok(());
    };

    let transaction = HidTransaction::new(&device, HidTransactionDirection::Input)?;
    if let Some(element) = device.elements().into_iter().next() {
        transaction.add_element(&element);
        println!("transaction contains element: {}", transaction.contains_element(&element));
        transaction.remove_element(&element);
    } else {
        println!("device has no HID elements");
    }
    transaction.clear();
    Ok(())
}
