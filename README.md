# iohidmanager

Safe Rust bindings for Apple's [IOKit HID](https://developer.apple.com/documentation/iokit/iohidmanager_h) subsystem on macOS — enumerate connected mice, keyboards, gamepads, and other HID devices.

> **Status:** actively developed. v0.5 ships the current public `IOHIDManager` / `IOHIDDevice` / `IOHIDElement` / `IOHIDValue` C header surface in raw FFI, plus safe wrappers for multi-match dictionaries, element discovery, report I/O, value access, report-descriptor reads, and live input callbacks.

Pure C — **zero Swift bridge** (like `cgevents`, `imageio`, `videotoolbox`).

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
- [x] `set_device_matching(Option<HidUsage>)` convenience filter
- [x] `DeviceMatch` / `ElementMatch` builders + multi-match dictionaries
- [x] `devices()` enumeration with vendor/product/usage/serial/transport snapshots
- [x] `live_devices()` handles for per-device work
- [x] Live input-report callbacks (`on_input_report`, `on_input_report_with_timestamp`)
- [x] Live input-value callbacks (`on_input_value`)
- [x] Element discovery + metadata (`elements`, `matching_elements`, `HidElement` accessors)
- [x] `HidValue` creation + synchronous reads (`get_value`, `get_value_with_options`)
- [x] Synchronous report read / write (`get_report`, `set_report`)
- [x] Generic property helpers + report-descriptor reads
- [ ] Dispatch-queue convenience wrappers
- [ ] Safe manager-level add/remove/value/report callback wrappers

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
