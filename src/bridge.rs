#![allow(dead_code)]

use apple_cf::raw::{CFIndex, CFTypeID};
use core::ffi::c_void;

unsafe extern "C" {
    pub fn iohidmanager_swift_manager_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_device_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_element_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_value_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_value_create_with_bytes_no_copy(
        element: *mut c_void,
        timestamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> *mut c_void;
    pub fn iohidmanager_swift_transaction_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_queue_type_id() -> CFTypeID;
    pub fn iohidmanager_swift_keys_queue_enqueue_all() -> u32;
    pub fn iohidmanager_swift_keys_standard_type_ansi() -> u32;
    pub fn iohidmanager_swift_usage_page_generic_desktop() -> u32;
    pub fn iohidmanager_swift_usage_generic_desktop_keyboard() -> u32;
    pub fn iohidmanager_swift_usage_generic_desktop_mouse() -> u32;
    pub fn iohidmanager_swift_service_plugin_uuid(kind: u32, out_bytes: *mut u8) -> bool;
    pub fn iohidmanager_swift_event_keyboard_no_repeat() -> u32;
    pub fn iohidmanager_swift_event_sensor_control_decimation() -> u32;
}
