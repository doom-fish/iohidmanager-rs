# Changelog

## [0.5.0] - 2026-05-16

### Added

- Full raw FFI coverage for the current public `IOHIDManager`, `IOHIDDevice`, `IOHIDElement`, and `IOHIDValue` header functions.
- `DeviceMatch` and `ElementMatch` builders for custom and multi-dictionary matching.
- `HidReportType`, `HidValueScale`, `HidElementType`, and `HidCollectionType` helper enums.
- Safe `HidDevice` helpers for:
  - generic property reads/writes
  - report descriptor reads
  - synchronous `get_value` / `get_value_with_options` / `set_value`
  - synchronous `get_report` / `set_report`
  - timestamped report callbacks and input-value callbacks
  - filtered element enumeration
- `HidElement` wrappers for parent/child traversal, metadata access, device lookup, and generic property reads/writes.
- Owned `HidValue` wrapper with constructors and typed accessors.
- New smoke examples for matching/properties, element/value reads, and callback registration.
- Coverage tests for `IOHIDElement` and `IOHIDValue` in addition to manager/device coverage.

### Changed

- `HidDeviceInfo` and property decoding now accept full unsigned 32-bit values.
- Package contents now include examples and tests.
- README status and roadmap now reflect the shipped safe surface.

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
