use iohidmanager::keys;

#[test]
fn keys_catalog_contains_expected_entries() {
    assert_eq!(keys::string_key("kIOHIDVendorIDKey"), Some(keys::VENDOR_ID_KEY));
    assert_eq!(keys::bridge_queue_enqueue_all(), keys::QUEUE_OPTIONS_TYPE_ENQUEUE_ALL);
    assert!(keys::ALL_STRING_KEYS.len() >= 100);
}
