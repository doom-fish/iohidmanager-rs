//! Enumerate element metadata and sample a current input value.
//!
//! Run: `cargo run --example 04_elements_and_values`

use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(Some(HidUsage::Keyboard))?;

    let Some(device) = manager.live_devices().into_iter().next() else {
        println!("No keyboard found");
        return Ok(());
    };

    let elements = device.elements();
    println!("keyboard exposes {} elements", elements.len());
    for element in elements.iter().take(8) {
        println!(
            "usage={:>4}/{:>4} kind={:?} report_id={} size={} count={} name={:?}",
            element.usage_page(),
            element.usage(),
            element.element_kind(),
            element.report_id(),
            element.report_size_bits(),
            element.report_count(),
            element.name(),
        );
    }

    if let Some(element) = elements.iter().find(|element| {
        matches!(
            element.element_kind(),
            HidElementType::InputMisc
                | HidElementType::InputButton
                | HidElementType::InputAxis
                | HidElementType::InputScanCodes
        )
    }) {
        match device.get_value(element) {
            Ok(value) => {
                let bytes = value.bytes();
                let preview_len = bytes.len().min(8);
                println!(
                    "sample value usage={}/{} int={} len={} bytes={:?}",
                    element.usage_page(),
                    element.usage(),
                    value.integer_value(),
                    bytes.len(),
                    &bytes[..preview_len],
                );
            }
            Err(err) => println!("sample value unavailable: {err}"),
        }
    }

    Ok(())
}
