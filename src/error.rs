//! Errors returned by the `iohidmanager` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
/// Represents errors returned by `IOHIDManager*`, `IOHIDDevice*`, and related wrappers.
pub enum HidError {
    /// `IOHIDManagerCreate` returned a null manager.
    ManagerCreateFailed,
    /// `IOHIDManagerOpen` failed with the enclosed `IOReturn`.
    ManagerOpenFailed(i32),
    /// `IOHIDDeviceOpen` failed with the enclosed `IOReturn`.
    DeviceOpenFailed(i32),
    /// A raw `IOHID*` call failed with the named operation and `IOReturn`.
    IoReturn(&'static str, i32),
    /// A non-`IOReturn` wrapper operation failed.
    OperationFailed(&'static str),
    /// A Rust-side argument could not be converted for the underlying framework call.
    InvalidArgument(String),
}

impl fmt::Display for HidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManagerCreateFailed => write!(f, "IOHIDManagerCreate returned NULL"),
            Self::ManagerOpenFailed(status) => {
                write!(f, "IOHIDManagerOpen failed: IOReturn 0x{status:x}")
            }
            Self::DeviceOpenFailed(status) => {
                write!(f, "IOHIDDeviceOpen failed: IOReturn 0x{status:x}")
            }
            Self::IoReturn(operation, status) => {
                write!(f, "{operation} failed: IOReturn 0x{status:x}")
            }
            Self::OperationFailed(operation) => write!(f, "{operation} failed"),
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
        }
    }
}

impl std::error::Error for HidError {}
