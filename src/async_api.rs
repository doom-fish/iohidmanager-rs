//! Executor-agnostic async stream wrappers for `IOKit` HID callback APIs.
//!
//! Each function on an `IOHIDManager`, `IOHIDDevice`, or `IOHIDQueue` that
//! accepts a C callback is wrapped here as a
//! [`doom_fish_utils::stream::BoundedAsyncStream`] of typed events.
//!
//! # Run-loop convention
//!
//! Each stream owns the matching subscription from the synchronous API, so
//! callbacks are delivered on the crate's shared event-loop thread and no
//! caller run loop or async executor has to run.  Dropping a stream stops
//! delivery at once; the `IOKit` objects are released on the event-loop
//! thread once any callback that is already running has returned.
//!
//! # Stream surfaces
//!
//! | Stream type | `IOKit` registration | Event payload |
//! |---|---|---|
//! | [`ManagerInputValueStream`] | `IOHIDManagerRegisterInputValueCallback` | [`InputValueEvent`] |
//! | [`ManagerDeviceMatchingStream`] | `IOHIDManagerRegisterDeviceMatchingCallback` | [`DeviceMatchingEvent`] |
//! | [`ManagerDeviceRemovalStream`] | `IOHIDManagerRegisterDeviceRemovalCallback` | [`DeviceRemovalEvent`] |
//! | [`ManagerInputReportStream`] | `IOHIDManagerRegisterInputReportCallback` | [`InputReportEvent`] |
//! | [`DeviceRemovalStream`] | `IOHIDDeviceRegisterRemovalCallback` | `()` |
//! | [`DeviceInputValueStream`] | `IOHIDDeviceRegisterInputValueCallback` | [`HidValueInfo`] |
//! | [`QueueValueStream`] | `IOHIDQueueRegisterValueAvailableCallback` | `()` |
//!
//! # Prerequisites for manager-level streams
//!
//! Before calling [`ManagerInputValueStream::subscribe`],
//! [`ManagerDeviceMatchingStream::subscribe`],
//! [`ManagerDeviceRemovalStream::subscribe`], or
//! [`ManagerInputReportStream::subscribe`] you **must** call
//! [`crate::HidManager::set_device_matching`] (or its `_dict` / `_multiple`
//! variants) on the manager; otherwise `subscribe` returns
//! [`HidError::InvalidArgument`].  Passing `None` matches all devices.
//!
//! ```no_run
//! use iohidmanager::prelude::*;
//! use iohidmanager::async_api::ManagerDeviceMatchingStream;
//!
//! let manager = HidManager::new().unwrap();
//! manager.set_device_matching(None).unwrap(); // required before any subscribe call
//! let stream = ManagerDeviceMatchingStream::subscribe(&manager, 32).unwrap();
//! ```

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::error::HidError;
use crate::hid::{
    DeviceRemovalSubscription, ElementMatch, HidDevice, HidDeviceId, HidDeviceInfo, HidInputReport,
    HidManager, HidQueue, HidValueInfo, ManagerDeviceSubscription, ManagerReportSubscription,
    ManagerValueSubscription, QueueValueAvailableSubscription, ValueSubscription,
};

// ──────────────────────────── event types ────────────────────────────────────

/// Event emitted by [`ManagerInputValueStream`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputValueEvent {
    /// The device that produced the value (manager-level stream only; for
    /// per-device streams use [`DeviceInputValueStream`] which emits [`HidValueInfo`]
    /// directly).
    pub device: HidDeviceId,
    /// The changed input value.
    pub value: HidValueInfo,
}

/// Event emitted by [`ManagerDeviceMatchingStream`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceMatchingEvent {
    /// The newly attached device.
    pub device: HidDeviceId,
    pub info: HidDeviceInfo,
}

/// Event emitted by [`ManagerDeviceRemovalStream`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeviceRemovalEvent {
    /// The device that was removed.
    pub device: HidDeviceId,
}

/// Event emitted by [`ManagerInputReportStream`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputReportEvent {
    /// The device that sent the report.
    pub device: HidDeviceId,
    /// The raw HID report.
    pub report: HidInputReport,
}

fn stream<T>(capacity: usize) -> Result<(BoundedAsyncStream<T>, AsyncStreamSender<T>), HidError> {
    if capacity == 0 {
        return Err(HidError::InvalidArgument(
            "stream capacity must be greater than zero".to_owned(),
        ));
    }
    Ok(BoundedAsyncStream::new(capacity))
}

// ════════════════════════════════════════════════════════════════════════════
//  Manager-level streams
// ════════════════════════════════════════════════════════════════════════════

// ── ManagerInputValueStream ───────────────────────────────────────────────────

/// Async stream of [`InputValueEvent`]s from
/// `IOHIDManagerRegisterInputValueCallback`.
pub struct ManagerInputValueStream {
    _subscription: ManagerValueSubscription,
    inner: BoundedAsyncStream<InputValueEvent>,
}

#[allow(clippy::missing_errors_doc)]
impl ManagerInputValueStream {
    /// Subscribe to input-value events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing; otherwise
    /// this returns [`HidError::InvalidArgument`].  Passing `None` matches all
    /// devices.
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = manager.on_input_value(move |device, value| {
            if let Ok(device) = device.id() {
                sender.push(InputValueEvent {
                    device,
                    value: value.info(),
                });
            }
        })?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to the next [`InputValueEvent`], or
    /// `None` when the subscription has been dropped.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, InputValueEvent> {
        self.inner.next()
    }
    /// Non-blocking poll — returns `None` if the buffer is empty.
    #[must_use]
    pub fn try_next(&self) -> Option<InputValueEvent> {
        self.inner.try_next()
    }
    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── ManagerDeviceMatchingStream ────────────────────────────────────────────────

/// Async stream of [`DeviceMatchingEvent`]s from
/// `IOHIDManagerRegisterDeviceMatchingCallback`.
pub struct ManagerDeviceMatchingStream {
    _subscription: ManagerDeviceSubscription,
    inner: BoundedAsyncStream<DeviceMatchingEvent>,
}

#[allow(clippy::missing_errors_doc)]
impl ManagerDeviceMatchingStream {
    /// Subscribe to device-matching events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing; otherwise
    /// this returns [`HidError::InvalidArgument`].  Passing `None` matches all
    /// devices.
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = manager.on_device_matching(move |device| {
            if let Ok(id) = device.id() {
                sender.push(DeviceMatchingEvent {
                    device: id,
                    info: device.info(),
                });
            }
        })?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to the next [`DeviceMatchingEvent`].
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, DeviceMatchingEvent> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<DeviceMatchingEvent> {
        self.inner.try_next()
    }
    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── ManagerDeviceRemovalStream ────────────────────────────────────────────────

/// Async stream of [`DeviceRemovalEvent`]s from
/// `IOHIDManagerRegisterDeviceRemovalCallback`.
pub struct ManagerDeviceRemovalStream {
    _subscription: ManagerDeviceSubscription,
    inner: BoundedAsyncStream<DeviceRemovalEvent>,
}

#[allow(clippy::missing_errors_doc)]
impl ManagerDeviceRemovalStream {
    /// Subscribe to device-removal events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing; otherwise
    /// this returns [`HidError::InvalidArgument`].  Passing `None` matches all
    /// devices.
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = manager.on_device_removal(move |device| {
            if let Ok(device) = device.id() {
                sender.push(DeviceRemovalEvent { device });
            }
        })?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to the next [`DeviceRemovalEvent`].
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, DeviceRemovalEvent> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<DeviceRemovalEvent> {
        self.inner.try_next()
    }
    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── ManagerInputReportStream ──────────────────────────────────────────────────

/// Async stream of [`InputReportEvent`]s from
/// `IOHIDManagerRegisterInputReportCallback`.
pub struct ManagerInputReportStream {
    _subscription: ManagerReportSubscription,
    inner: BoundedAsyncStream<InputReportEvent>,
}

#[allow(clippy::missing_errors_doc)]
impl ManagerInputReportStream {
    /// Subscribe to raw HID-report events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing; otherwise
    /// this returns [`HidError::InvalidArgument`].  Passing `None` matches all
    /// devices.
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = manager.on_input_report(move |device, report| {
            if let Ok(device) = device.id() {
                sender.push(InputReportEvent { device, report });
            }
        })?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to the next [`InputReportEvent`].
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, InputReportEvent> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<InputReportEvent> {
        self.inner.try_next()
    }
    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ════════════════════════════════════════════════════════════════════════════
//  Per-device streams
// ════════════════════════════════════════════════════════════════════════════

// ── DeviceRemovalStream ───────────────────────────────────────────────────────

/// Async stream of `()` events from `IOHIDDeviceRegisterRemovalCallback`.
///
/// Fires once when the device is removed.  Drop the stream to clean up.
pub struct DeviceRemovalStream {
    _subscription: DeviceRemovalSubscription,
    inner: BoundedAsyncStream<()>,
}

#[allow(clippy::missing_errors_doc)]
impl DeviceRemovalStream {
    /// Subscribe to the removal event for `device`.
    pub fn subscribe(device: &HidDevice, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = device.on_removal(move || sender.push(()))?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to `Some(())` when the device is removed,
    /// or `None` once the stream is closed.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, ()> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<()> {
        self.inner.try_next()
    }
    /// Number of events currently buffered (0 or 1 for removal streams).
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ── DeviceInputValueStream ────────────────────────────────────────────────────

/// Async stream of [`HidValueInfo`] events from
/// `IOHIDDeviceRegisterInputValueCallback`.
pub struct DeviceInputValueStream {
    _subscription: ValueSubscription,
    inner: BoundedAsyncStream<HidValueInfo>,
}

#[allow(clippy::missing_errors_doc)]
impl DeviceInputValueStream {
    /// Subscribe to input-value events for `device`.
    pub fn subscribe(
        device: &HidDevice,
        matching: &[ElementMatch],
        capacity: usize,
    ) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription =
            device.on_input_value(matching, move |value| sender.push(value.info()))?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to the next [`HidValueInfo`].
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, HidValueInfo> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<HidValueInfo> {
        self.inner.try_next()
    }
    /// Number of events currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}

// ════════════════════════════════════════════════════════════════════════════
//  Queue stream
// ════════════════════════════════════════════════════════════════════════════

/// Async stream of `()` notifications from
/// `IOHIDQueueRegisterValueAvailableCallback`.
///
/// Each item signals that new values are available in the queue.  Call
/// [`HidQueue::copy_next_value`](crate::HidQueue::copy_next_value) repeatedly
/// until it returns `None` to drain all available values.
pub struct QueueValueStream {
    _subscription: QueueValueAvailableSubscription,
    inner: BoundedAsyncStream<()>,
}

#[allow(clippy::missing_errors_doc)]
impl QueueValueStream {
    /// Subscribe to value-available notifications on `queue`.
    pub fn subscribe(queue: &HidQueue, capacity: usize) -> Result<Self, HidError> {
        let (inner, sender) = stream(capacity)?;
        let subscription = queue.on_value_available(move || sender.push(()))?;
        Ok(Self {
            _subscription: subscription,
            inner,
        })
    }

    /// Returns a future that resolves to `Some(())` when values become
    /// available, or `None` when the stream is closed.
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, ()> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<()> {
        self.inner.try_next()
    }
    /// Number of notifications currently buffered.
    #[must_use]
    pub fn buffered_count(&self) -> usize {
        self.inner.buffered_count()
    }
    /// `true` once the subscription handle has been dropped and the buffer is
    /// empty.
    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.inner.is_closed()
    }
}
