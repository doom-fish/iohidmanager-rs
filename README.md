# iohidmanager

Safe Rust bindings for Apple's [IOKit HID](https://developer.apple.com/documentation/iokit/iohidmanager_h) subsystem on macOS — enumerate connected mice, keyboards, gamepads, and other HID devices.

> **Status:** actively developed. v0.11 delivers every callback on a background event-loop thread owned by the crate, gives each subscription its own `IOKit` manager or device object, and makes subscribing fallible instead of hiding open and permission failures.

## Installation

```toml
[dependencies]
iohidmanager = "0.11"
```

## Requirements

- macOS 10.15 or later, the minimum of the `iokit` dependency. The crate calls `IOKit` directly; its `iokit` dependency builds a small Swift bridge, so building needs the Xcode command-line tools.
- Reading input from keyboards and keypads needs the **Input Monitoring** permission (System Settings → Privacy & Security). Without it, opening those devices fails with `kIOReturnNotPermitted`, and subscriptions that need an open device return `HidError::DeviceOpenFailed` or `HidError::ManagerOpenFailed`. Enumerating devices and watching them arrive or leave works without it.

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

## Callbacks and subscriptions

```rust,no_run
use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(Some(HidUsage::GamePad))?;

    let arrivals = manager.on_device_matching(|device| {
        println!("attached: {:?}", device.info().product);
    })?;

    std::thread::sleep(std::time::Duration::from_secs(5));
    drop(arrivals);
    Ok(())
}
```

- Callbacks run on one background thread that the crate starts on first use, so no caller run loop has to run. Keep them short: a callback that blocks delays every other subscription.
- Every subscription opens its own `IOKit` object: manager subscriptions use a private `IOHIDManager` built from the device and input-value matching set on the `HidManager` at the time you subscribe, and device subscriptions use a private `IOHIDDevice` for the same service. Subscriptions therefore never replace or unschedule each other, and dropping one never closes the device or manager you hold.
- Manager subscriptions need `set_device_matching` (or `_dict` / `_multiple`) first; otherwise they return `HidError::InvalidArgument`. `None` matches every device.
- Dropping a subscription stops delivery at once. The `IOKit` objects and the callback are released on the event-loop thread after any callback that is already running returns, so dropping never waits and may happen inside the callback itself.
- Subscriptions never take exclusive access. Seize a device only explicitly, with `HidDevice::open_with_options(keys::OPTIONS_TYPE_SEIZE_DEVICE)`.
- `HidQueue::on_value_available` allows one subscription per queue at a time.
- `HidManager`, `HidDevice`, `HidElement`, `HidValue`, `HidQueue` and `HidTransaction` are neither `Send` nor `Sync`, so each stays on the thread that created it. `IOKit` does not lock these objects: `IOHIDDevice` and `IOHIDManager` take no locks (or, for the manager, only around its device set) through macOS 12, and `IOHIDElement`, `IOHIDQueue` and `IOHIDTransaction` never do, while calls that look read-only, such as `IOHIDDeviceGetProperty` and `IOHIDDeviceCopyMatchingElements`, update internal caches. A device passed to a callback belongs to the event-loop thread; to use it elsewhere, send `device.id()` and open the device there with `HidDevice::from_id`. The async streams deliver plain data: `HidDeviceId`, `HidDeviceInfo`, `HidValueInfo` and `HidInputReport`.
- The dispatch-queue functions (`activate`, `cancel`) are `unsafe`: `IOKit` aborts the process unless a dispatch queue was set first, which only the raw `ffi` module can do.

The `async` feature wraps the same subscriptions as executor-agnostic streams: `ManagerInputValueStream`, `ManagerDeviceMatchingStream`, `ManagerDeviceRemovalStream`, `ManagerInputReportStream`, `DeviceRemovalStream`, `DeviceInputValueStream` and `QueueValueStream`.

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
- **Value** — integer/byte constructors, `unsafe` no-copy constructor, typed accessors.
- **Transaction** — batched input/output element transactions.
- **Queue** — buffered value delivery and queue depth management.
- **Keys** — generated string/numeric key catalog from `IOHIDKeys.h`, `IOHIDDeviceKeys.h`, and `IOHIDProperties.h`.
- **Usage** — generated usage/page constant catalog from `IOHIDUsageTables.h`.
- **Service plug-in** — UUID helpers for the `IOHIDDevicePlugIn.h` interfaces.
- **Event system** — `IOHIDEventServiceKeys.h` and `IOHIDEventServiceTypes.h` constants.

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
- [x] `raw-ffi` feature split
- [x] callbacks on a crate-owned event loop with per-subscription `IOKit` objects
- [ ] the macOS 15 `CoreHID` framework (not wrapped)

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
