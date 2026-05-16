//! Match multiple device classes and inspect a few common properties.
//!
//! Run: `cargo run --example 03_matching_and_properties`

use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching_multiple(&[
        DeviceMatch::usage(HidUsage::Keyboard),
        DeviceMatch::usage(HidUsage::Mouse),
    ])?;

    let devices = manager.live_devices();
    println!("matched {} keyboard/mouse devices", devices.len());
    for device in devices.iter().take(5) {
        let info = device.info();
        let descriptor_len = device
            .report_descriptor()
            .map_or(0, |descriptor| descriptor.len());
        println!(
            "[{:04x}:{:04x}] {:<10} {:<30} descriptor={} bytes",
            info.vendor_id.unwrap_or(0),
            info.product_id.unwrap_or(0),
            info.transport.as_deref().unwrap_or("?"),
            info.product.as_deref().unwrap_or("?"),
            descriptor_len,
        );
    }

    Ok(())
}
