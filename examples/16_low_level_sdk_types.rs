use std::mem::size_of;

use iohidmanager::{event_system, hid, keys, service_plugin, usage, HidManagerOptions};

fn main() {
    let manager_options =
        HidManagerOptions::USE_PERSISTENT_PROPERTIES | HidManagerOptions::DO_NOT_SAVE_PROPERTIES;

    println!("manager options bits: {:#x}", manager_options.bits());
    println!(
        "device get-value with update: {:#x}",
        hid::DEVICE_GET_VALUE_WITH_UPDATE
    );
    println!(
        "value update option: {:#x}",
        hid::VALUE_OPTIONS_UPDATE_ELEMENT_VALUES
    );
    println!(
        "scroll accel curves key: {}",
        event_system::SCROLL_ACCEL_PARAMETRIC_CURVES_KEY
    );
    println!(
        "vendor specific key: {}",
        keys::string_key("kIOHIDElementVendorSpecificKey")
            .unwrap_or(keys::ELEMENT_VENDOR_SPECIFIC_KEY)
    );
    println!(
        "dark boot message: {}",
        keys::numeric_constant("kIOHIDSystemButtonPressedDuringDarkBoot")
            .unwrap_or(keys::SYSTEM_BUTTON_PRESSED_DURING_DARK_BOOT)
    );
    println!(
        "system menu usage: {}",
        usage::constant("kHIDUsage_GD_SystemMenu")
            .map_or(usage::USAGE_SYSTEM_MENU, |usage| usage.value)
    );
    println!(
        "timestamped interface size: {}",
        size_of::<service_plugin::IOHIDDeviceTimeStampedDeviceInterface>()
    );
}
