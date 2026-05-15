//! Filter to keyboards / mice / gamepads via the `HidUsage` presets.
//!
//! Run: `cargo run --example 02_filter_keyboards`

use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for (label, usage) in [
        ("Keyboards", HidUsage::Keyboard),
        ("Mice",      HidUsage::Mouse),
        ("Joysticks", HidUsage::Joystick),
        ("GamePads",  HidUsage::GamePad),
    ] {
        let m = HidManager::new()?;
        m.set_device_matching(Some(usage))?;
        let devices = m.devices();
        println!("== {label} ({} found) ==", devices.len());
        for d in &devices {
            println!("  - {:<30} ({:04x}:{:04x})",
                d.product.as_deref().unwrap_or("?"),
                d.vendor_id.unwrap_or(0),
                d.product_id.unwrap_or(0));
        }
        println!();
    }
    Ok(())
}
