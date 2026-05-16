//! Register timestamped report and input-value callbacks.
//!
//! Run: `cargo run --example 05_callback_registration`

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(Some(HidUsage::Keyboard))?;

    let devices = manager.live_devices();
    if devices.is_empty() {
        println!("No keyboard found");
        return Ok(());
    }

    let report_events = Arc::new(AtomicUsize::new(0));
    let mut report_registered = false;
    for device in &devices {
        if let Ok(_subscription) = device.on_input_report_with_timestamp(64, {
            let report_events = Arc::clone(&report_events);
            move |report| {
                let _ = report.report_id;
                let _ = report.bytes.len();
                report_events.fetch_add(1, Ordering::Relaxed);
            }
        }) {
            std::thread::sleep(Duration::from_millis(250));
            report_registered = true;
            break;
        }
    }

    let value_events = Arc::new(AtomicUsize::new(0));
    let mut value_registered = false;
    for device in &devices {
        if let Ok(_subscription) = device.on_input_value({
            let value_events = Arc::clone(&value_events);
            move |value| {
                let _ = value.integer_value();
                value_events.fetch_add(1, Ordering::Relaxed);
            }
        }) {
            std::thread::sleep(Duration::from_millis(250));
            value_registered = true;
            break;
        }
    }

    println!(
        "registered report callback: {report_registered}, value callback: {value_registered}; observed {} report events and {} value events",
        report_events.load(Ordering::Relaxed),
        value_events.load(Ordering::Relaxed),
    );

    Ok(())
}
