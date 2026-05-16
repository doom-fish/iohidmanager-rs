# Changelog

## [0.6.0] - 2026-05-16

### Added

- Swift bridge build pipeline (`swift-bridge/`) with one bridge file per logical HID area.
- `raw-ffi` feature (enabled by default) for the raw `iohidmanager::ffi` surface.
- New safe logical areas and exports:
  - `HidTransaction`, `HidTransactionDirection`, and `HidTransactionOptions`
  - `HidQueue`, `HidQueueOptions`, and queue value-available subscriptions
  - generated `keys` and `usage` catalogs
  - service-plugin UUID helpers and event-system constants
- Manager helpers for input-value matching configuration.
- Device helpers for removal subscriptions, multi-value reads/writes, and timeout-based setters.
- `HidElement::attach` / `detach`.
- `unsafe HidValue::from_bytes_no_copy` via the Swift bridge.
- 10 new numbered examples covering the new logical areas.
- 10 new per-area smoke tests plus queue/transaction additions to API coverage validation.
- `COVERAGE.md` documenting header coverage across the HID framework areas.

### Changed

- The crate now builds and links a small Swift bridge alongside the Rust library.
- README now documents the v0.6 logical-area split and `raw-ffi` feature.
- Package contents now include the Swift bridge and coverage document.

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
