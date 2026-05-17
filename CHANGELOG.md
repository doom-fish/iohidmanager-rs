# Changelog

## [0.7.1] - 2026-05-17

### Fixed

- **Panic safety**: all four synchronous-callback trampolines in `hid/manager.rs`
  (`manager_device_trampoline`, `manager_report_trampoline`,
  `manager_timestamped_report_trampoline`, `manager_value_trampoline`) now
  wrap user-closure invocations in
  `doom_fish_utils::panic_safe::catch_user_panic`, preventing UB from a panic
  crossing the `extern "C"` boundary.
- **Documentation — `set_device_matching` prerequisite**: the `async_api`
  module doc and all four manager-level `subscribe()` methods
  (`ManagerInputValueStream`, `ManagerDeviceMatchingStream`,
  `ManagerDeviceRemovalStream`, `ManagerInputReportStream`) now carry an
  explicit `# Prerequisites` section documenting that
  `HidManager::set_device_matching` (or a variant) **must** be called before
  subscribing, to avoid the `SIGTRAP` IOKit raises when a manager is scheduled
  on a run loop without a device-matching filter.
- **SAFETY comments**: added `// SAFETY:` comments to every raw-pointer
  dereference inside the seven `async_api` stream callbacks.

## [0.7.0] - 2026-05-17

### Added

- `async` Cargo feature (disabled by default) exposing the new `async_api` module.
- 7 executor-agnostic async stream types, each wrapping an `IOKit` HID callback API
  as a `BoundedAsyncStream<T>` (from `doom-fish-utils`):
  - `ManagerInputValueStream` — `IOHIDManagerRegisterInputValueCallback`
  - `ManagerDeviceMatchingStream` — `IOHIDManagerRegisterDeviceMatchingCallback`
  - `ManagerDeviceRemovalStream` — `IOHIDManagerRegisterDeviceRemovalCallback`
  - `ManagerInputReportStream` — `IOHIDManagerRegisterInputReportCallback`
  - `DeviceRemovalStream` — `IOHIDDeviceRegisterRemovalCallback`
  - `DeviceInputValueStream` — `IOHIDDeviceRegisterInputValueCallback`
  - `QueueValueStream` — `IOHIDQueueRegisterValueAvailableCallback`
- Each stream spawns a dedicated background `CFRunLoop` thread; no async runtime is required.
- `HidDevice::from_raw_retained` (pub crate) helper for constructing owned device handles from borrowed IOKit refs.
- `CFRunLoopRun` / `CFRunLoopStop` FFI declarations.

## [0.6.1] - 2026-05-16

### Added

- Public low-level SDK type aliases for manager options, device get-value options, event-system option enums, and key typedefs.
- Raw/bridged coverage for `IOHIDDevicePlugIn.h` vtable structs, `IOHIDCompletion`, report-command enums, element flag/value-option masks, and manager option flags.
- A smoke example (`16_low_level_sdk_types`) plus coverage tests for the newly-exposed low-level surfaces.

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
