//! Errors returned by the `iohidmanager` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HidError {
    ManagerCreateFailed,
    ManagerOpenFailed(i32),
    DeviceOpenFailed(i32),
    IoReturn(&'static str, i32),
    OperationFailed(&'static str),
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
