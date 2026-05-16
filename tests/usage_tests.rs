use iohidmanager::usage;

#[test]
fn usage_catalog_contains_common_entries() {
    let mouse = usage::constant("kHIDUsage_GD_Mouse").expect("mouse usage");
    assert_eq!(mouse.value, usage::USAGE_MOUSE);
    assert_eq!(usage::bridge_generic_desktop_page(), usage::PAGE_GENERIC_DESKTOP);
}
