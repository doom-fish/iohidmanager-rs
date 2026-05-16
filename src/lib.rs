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
pub mod ffi;
pub mod hid;

pub use error::HidError;
pub use hid::{HidDevice, HidDeviceInfo, HidElement, HidManager, HidUsage, ReportSubscription};

/// Common imports.
pub mod prelude {
    pub use crate::error::HidError;
    pub use crate::hid::{HidDevice, HidDeviceInfo, HidElement, HidManager, HidUsage, ReportSubscription};
}
