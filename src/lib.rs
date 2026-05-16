#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's [IOKit HID](https://developer.apple.com/documentation/iokit/iohidmanager_h)
//! subsystem on macOS — enumerate connected mice, keyboards, gamepads,
//! and other HID devices.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
#[path = "ffi/mod.rs"]
pub(crate) mod ffi_impl;
mod bridge;
pub mod hid;

#[cfg(feature = "raw-ffi")]
pub mod ffi {
    pub use crate::ffi_impl::*;
}

pub use error::HidError;
pub use hid::{
    event_system, keys, service_plugin, usage, DeviceMatch, DeviceRemovalSubscription,
    ElementMatch, HidCollectionType, HidDevice, HidDeviceInfo, HidElement, HidElementType,
    HidInputReport, HidManager, HidQueue, HidQueueOptions, HidReportType, HidTransaction,
    HidTransactionDirection, HidTransactionOptions, HidUsage, HidValue, HidValueScale,
    ManagerDeviceSubscription, ManagerReportSubscription, ManagerValueSubscription,
    QueueValueAvailableSubscription, ReportSubscription, TimestampedReportSubscription,
    ValueSubscription,
};

/// Common imports.
pub mod prelude {
    pub use crate::error::HidError;
    pub use crate::hid::{
        DeviceMatch, DeviceRemovalSubscription, ElementMatch, HidCollectionType, HidDevice,
        HidDeviceInfo, HidElement, HidElementType, HidInputReport, HidManager, HidQueue,
        HidQueueOptions, HidReportType, HidTransaction, HidTransactionDirection,
        HidTransactionOptions, HidUsage, HidValue, HidValueScale, ManagerDeviceSubscription,
        ManagerReportSubscription, ManagerValueSubscription, QueueValueAvailableSubscription,
        ReportSubscription, TimestampedReportSubscription, ValueSubscription,
    };
}
