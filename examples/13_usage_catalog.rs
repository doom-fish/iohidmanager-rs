use iohidmanager::usage;

fn main() {
    println!(
        "{} usage constants; generic desktop page={} keyboard={} mouse={}",
        usage::ALL_USAGE_CONSTANTS.len(),
        usage::PAGE_GENERIC_DESKTOP,
        usage::USAGE_KEYBOARD,
        usage::USAGE_MOUSE,
    );
}
