mod common;

use iohidmanager::usage;

#[test]
fn usage_catalog_contains_common_entries() {
    let mouse = usage::constant("kHIDUsage_GD_Mouse").expect("mouse usage");
    assert_eq!(mouse.value, usage::USAGE_MOUSE);
}

#[test]
fn usage_constants_match_sdk_header() {
    let header = common::hid_header("IOHIDUsageTables");
    for (symbol, value) in [
        ("kHIDPage_GenericDesktop", usage::PAGE_GENERIC_DESKTOP),
        ("kHIDUsage_GD_Keyboard", usage::USAGE_KEYBOARD),
        ("kHIDUsage_GD_Mouse", usage::USAGE_MOUSE),
    ] {
        assert_eq!(
            u64::from(value),
            common::enum_value(&header, symbol),
            "{symbol}"
        );
    }
}
