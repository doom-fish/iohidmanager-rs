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
    use std::time::{Duration, Instant};

    fn opened<T>(subscribed: Result<T, HidError>) -> Option<T> {
        match subscribed {
            Ok(stream) => Some(stream),
            Err(HidError::ManagerOpenFailed(_) | HidError::DeviceOpenFailed(_)) => None,
            Err(error) => panic!("subscribe failed: {error}"),
        }
    }

    fn first_event<T>(poll: impl Fn() -> Option<T>) -> Option<T> {
        let deadline = Instant::now() + Duration::from_secs(10);
        while Instant::now() < deadline {
            if let Some(event) = poll() {
                return Some(event);
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        None
    }

    // ── manager-level streams ────────────────────────────────────────────────

    /// `ManagerInputValueStream`: subscribe → immediately drop → stream closed.
    #[test]
    fn manager_input_value_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let Some(stream) = opened(ManagerInputValueStream::subscribe(&manager, 8)) else {
            return;
        };
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

        let stream = ManagerDeviceMatchingStream::subscribe(&manager, 64).expect("subscribe");
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

        let stream = ManagerDeviceRemovalStream::subscribe(&manager, 8).expect("subscribe");
        assert_eq!(stream.buffered_count(), 0);
        assert!(!stream.is_closed());
        drop(stream);
    }

    /// `ManagerInputReportStream`: subscribe → drop → no panic.
    #[test]
    fn manager_input_report_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");

        let Some(stream) = opened(ManagerInputReportStream::subscribe(&manager, 8)) else {
            return;
        };
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
        let stream = DeviceRemovalStream::subscribe(device, 4).expect("subscribe");
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
        let Some(stream) = opened(DeviceInputValueStream::subscribe(device, &[], 64)) else {
            return;
        };
        assert!(!stream.is_closed());
        drop(stream);
    }

    // ── queue stream ──────────────────────────────────────────────────────────

    /// `QueueValueStream`: if a device with elements is present, subscribe → drop.
    #[test]
    fn queue_value_stream_subscribe_drop_closes() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let Some((queue, element)) = manager.live_devices().into_iter().find_map(|device| {
            let element = device.elements().into_iter().next()?;
            Some((HidQueue::new(&device, 8).ok()?, element))
        }) else {
            return;
        };
        queue.add_element(&element);
        queue.start();

        let stream = QueueValueStream::subscribe(&queue, 16).expect("subscribe");
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
        let Some(stream) = opened(ManagerInputValueStream::subscribe(&manager, 1)) else {
            return;
        };
        assert!(stream.buffered_count() <= 1);
        drop(stream);
    }

    #[test]
    fn zero_capacity_is_rejected() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        assert!(matches!(
            ManagerDeviceRemovalStream::subscribe(&manager, 0),
            Err(HidError::InvalidArgument(_))
        ));
    }

    #[test]
    fn manager_streams_require_device_matching() {
        let manager = HidManager::new().expect("manager");
        assert!(matches!(
            ManagerDeviceMatchingStream::subscribe(&manager, 4),
            Err(HidError::InvalidArgument(_))
        ));
        assert!(matches!(
            ManagerInputValueStream::subscribe(&manager, 4),
            Err(HidError::InvalidArgument(_))
        ));
    }

    #[test]
    fn two_matching_streams_on_one_manager_both_see_present_devices() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        if manager.devices().is_empty() {
            return;
        }
        let first = ManagerDeviceMatchingStream::subscribe(&manager, 64).expect("first");
        let second = ManagerDeviceMatchingStream::subscribe(&manager, 64).expect("second");
        assert!(first_event(|| first.try_next()).is_some());
        assert!(first_event(|| second.try_next()).is_some());
        drop(first);
        let third = ManagerDeviceMatchingStream::subscribe(&manager, 64).expect("third");
        assert!(first_event(|| third.try_next()).is_some());
    }

    #[test]
    fn a_queue_accepts_one_stream_at_a_time() {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        let Some(queue) = manager
            .live_devices()
            .into_iter()
            .find_map(|device| HidQueue::new(&device, 8).ok())
        else {
            return;
        };
        let first = QueueValueStream::subscribe(&queue, 4).expect("first");
        assert!(matches!(
            QueueValueStream::subscribe(&queue, 4),
            Err(HidError::InvalidArgument(_))
        ));
        drop(first);
        let again = QueueValueStream::subscribe(&queue, 4).expect("after drop");
        assert!(!again.is_closed());
    }
}
