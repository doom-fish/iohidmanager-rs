use iohidmanager::prelude::*;

#[test]
fn transaction_create_and_mutate() {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    let Some(device) = manager.live_devices().into_iter().next() else {
        return;
    };
    let transaction = HidTransaction::new(&device, HidTransactionDirection::Input).expect("transaction");
    if let Some(element) = device.elements().into_iter().next() {
        transaction.add_element(&element);
        assert!(transaction.contains_element(&element));
        transaction.remove_element(&element);
    }
    transaction.clear();
}
