//! Executor-agnostic async stream wrappers for `IOKit` HID callback APIs.
//!
//! Each function on an `IOHIDManager`, `IOHIDDevice`, or `IOHIDQueue` that
//! accepts a C callback is wrapped here as a
//! [`doom_fish_utils::stream::BoundedAsyncStream`] of typed events.
//!
//! # Run-loop convention
//!
//! Each stream subscription spawns a **dedicated background thread** that runs
//! a `CFRunLoop`.  The `IOKit` source (manager / device / queue) is scheduled
//! on that thread's run loop, so callbacks fire independently of any async
//! executor or the caller's run loop.  When the stream handle is dropped the
//! run loop is stopped, the thread is joined, and all `IOKit` resources are
//! released in the background thread before `drop` returns.
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
//! | [`DeviceInputValueStream`] | `IOHIDDeviceRegisterInputValueCallback` | [`HidValue`] |
//! | [`QueueValueStream`] | `IOHIDQueueRegisterValueAvailableCallback` | `()` |
//!
//! # Prerequisites for manager-level streams
//!
//! Before calling [`ManagerInputValueStream::subscribe`],
//! [`ManagerDeviceMatchingStream::subscribe`],
//! [`ManagerDeviceRemovalStream::subscribe`], or
//! [`ManagerInputReportStream::subscribe`] you **must** call
//! [`crate::HidManager::set_device_matching`] (or its `_dict` / `_multiple`
//! variants) on the manager.  Scheduling an `IOHIDManager` on a `CFRunLoop`
//! without a device-matching filter causes `IOKit` to raise `SIGTRAP`; passing
//! `None` matches all devices.
//!
//! ```no_run
//! use iohidmanager::prelude::*;
//! use iohidmanager::async_api::ManagerDeviceMatchingStream;
//!
//! let manager = HidManager::new().unwrap();
//! manager.set_device_matching(None).unwrap(); // required before any subscribe call
//! let stream = ManagerDeviceMatchingStream::subscribe(&manager, 32);
//! ```

use std::sync::mpsc;
use std::thread;

use core::ffi::c_void;
use core::ptr;

use doom_fish_utils::stream::{AsyncStreamSender, BoundedAsyncStream, NextItem};

use crate::ffi_impl as ffi;
use crate::hid::{
    clone_value_ref, HidDevice, HidInputReport, HidManager, HidQueue, HidReportType, HidValue,
};

// ──────────────────────────── event types ────────────────────────────────────

/// Event emitted by [`ManagerInputValueStream`] and [`DeviceInputValueStream`].
#[derive(Clone)]
pub struct InputValueEvent {
    /// The device that produced the value (manager-level stream only; for
    /// per-device streams use [`DeviceInputValueStream`] which emits [`HidValue`]
    /// directly).
    pub device: HidDevice,
    /// The changed input value.
    pub value: HidValue,
}

/// Event emitted by [`ManagerDeviceMatchingStream`].
#[derive(Clone)]
pub struct DeviceMatchingEvent {
    /// The newly attached device.
    pub device: HidDevice,
}

/// Event emitted by [`ManagerDeviceRemovalStream`].
#[derive(Clone)]
pub struct DeviceRemovalEvent {
    /// The device that was removed.
    pub device: HidDevice,
}

/// Event emitted by [`ManagerInputReportStream`].
#[derive(Clone)]
pub struct InputReportEvent {
    /// The device that sent the report.
    pub device: HidDevice,
    /// The raw HID report.
    pub report: HidInputReport,
}

// ──────────────────────── run-loop thread helpers ─────────────────────────────

/// Wraps a `CFRunLoopRef` so it can be sent through an `mpsc::channel`.
struct SendableRunLoop(ffi::CFRunLoopRef);
// SAFETY: `CFRunLoopRef` is documented as thread-safe.
unsafe impl Send for SendableRunLoop {}

/// RAII guard that stops the dedicated run-loop thread and joins it.
///
/// Stored inside every stream handle; dropping the stream handle drops this
/// guard, which stops the run loop, waits for the background thread to finish
/// its cleanup, and only then returns.
struct RunLoopThreadGuard {
    run_loop: ffi::CFRunLoopRef,
    thread: Option<thread::JoinHandle<()>>,
}

// SAFETY: `CFRunLoopRef` is thread-safe; the JoinHandle is Send.
unsafe impl Send for RunLoopThreadGuard {}
unsafe impl Sync for RunLoopThreadGuard {}

impl Drop for RunLoopThreadGuard {
    fn drop(&mut self) {
        if !self.run_loop.is_null() {
            unsafe { ffi::CFRunLoopStop(self.run_loop) };
        }
        if let Some(t) = self.thread.take() {
            t.join().ok();
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
//  Manager-level streams
// ════════════════════════════════════════════════════════════════════════════

// ── ManagerInputValueStream ───────────────────────────────────────────────────

unsafe extern "C" fn mgr_input_value_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void, // device ref
    value: ffi::IOHIDValueRef,
) {
    if context.is_null() || sender.is_null() || value.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let Some(value) = clone_value_ref(value) else {
        return;
    };
    let device = HidDevice::from_raw_retained(sender.cast());
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<InputValueEvent>` that was `Box::into_raw`'d in
    // `subscribe()`; it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<InputValueEvent>>() };
    tx.push(InputValueEvent { device, value });
}

/// Async stream of [`InputValueEvent`]s from
/// `IOHIDManagerRegisterInputValueCallback`.
pub struct ManagerInputValueStream {
    inner: BoundedAsyncStream<InputValueEvent>,
    _guard: RunLoopThreadGuard,
}

// SAFETY: inner is Send+Sync; guard is Send+Sync.
unsafe impl Send for ManagerInputValueStream {}
unsafe impl Sync for ManagerInputValueStream {}

impl ManagerInputValueStream {
    /// Subscribe to input-value events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing.  `IOKit`
    /// raises `SIGTRAP` if the manager is scheduled on a run loop without a
    /// device-matching filter; passing `None` matches all devices.
    #[must_use]
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let manager_raw = manager.as_ptr() as usize;
        unsafe { ffi::CFRetain(manager_raw as ffi::IOHIDManagerRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let manager_raw = manager_raw as ffi::IOHIDManagerRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<InputValueEvent>;
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            unsafe {
                ffi::IOHIDManagerScheduleWithRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::IOHIDManagerRegisterInputValueCallback(
                    manager_raw,
                    Some(mgr_input_value_cb),
                    sender_ptr.cast(),
                );
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            unsafe { ffi::CFRunLoopRun() };
            // Cleanup: deregister, unschedule, release, drop sender.
            unsafe {
                ffi::IOHIDManagerRegisterInputValueCallback(manager_raw, None, ptr::null_mut());
                ffi::IOHIDManagerUnscheduleFromRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::CFRelease(manager_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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

unsafe extern "C" fn mgr_device_matching_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    device: ffi::IOHIDDeviceRef,
) {
    if context.is_null() || device.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let device = HidDevice::from_raw_retained(device);
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<DeviceMatchingEvent>` that was `Box::into_raw`'d in
    // `subscribe()`; it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<DeviceMatchingEvent>>() };
    tx.push(DeviceMatchingEvent { device });
}

/// Async stream of [`DeviceMatchingEvent`]s from
/// `IOHIDManagerRegisterDeviceMatchingCallback`.
pub struct ManagerDeviceMatchingStream {
    inner: BoundedAsyncStream<DeviceMatchingEvent>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for ManagerDeviceMatchingStream {}
unsafe impl Sync for ManagerDeviceMatchingStream {}

impl ManagerDeviceMatchingStream {
    /// Subscribe to device-matching events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing.  `IOKit`
    /// raises `SIGTRAP` if the manager is scheduled on a run loop without a
    /// device-matching filter; passing `None` matches all devices.
    #[must_use]
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let manager_raw = manager.as_ptr() as usize;
        unsafe { ffi::CFRetain(manager_raw as ffi::IOHIDManagerRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let manager_raw = manager_raw as ffi::IOHIDManagerRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<DeviceMatchingEvent>;
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            unsafe {
                ffi::IOHIDManagerScheduleWithRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::IOHIDManagerRegisterDeviceMatchingCallback(
                    manager_raw,
                    Some(mgr_device_matching_cb),
                    sender_ptr.cast(),
                );
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            unsafe { ffi::CFRunLoopRun() };
            unsafe {
                ffi::IOHIDManagerRegisterDeviceMatchingCallback(manager_raw, None, ptr::null_mut());
                ffi::IOHIDManagerUnscheduleFromRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::CFRelease(manager_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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

unsafe extern "C" fn mgr_device_removal_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    device: ffi::IOHIDDeviceRef,
) {
    if context.is_null() || device.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let device = HidDevice::from_raw_retained(device);
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<DeviceRemovalEvent>` that was `Box::into_raw`'d in
    // `subscribe()`; it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<DeviceRemovalEvent>>() };
    tx.push(DeviceRemovalEvent { device });
}

/// Async stream of [`DeviceRemovalEvent`]s from
/// `IOHIDManagerRegisterDeviceRemovalCallback`.
pub struct ManagerDeviceRemovalStream {
    inner: BoundedAsyncStream<DeviceRemovalEvent>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for ManagerDeviceRemovalStream {}
unsafe impl Sync for ManagerDeviceRemovalStream {}

impl ManagerDeviceRemovalStream {
    /// Subscribe to device-removal events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing.  `IOKit`
    /// raises `SIGTRAP` if the manager is scheduled on a run loop without a
    /// device-matching filter; passing `None` matches all devices.
    #[must_use]
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let manager_raw = manager.as_ptr() as usize;
        unsafe { ffi::CFRetain(manager_raw as ffi::IOHIDManagerRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let manager_raw = manager_raw as ffi::IOHIDManagerRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<DeviceRemovalEvent>;
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            unsafe {
                ffi::IOHIDManagerScheduleWithRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::IOHIDManagerRegisterDeviceRemovalCallback(
                    manager_raw,
                    Some(mgr_device_removal_cb),
                    sender_ptr.cast(),
                );
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            unsafe { ffi::CFRunLoopRun() };
            unsafe {
                ffi::IOHIDManagerRegisterDeviceRemovalCallback(manager_raw, None, ptr::null_mut());
                ffi::IOHIDManagerUnscheduleFromRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::CFRelease(manager_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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

unsafe extern "C" fn mgr_input_report_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void, // device ref
    report_type: ffi::IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: ffi::CFIndex,
) {
    if context.is_null() || sender.is_null() || report.is_null() || result != ffi::kIOReturnSuccess
    {
        return;
    }
    let length = usize::try_from(report_length).unwrap_or(0);
    let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
    let device = HidDevice::from_raw_retained(sender.cast());
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<InputReportEvent>` that was `Box::into_raw`'d in
    // `subscribe()`; it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<InputReportEvent>>() };
    tx.push(InputReportEvent {
        device,
        report: HidInputReport {
            report_type: HidReportType::from_raw(report_type),
            report_id,
            bytes,
            timestamp: 0,
        },
    });
}

/// Async stream of [`InputReportEvent`]s from
/// `IOHIDManagerRegisterInputReportCallback`.
pub struct ManagerInputReportStream {
    inner: BoundedAsyncStream<InputReportEvent>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for ManagerInputReportStream {}
unsafe impl Sync for ManagerInputReportStream {}

impl ManagerInputReportStream {
    /// Subscribe to raw HID-report events on `manager`.
    ///
    /// `capacity` is the number of events the bounded buffer can hold before
    /// the oldest item is dropped (lossy by design).
    ///
    /// # Prerequisites
    ///
    /// Call [`crate::HidManager::set_device_matching`] (or its `_dict` /
    /// `_multiple` variants) on `manager` **before** subscribing.  `IOKit`
    /// raises `SIGTRAP` if the manager is scheduled on a run loop without a
    /// device-matching filter; passing `None` matches all devices.
    #[must_use]
    pub fn subscribe(manager: &HidManager, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let manager_raw = manager.as_ptr() as usize;
        unsafe { ffi::CFRetain(manager_raw as ffi::IOHIDManagerRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let manager_raw = manager_raw as ffi::IOHIDManagerRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<InputReportEvent>;
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            unsafe {
                ffi::IOHIDManagerScheduleWithRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::IOHIDManagerRegisterInputReportCallback(
                    manager_raw,
                    Some(mgr_input_report_cb),
                    sender_ptr.cast(),
                );
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            unsafe { ffi::CFRunLoopRun() };
            unsafe {
                ffi::IOHIDManagerRegisterInputReportCallback(manager_raw, None, ptr::null_mut());
                ffi::IOHIDManagerUnscheduleFromRunLoop(
                    manager_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::CFRelease(manager_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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

unsafe extern "C" fn dev_removal_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    if context.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<()>` that was `Box::into_raw`'d in `subscribe()`;
    // it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<()>>() };
    tx.push(());
}

/// Async stream of `()` events from `IOHIDDeviceRegisterRemovalCallback`.
///
/// Fires once when the device is removed.  Drop the stream to clean up.
pub struct DeviceRemovalStream {
    inner: BoundedAsyncStream<()>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for DeviceRemovalStream {}
unsafe impl Sync for DeviceRemovalStream {}

impl DeviceRemovalStream {
    /// Subscribe to the removal event for `device`.
    #[must_use]
    pub fn subscribe(device: &HidDevice, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let device_raw = device.as_ptr() as usize;
        unsafe { ffi::CFRetain(device_raw as ffi::IOHIDDeviceRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let device_raw = device_raw as ffi::IOHIDDeviceRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<()>;
            // Open and schedule on this thread's run loop.
            let open_ok = unsafe {
                ffi::IOHIDDeviceOpen(device_raw, ffi::kIOHIDOptionsTypeNone)
                    == ffi::kIOReturnSuccess
            };
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            if open_ok {
                unsafe {
                    ffi::IOHIDDeviceScheduleWithRunLoop(
                        device_raw,
                        run_loop,
                        ffi::kCFRunLoopDefaultMode,
                    );
                    ffi::IOHIDDeviceRegisterRemovalCallback(
                        device_raw,
                        Some(dev_removal_cb),
                        sender_ptr.cast(),
                    );
                }
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            if open_ok {
                unsafe { ffi::CFRunLoopRun() };
            }
            unsafe {
                if open_ok {
                    ffi::IOHIDDeviceRegisterRemovalCallback(device_raw, None, ptr::null_mut());
                    ffi::IOHIDDeviceUnscheduleFromRunLoop(
                        device_raw,
                        run_loop,
                        ffi::kCFRunLoopDefaultMode,
                    );
                    let _ = ffi::IOHIDDeviceClose(device_raw, ffi::kIOHIDOptionsTypeNone);
                }
                ffi::CFRelease(device_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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

unsafe extern "C" fn dev_input_value_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    value: ffi::IOHIDValueRef,
) {
    if context.is_null() || value.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let Some(value) = clone_value_ref(value) else {
        return;
    };
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<HidValue>` that was `Box::into_raw`'d in
    // `subscribe()`; it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<HidValue>>() };
    tx.push(value);
}

/// Async stream of [`HidValue`] events from
/// `IOHIDDeviceRegisterInputValueCallback`.
pub struct DeviceInputValueStream {
    inner: BoundedAsyncStream<HidValue>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for DeviceInputValueStream {}
unsafe impl Sync for DeviceInputValueStream {}

impl DeviceInputValueStream {
    /// Subscribe to input-value events for `device`.
    #[must_use]
    pub fn subscribe(device: &HidDevice, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let device_raw = device.as_ptr() as usize;
        unsafe { ffi::CFRetain(device_raw as ffi::IOHIDDeviceRef) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let device_raw = device_raw as ffi::IOHIDDeviceRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<HidValue>;
            let open_ok = unsafe {
                ffi::IOHIDDeviceOpen(device_raw, ffi::kIOHIDOptionsTypeNone)
                    == ffi::kIOReturnSuccess
            };
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            if open_ok {
                unsafe {
                    ffi::IOHIDDeviceScheduleWithRunLoop(
                        device_raw,
                        run_loop,
                        ffi::kCFRunLoopDefaultMode,
                    );
                    ffi::IOHIDDeviceRegisterInputValueCallback(
                        device_raw,
                        Some(dev_input_value_cb),
                        sender_ptr.cast(),
                    );
                }
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            if open_ok {
                unsafe { ffi::CFRunLoopRun() };
            }
            unsafe {
                if open_ok {
                    ffi::IOHIDDeviceRegisterInputValueCallback(device_raw, None, ptr::null_mut());
                    ffi::IOHIDDeviceUnscheduleFromRunLoop(
                        device_raw,
                        run_loop,
                        ffi::kCFRunLoopDefaultMode,
                    );
                    let _ = ffi::IOHIDDeviceClose(device_raw, ffi::kIOHIDOptionsTypeNone);
                }
                ffi::CFRelease(device_raw);
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
    }

    /// Returns a future that resolves to the next [`HidValue`].
    #[must_use]
    pub const fn next(&self) -> NextItem<'_, HidValue> {
        self.inner.next()
    }
    /// Non-blocking poll.
    #[must_use]
    pub fn try_next(&self) -> Option<HidValue> {
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

unsafe extern "C" fn queue_value_available_cb(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    if context.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    // SAFETY: context is non-null (checked above) and points to the
    // `AsyncStreamSender<()>` that was `Box::into_raw`'d in `subscribe()`;
    // it lives until the run-loop cleanup block drops it.
    let tx = unsafe { &*context.cast::<AsyncStreamSender<()>>() };
    tx.push(());
}

/// Async stream of `()` notifications from
/// `IOHIDQueueRegisterValueAvailableCallback`.
///
/// Each item signals that new values are available in the queue.  Call
/// [`HidQueue::copy_next_value`](crate::HidQueue::copy_next_value) repeatedly
/// until it returns `None` to drain all available values.
pub struct QueueValueStream {
    inner: BoundedAsyncStream<()>,
    _guard: RunLoopThreadGuard,
}

unsafe impl Send for QueueValueStream {}
unsafe impl Sync for QueueValueStream {}

impl QueueValueStream {
    /// Subscribe to value-available notifications on `queue`.
    #[must_use]
    pub fn subscribe(queue: &HidQueue, capacity: usize) -> Self {
        let (stream, sender) = BoundedAsyncStream::new(capacity);
        let sender_ptr = Box::into_raw(Box::new(sender)) as usize;
        let queue_raw = queue.as_ptr() as usize;
        unsafe { ffi::CFRetain((queue_raw as ffi::IOHIDQueueRef).cast_const()) };

        let (tx, rx) = mpsc::channel::<SendableRunLoop>();
        let thread = thread::spawn(move || {
            let queue_raw = queue_raw as ffi::IOHIDQueueRef;
            let sender_ptr = sender_ptr as *mut AsyncStreamSender<()>;
            let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
            unsafe {
                ffi::IOHIDQueueScheduleWithRunLoop(queue_raw, run_loop, ffi::kCFRunLoopDefaultMode);
                ffi::IOHIDQueueRegisterValueAvailableCallback(
                    queue_raw,
                    Some(queue_value_available_cb),
                    sender_ptr.cast(),
                );
            }
            tx.send(SendableRunLoop(run_loop)).ok();
            unsafe { ffi::CFRunLoopRun() };
            unsafe {
                ffi::IOHIDQueueRegisterValueAvailableCallback(queue_raw, None, ptr::null_mut());
                ffi::IOHIDQueueUnscheduleFromRunLoop(
                    queue_raw,
                    run_loop,
                    ffi::kCFRunLoopDefaultMode,
                );
                ffi::CFRelease(queue_raw.cast_const());
                drop(Box::from_raw(sender_ptr));
            }
        });
        let run_loop = rx.recv().map_or(ptr::null_mut(), |s| s.0);
        Self {
            inner: stream,
            _guard: RunLoopThreadGuard {
                run_loop,
                thread: Some(thread),
            },
        }
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
