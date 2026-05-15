//! List every connected HID device with its vendor/product/usage info.
//!
//! Run: `cargo run --example 01_list_devices`

use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    let devices = manager.devices();
    println!("Found {} HID devices:\n", devices.len());
    for d in &devices {
        println!(
            "[{:04x}:{:04x}] usage={}/{} {:<20} | {:<30} | {} | {}",
            d.vendor_id.unwrap_or(0),
            d.product_id.unwrap_or(0),
            d.usage_page.unwrap_or(0),
            d.usage.unwrap_or(0),
            d.transport.as_deref().unwrap_or("—"),
            d.manufacturer.as_deref().unwrap_or("?"),
            d.product.as_deref().unwrap_or("?"),
            d.serial_number.as_deref().unwrap_or("—"),
        );
    }
    Ok(())
}
