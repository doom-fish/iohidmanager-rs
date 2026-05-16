use iohidmanager::keys;

fn main() {
    println!(
        "{} string keys, {} numeric constants; {} => {}",
        keys::ALL_STRING_KEYS.len(),
        keys::ALL_NUMERIC_CONSTANTS.len(),
        keys::DEVICE_KEY,
        keys::TRANSPORT_KEY,
    );
}
