use crate::bridge;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsageConstant {
    pub symbol: &'static str,
    pub value: u32,
}

pub const PAGE_GENERIC_DESKTOP: u32 = 0x01;
pub const PAGE_KEYBOARD_OR_KEYPAD: u32 = 0x07;
pub const USAGE_MOUSE: u32 = 0x02;
pub const USAGE_JOYSTICK: u32 = 0x04;
pub const USAGE_GAME_PAD: u32 = 0x05;
pub const USAGE_KEYBOARD: u32 = 0x06;
pub const USAGE_KEYPAD: u32 = 0x07;
pub const USAGE_MULTI_AXIS_CONTROLLER: u32 = 0x08;
pub const USAGE_SYSTEM_MENU: u32 = 0x89;

include!("generated_usage.rs");

#[must_use]
pub fn constant(symbol: &str) -> Option<UsageConstant> {
    ALL_USAGE_CONSTANTS
        .iter()
        .copied()
        .find(|definition| definition.symbol == symbol)
}

#[must_use]
pub fn bridge_generic_desktop_page() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_page_generic_desktop() }
}

#[must_use]
pub fn bridge_keyboard_usage() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_generic_desktop_keyboard() }
}

#[must_use]
pub fn bridge_mouse_usage() -> u32 {
    unsafe { bridge::iohidmanager_swift_usage_generic_desktop_mouse() }
}
