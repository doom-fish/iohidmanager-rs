use iohidmanager::event_system;

fn main() {
    println!(
        "{} event-system keys, keyboard-no-repeat=0x{:x}",
        event_system::ALL_EVENT_SYSTEM_KEYS.len(),
        event_system::KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT,
    );
}
