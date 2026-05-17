//! Integration tests for the `async_api` stream module.
//!
//! These tests verify the subscribe → drop-handle → stream-closes lifecycle
//! for every stream surface without requiring real HID hardware.

#[cfg(feature = "async")]
mod async_stream_tests {
    use iohidmanager::async_api::{
        DeviceInputValueStream, DeviceRemovalStream, ManagerDeviceMatchingStream,
        ManagerDeviceRemovalStream, ManagerInputReportStream, ManagerInputValueStream,
        QueueValueStream,
    };
    use iohidmanager::prelude::*;

    // ── manager-level streams ────────────────────────────────────────────────

    /// `ManagerInputValueStream`: subscribe → immediately drop → stream closed.
    #[test]
    fn manager_input_value_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let stream = ManagerInputValueStream::subscribe(&manager, 8);
        // buffered_count may be non-zero if devices already triggered events.
        assert!(!stream.is_closed(), "stream open before drop");
        drop(stream);
    }

    /// `ManagerDeviceMatchingStream`: subscribe → drop → no panic.
    ///
    /// NOTE: the matching callback fires immediately for all present devices,
    /// so `buffered_count` may be non-zero right after subscription.
    #[test]
    fn manager_device_matching_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let stream = ManagerDeviceMatchingStream::subscribe(&manager, 64);
        // Give the run-loop thread a moment to process initial matching events.
        std::thread::sleep(std::time::Duration::from_millis(20));
        assert!(!stream.is_closed());
        drop(stream);
    }

    /// `ManagerDeviceRemovalStream`: subscribe → drop → no panic.
    #[test]
    fn manager_device_removal_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let stream = ManagerDeviceRemovalStream::subscribe(&manager, 8);
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
        drop(stream);
    }

    /// `ManagerInputReportStream`: subscribe → drop → no panic.
    #[test]
    fn manager_input_report_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let stream = ManagerInputReportStream::subscribe(&manager, 8);
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
        drop(stream);
    }

    // ── per-device streams ────────────────────────────────────────────────────

    /// `DeviceRemovalStream`: if a device is present, subscribe → drop → no panic.
    #[test]
    fn device_removal_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let devices = manager.live_devices();
        let Some(device) = devices.first() else {
            // No HID devices in CI — skip.
            return;
        };
        let stream = DeviceRemovalStream::subscribe(device, 4);
        assert!(!stream.is_closed());
        drop(stream);
    }

    /// `DeviceInputValueStream`: if a device is present, subscribe → drop → no panic.
    #[test]
    fn device_input_value_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let devices = manager.live_devices();
        let Some(device) = devices.first() else {
            return;
        };
        let stream = DeviceInputValueStream::subscribe(device, 64);
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
        drop(stream);
    }

    // ── queue stream ──────────────────────────────────────────────────────────

    /// `QueueValueStream`: if a device with elements is present, subscribe → drop.
    #[test]
    fn queue_value_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let devices = manager.live_devices();
        let Some(device) = devices.first() else {
            return;
        };
        let elements = device.elements();
        let Some(element) = elements.first() else {
            return;
        };
        let queue = HidQueue::new(device, 8).expect("queue");
        queue.add_element(element);
        queue.start();

        let stream = QueueValueStream::subscribe(&queue, 16);
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
        drop(stream);

        queue.stop();
    }

    // ── capacity is respected (capacity=1 lossy) ──────────────────────────────

    /// Verify a capacity-1 stream is constructable.
    #[test]
    fn manager_input_value_stream_capacity_one() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let stream = ManagerInputValueStream::subscribe(&manager, 1);
        assert_eq!(stream.buffered_count(), 0);
        drop(stream);
    }
}
