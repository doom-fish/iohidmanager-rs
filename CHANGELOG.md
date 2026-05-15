# Changelog

## [0.1.0] - Initial release

### Added

- `HidManager::new()` — opens an `IOHIDManagerRef`. Drops on scope exit.
- `set_device_matching(Option<HidUsage>)` filters subsequent enumeration.
- `devices() -> Vec<HidDeviceInfo>` enumerates currently-matched devices.
- `HidUsage` enum: `Keyboard`, `Mouse`, `Joystick`, `GamePad`, `Custom(p, u)`.
- `HidDeviceInfo` snapshot with vendor_id, product_id, product,
  manufacturer, serial_number, transport, usage_page, usage, location_id.
- `HidError`: `ManagerCreateFailed`, `ManagerOpenFailed`, `InvalidArgument`.
- 2 examples (`01_list_devices`, `02_filter_keyboards`).
- 2 API-coverage tests (`IOHIDManager`, `IOHIDDevice`) using the
  C-function-regex harness pattern.

Pure C — zero-Swift bridge.
