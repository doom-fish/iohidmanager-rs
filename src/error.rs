//! Errors returned by the `iohidmanager` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HidError {
    ManagerCreateFailed,
    ManagerOpenFailed(i32),
    InvalidArgument(String),
}

impl fmt::Display for HidError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ManagerCreateFailed => write!(f, "IOHIDManagerCreate returned NULL"),
            Self::ManagerOpenFailed(s) => {
                write!(f, "IOHIDManagerOpen failed: IOReturn 0x{s:x}")
            }
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
        }
    }
}

impl std::error::Error for HidError {}
