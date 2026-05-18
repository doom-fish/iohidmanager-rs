use crate::bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Describes one usage-page or usage constant mirrored from `IOHIDUsageTables.h`.
pub struct UsageConstant {
    /// Header symbol name such as `kHIDPage_GenericDesktop`.
    pub symbol: &'static str,
    /// Raw numeric value exposed by the framework.
    pub value: u32,
}

/// Mirrors a usage-page constant from `IOHIDUsageTables.h`.
pub const PAGE_GENERIC_DESKTOP: u32 = 0x01;
/// Mirrors a usage-page constant from `IOHIDUsageTables.h`.
pub const PAGE_KEYBOARD_OR_KEYPAD: u32 = 0x07;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_MOUSE: u32 = 0x02;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_JOYSTICK: u32 = 0x04;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_GAME_PAD: u32 = 0x05;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_KEYBOARD: u32 = 0x06;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_KEYPAD: u32 = 0x07;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_MULTI_AXIS_CONTROLLER: u32 = 0x08;
/// Mirrors a usage constant from `IOHIDUsageTables.h`.
pub const USAGE_SYSTEM_MENU: u32 = 0x89;

include!("generated_usage.rs");

#[must_use]
/// Looks up a usage constant mirrored from `IOHIDUsageTables.h` by symbol name.
pub fn constant(symbol: &str) -> Option<UsageConstant> {
    ALL_USAGE_CONSTANTS
        .iter()
        .copied()
        .find(|definition| definition.symbol == symbol)
}

#[must_use]
/// Returns the Swift-bridge mirror of `kHIDPage_GenericDesktop`.
pub fn bridge_generic_desktop_page() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_page_generic_desktop() }
}

#[must_use]
/// Returns the Swift-bridge mirror of `kHIDUsage_GD_Keyboard`.
pub fn bridge_keyboard_usage() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_generic_desktop_keyboard() }
}

#[must_use]
/// Returns the Swift-bridge mirror of `kHIDUsage_GD_Mouse`.
pub fn bridge_mouse_usage() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_generic_desktop_mouse() }
}
