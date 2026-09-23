mod common;

use iohidmanager::keys;

#[test]
fn keys_catalog_contains_expected_entries() {
    assert_eq!(keys::string_key("kIOHIDVendorIDKey"), Some(keys::VENDOR_ID_KEY));
    assert!(keys::ALL_STRING_KEYS.len() >= 100);
}

#[test]
fn queue_option_matches_sdk_header() {
    let header = common::hid_header("IOHIDKeys");
    assert_eq!(
        u64::from(keys::QUEUE_OPTIONS_TYPE_ENQUEUE_ALL),
        common::enum_value(&header, "kIOHIDQueueOptionsTypeEnqueueAll"),
    );
    assert_eq!(
        u64::from(keys::STANDARD_TYPE_ANSI),
        common::enum_value(&header, "kIOHIDStandardTypeANSI"),
    );
}
