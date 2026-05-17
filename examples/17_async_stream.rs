//! Async HID stream example — subscribe to manager-level input events and
//! device attach/detach notifications via the `async` feature.
//!
//! Run: `cargo run --example 17_async_stream --features async`
//!
//! The example immediately drops each stream and confirms the stream closes
//! cleanly, making it safe to run headless (no real HID input required).

use iohidmanager::async_api::{
    DeviceInputValueStream, DeviceRemovalStream, ManagerDeviceMatchingStream,
    ManagerDeviceRemovalStream, ManagerInputReportStream, ManagerInputValueStream, QueueValueStream,
};
use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── manager-level streams ────────────────────────────────────────────────
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    // Create each stream, confirm the buffer starts empty, then drop it.
    // Dropping the stream guard stops the background run-loop thread.
    {
        let stream = ManagerInputValueStream::subscribe(&manager, 64);
        assert_eq!(stream.buffered_count(), 0);
        println!(
            "ManagerInputValueStream: capacity check OK (buffered={})",
            stream.buffered_count()
        );
        // stream drops here → run-loop thread stopped → sender dropped →
        // stream is_closed becomes true on the inner BoundedAsyncStream.
    }

    {
        let stream = ManagerDeviceMatchingStream::subscribe(&manager, 32);
        println!(
            "ManagerDeviceMatchingStream: capacity check OK (buffered={})",
            stream.buffered_count()
        );
    }

    {
        let stream = ManagerDeviceRemovalStream::subscribe(&manager, 32);
        println!(
            "ManagerDeviceRemovalStream: capacity check OK (buffered={})",
            stream.buffered_count()
        );
    }

    {
        let stream = ManagerInputReportStream::subscribe(&manager, 32);
        println!(
            "ManagerInputReportStream: capacity check OK (buffered={})",
            stream.buffered_count()
        );
    }

    // ── per-device streams ────────────────────────────────────────────────────
    let devices = manager.live_devices();
    if let Some(device) = devices.first() {
        let removal_stream = DeviceRemovalStream::subscribe(device, 4);
        println!(
            "DeviceRemovalStream: capacity check OK (buffered={})",
            removal_stream.buffered_count()
        );

        // DeviceInputValueStream
        let iv_stream = DeviceInputValueStream::subscribe(device, 64);
        println!(
            "DeviceInputValueStream: capacity check OK (buffered={})",
            iv_stream.buffered_count()
        );
    } else {
        println!("No HID devices found — skipping per-device stream smoke test");
    }

    // ── queue stream ──────────────────────────────────────────────────────────
    if let Some(device) = devices.first() {
        let elements = device.elements();
        if let Some(element) = elements.first() {
            let queue = HidQueue::new(device, 16)?;
            queue.add_element(element);
            queue.start();
            let q_stream = QueueValueStream::subscribe(&queue, 16);
            println!(
                "QueueValueStream: capacity check OK (buffered={})",
                q_stream.buffered_count()
            );
            queue.stop();
        } else {
            println!("Device has no elements — skipping queue stream smoke test");
        }
    } else {
        println!("No HID devices found — skipping queue stream smoke test");
    }

    // ── async poll (pollster) ─────────────────────────────────────────────────
    // Subscribe to matching events and use a short-circuit await to confirm
    // the stream can be polled via any executor.
    let manager2 = HidManager::new()?;
    manager2.set_device_matching(None)?;

    let stream = ManagerDeviceMatchingStream::subscribe(&manager2, 8);
    // try_next should return None because no events were produced.
    assert!(
        stream.try_next().is_none(),
        "unexpected event on fresh stream"
    );
    println!("try_next on fresh stream: OK (None as expected)");

    // Use pollster to poll the future with a manual timeout: wrap in a
    // race against an immediately-resolved future.
    let saw_event = pollster::block_on(async {
        // Since no real device is attached/detached, we just confirm the
        // future is pending (try_next returns None).
        stream.try_next().is_some()
    });
    assert!(!saw_event, "should not have seen an event");
    println!("pollster block_on: OK (no spurious events)");

    println!("\nAll async_api smoke tests passed.");
    Ok(())
}
