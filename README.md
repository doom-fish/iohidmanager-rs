# iohidmanager

Safe Rust bindings for Apple's [IOKit HID](https://developer.apple.com/documentation/iokit/iohidmanager_h) subsystem on macOS — enumerate connected mice, keyboards, gamepads, and other HID devices.

> **Status:** actively developed. v0.7 adds an executor-agnostic `async` feature with 7 stream types wrapping every IOKit HID callback API; v0.6 ships a Swift bridge for `IOKit/hid`, keeps the raw C surface behind the `raw-ffi` feature (enabled by default), and adds logical-area coverage for Manager, Device, Element, Value, Transaction, Queue, Keys, Usage, ServicePlugIn, and EventSystem.

## Quick start

```rust,no_run
use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;

    for d in manager.devices() {
        println!(
            "[{:04x}:{:04x}] {} — {} ({})",
            d.vendor_id.unwrap_or(0),
            d.product_id.unwrap_or(0),
            d.manufacturer.as_deref().unwrap_or("?"),
            d.product.as_deref().unwrap_or("?"),
            d.transport.as_deref().unwrap_or("?"),
        );
    }

    let kb_or_mouse = [
        DeviceMatch::usage(HidUsage::Keyboard),
        DeviceMatch::usage(HidUsage::Mouse),
    ];
    manager.set_device_matching_multiple(&kb_or_mouse)?;
    println!("{} keyboards/mice connected", manager.devices().len());
    Ok(())
}
```

## Highlights in v0.7

- `async` Cargo feature exposing 7 executor-agnostic stream types wrapping every `IOKit` HID callback API.
- Each stream spawns a dedicated background `CFRunLoop` thread — no async runtime required.
- Stream types: `ManagerInputValueStream`, `ManagerDeviceMatchingStream`, `ManagerDeviceRemovalStream`, `ManagerInputReportStream`, `DeviceRemovalStream`, `DeviceInputValueStream`, `QueueValueStream`.

## Highlights in v0.6

- Swift bridge target under `swift-bridge/` with one Swift file per logical area.
- `raw-ffi` Cargo feature (enabled by default) re-exporting the raw `IOKit/hid` declarations.
- New safe transaction and queue wrappers: `HidTransaction`, `HidTransactionDirection`, `HidQueue`, and `HidQueueOptions`.
- Manager matching helpers for input-value filtering.
- Device helpers for multi-value reads/writes, removal callbacks, and timeout-based value/report setters.
- Element attach/detach helpers.
- `unsafe { HidValue::from_bytes_no_copy(...) }` bridged through Swift.
- Generated `keys` and `usage` catalogs, event-system constants, and service-plugin UUID helpers.
- 15 runnable examples and per-area smoke tests.

## Feature flags

- `raw-ffi` *(default)* — re-export the raw `iohidmanager::ffi` module.
- `async` — enable the `async_api` module with 7 executor-agnostic stream types.

Disable default features if you only want the safe surface:

```bash
cargo add iohidmanager --no-default-features
```

## Area guide

- **Manager** — device matching, properties, enumeration, and manager-level filtering.
- **Device** — properties, reports, synchronous values, multi-value helpers, and removal subscriptions.
- **Element** — hierarchy traversal, metadata, and attach/detach relationships.
- **Value** — integer/byte constructors, no-copy bridge constructor, typed accessors.
- **Transaction** — batched input/output element transactions.
- **Queue** — buffered value delivery and queue depth management.
- **Keys** — generated string/numeric key catalog from `IOHIDKeys.h`, `IOHIDDeviceKeys.h`, and `IOHIDProperties.h`.
- **Usage** — generated usage/page constant catalog from `IOHIDUsageTables.h`.
- **ServicePlugIn** — UUID helpers for the `IOHIDDevicePlugIn.h` interfaces.
- **EventSystem** — `IOHIDEventServiceKeys.h` and `IOHIDEventServiceTypes.h` constants.

## Examples

The crate ships 17 examples, including:

- `06_manager_callbacks`
- `07_device_roundtrip`
- `08_element_attachment`
- `09_value_constructors`
- `10_transaction_roundtrip`
- `11_queue_roundtrip`
- `12_keys_catalog`
- `13_usage_catalog`
- `14_service_plugin_ids`
- `15_event_system_catalog`
- `17_async_stream` (requires `--features async`)

Run them all with:

```bash
for ex in examples/*.rs; do cargo run --example "$(basename "$ex" .rs)"; done
```

## Pipeline composition

```text
iohidmanager (enumerate + inspect + subscribe) ──► your custom dispatch
                                               │
                                               ├─► gamepad-mapper
                                               ├─► macropad-driver
                                               └─► hardware-token authentication
```

Pairs naturally with [`cgevents`](https://github.com/doom-fish/cgevents-rs) (synthesise events triggered by HID input) and [`carbonhotkey`](https://github.com/doom-fish/carbonhotkey-rs) (more focused: just global hotkeys).

## Roadmap

- [x] `HidManager::new()` + open
- [x] `DeviceMatch` / `ElementMatch` builders + multi-match dictionaries
- [x] device enumeration and live device handles
- [x] report/value callbacks on devices
- [x] manager input-value matching helpers
- [x] `HidTransaction` and `HidQueue`
- [x] `unsafe HidValue::from_bytes_no_copy`
- [x] generated keys/usage catalogs
- [x] service-plugin UUID helpers and event-system constants
- [x] Swift bridge + `raw-ffi` feature split

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
