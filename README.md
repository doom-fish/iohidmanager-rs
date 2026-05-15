# iohidmanager

Safe Rust bindings for Apple's [IOKit HID](https://developer.apple.com/documentation/iokit/iohidmanager_h) subsystem on macOS — enumerate connected mice, keyboards, gamepads, and other HID devices.

> **Status:** experimental. v0.1 ships device enumeration + property snapshots (vendor / product / manufacturer / usage page / serial / transport). Live input-report subscription via `IOHIDDeviceRegisterInputReportCallback` lands in v0.2.

Pure C — **zero Swift bridge** (like `cgevents`, `imageio`, `videotoolbox`).

## Quick start

```rust,no_run
use iohidmanager::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manager = HidManager::new()?;
    manager.set_device_matching(None)?;  // all devices

    for d in manager.devices() {
        println!(
            "[{:04x}:{:04x}] {} — {} ({})",
            d.vendor_id.unwrap_or(0),
            d.product_id.unwrap_or(0),
            d.manufacturer.as_deref().unwrap_or("?"),
            d.product.as_deref().unwrap_or("?"),
            d.transport.as_deref().unwrap_or("?")
        );
    }

    // Filter to keyboards only.
    let kb = HidManager::new()?;
    kb.set_device_matching(Some(HidUsage::Keyboard))?;
    println!("\n{} keyboards connected:", kb.devices().len());
    for d in kb.devices() {
        println!("  - {}", d.product.as_deref().unwrap_or("?"));
    }
    Ok(())
}
```

## Pipeline composition

```text
iohidmanager (enumerate) ──► your custom dispatch
                              │
                              ├─► gamepad-mapper
                              ├─► macropad-driver
                              └─► hardware-token authentication
```

Pairs naturally with [`cgevents`](https://github.com/doom-fish/cgevents-rs) (synthesise events triggered by HID input) and [`carbonhotkey`](https://github.com/doom-fish/carbonhotkey-rs) (more focused: just global hotkeys).

## Roadmap

- [x] `HidManager::new()` + open
- [x] `set_device_matching(Option<HidUsage>)` filter
- [x] `devices()` enumeration with vendor/product/usage/serial/transport
- [x] `HidUsage` enum with Keyboard / Mouse / Joystick / `GamePad` presets
- [ ] Live input-report callbacks (`IOHIDDeviceRegisterInputReportCallback`)
- [ ] Element discovery (`IOHIDDeviceCopyMatchingElements` + `IOHIDElementGetUsage`)
- [ ] Synchronous report read / write (`IOHIDDeviceGetReport` / `SetReport`)
- [ ] Run-loop integration (`IOHIDManagerScheduleWithRunLoop`)
- [ ] Async API

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
