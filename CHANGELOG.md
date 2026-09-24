# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.11.0] - Unreleased

### Security

- `HidManager::with_options` passed the manager-creation options to `IOHIDManagerOpen` as well. `USE_PERSISTENT_PROPERTIES` has the value of `kIOHIDOptionsTypeSeizeDevice`, so it seized every matched device and the rest of the system stopped receiving their input. Managers are now always opened with `kIOHIDOptionsTypeNone`.

### Fixed

- Use-after-free when a subscription was dropped from another thread or inside its own callback. Drop freed the context, closure and report buffer while IOKit could still call them. Subscriptions now give IOKit a doom-fish-utils `CallbackContext` and stop delivering as soon as they are dropped. They are torn down on the crate's event-loop thread, after any callback that is already running returns.
- IOKit cannot reliably unregister device input-value, input-report and removal callbacks: its callback sets compare only the context, but hash the callback pointer too. The device plug-in also keeps copying reports into the last registered buffer, and queue callbacks cannot be unregistered at all. Each subscription therefore owns a private `IOHIDManager` or `IOHIDDevice`, which is unscheduled and closed before the context and buffer are freed. A queue subscription replaces its context with NULL on teardown.
- Dropping one subscription unscheduled and closed the whole device or manager, which killed every other subscription on it. A second subscription on the same manager hit IOKit's already-scheduled assertion and aborted with SIGTRAP. That is the crash the async docs had blamed on missing device matching. Subscriptions no longer share IOKit objects.
- Async streams could hang in `drop` right after subscribing, because a `CFRunLoopStop` sent before the stream's thread started running was lost. Streams no longer own threads.
- The device streams turned an `IOHIDDeviceOpen` or Input Monitoring failure into a stream that silently closed. Subscribing now returns the error.
- `HidElement` was a `Copy` handle around a borrowed element: `elements()` released the array it borrowed from, and `parent()` and `element()` returned +0 references. Using an element after its device went away read freed memory. Elements now retain the element and its device. Values, queues and transactions also keep their device alive.
- `HidQueue::copy_next_value` and `copy_next_value_with_timeout` leaked one value per call. `copy_next_value_with_timeout` passed milliseconds where IOKit expects seconds.
- The `set_property_*` setters leaked the value when the key contained NUL, and converting a `CFString` used unchecked arithmetic.
- Device report buffers are sized from `kIOHIDMaxInputReportSizeKey` (64 bytes if absent, at most 1 MiB) instead of a caller-supplied length.
- README and `COVERAGE*.md` no longer describe the removed Swift bridge or claim safe wrappers for raw-only functions.

### Changed

- **Breaking:** callbacks run on a background event-loop thread owned by the crate, not on the run loop of the thread that subscribed. No caller run loop is needed.
- **Breaking:** manager subscriptions (`HidManager::on_*` and the manager streams) use a private manager built from the device and input-value matching set on the `HidManager` when you subscribe. Later matching changes do not affect live subscriptions. Without `set_device_matching` they return `HidError::InvalidArgument`.
- **Breaking:** device subscriptions use a private device for the same service and no longer open or close the caller's device. `on_removal` does not open a device at all.
- **Breaking:** `HidDevice::on_input_report` and `on_input_report_with_timestamp` no longer take a `max_report_length`.
- **Breaking:** `HidDevice::on_input_value` takes the element matching as `&[ElementMatch]`, and so does `DeviceInputValueStream::subscribe`.
- **Breaking:** `HidQueue::on_value_available` returns `HidError::InvalidArgument` while another subscription on the same queue is alive.
- **Breaking:** every `async_api` `subscribe` returns `Result<Self, HidError>`, and a capacity of zero is `InvalidArgument` instead of a panic.
- **Breaking:** `activate` and `cancel` on `HidManager`, `HidDevice` and `HidQueue` are `unsafe fn`. IOKit aborts unless a dispatch queue was set first, and only the raw `ffi` module can set one.
- **Breaking:** `HidQueue` and `HidTransaction` no longer implement `Clone`; a clone shared one mutable IOKit object.
- **Breaking:** `HidElement` is no longer `Copy`. `parent`, `children`, `attached` and `HidValue::element` return retained handles; `HidValue::from_integer` and `from_bytes` take `&HidElement`; and `HidElement::attach` returns `Result` and rejects elements of another device.
- **Breaking:** `HidQueue::copy_next_value_with_timeout` takes a `Duration`.
- **Breaking:** `HidManager`, `HidDevice`, `HidElement`, `HidQueue` and `HidTransaction` are no longer `Sync`, because their setters take `&self`.
- The subscription types and async streams are `Send + Sync` without `unsafe impl`.
- The crate's own Swift bridge is gone; it calls IOKit directly.
- `rust-version` is 1.82. `apple-cf` is required at `>=0.11, <0.12`, `iokit` at `>=0.6, <0.7` and `doom-fish-utils` at `>=0.4.1, <0.5`.

### Added

- `ffi::kIOHIDMaxInputReportSizeKey`.

### Removed

- **Breaking:** `HidDevice::set_input_value_matching` and `set_input_value_matching_multiple`: a subscription's private device cannot see them. Pass the matching to `on_input_value` instead.
- **Breaking:** `HidQueue::schedule_current_run_loop` and `unschedule_from_run_loop`: scheduling a queue twice aborts, and `on_value_available` schedules it on the event loop.
- **Breaking:** the Swift bridge stubs and their helpers: `keys::bridge_queue_enqueue_all`, `keys::bridge_standard_type_ansi`, `usage::bridge_generic_desktop_page`, `usage::bridge_keyboard_usage`, `usage::bridge_mouse_usage`, `event_system::bridge_keyboard_no_key_repeat`, `event_system::bridge_sensor_control_decimation`, `ServicePlugInUuid::bridge_bytes`, and the `iohidmanager_swift_*` raw declarations.

## [0.10.4] - 2026-06-06

- Contain panics from user callbacks in every callback trampoline instead of unwinding across the FFI boundary, and drop the empty Swift bridge header.

## [0.10.3] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.10.2] - 2026-05-18

### Changed

- Added one-line rustdoc coverage across the safe `src/` surface, mirroring the corresponding `IOHID*`, `kIOHID*`, and HID usage-table counterparts throughout the manager, device, element, value, queue, transaction, key, usage, and event-system APIs.

## [0.10.1] - 2026-05-18

### Changed

- chore: re-export OS primitives (Boolean) from apple-cf

## [0.10.0] - 2026-05-18

### Changed

- **Breaking — shared IOKit HID raw types**: the raw `ffi` module now re-exports `IOHIDManagerRef`, `IOHIDDeviceRef`, `IOHIDElementRef`, `IOHIDValueRef`, `IOReturn`, `IOOptionBits`, and `IOHIDReportType` from `iokit::ffi` instead of maintaining local duplicate definitions, so downstream users see the shared `iokit` ABI surface for those seven IOHID types.
- Added an explicit `iokit` path dependency with version requirement `>=0.4, <0.6` for the shared raw IOHID type surface used by `ffi`.

## [0.9.0] - 2026-05-18

### Changed

- **Breaking — shared Core Foundation raw types**: the raw `ffi` module now re-exports `CFIndex`, `CFTypeID`, `CFTimeInterval`, and `CFUUIDBytes` from `apple_cf::raw` instead of maintaining local duplicate definitions, so downstream users see the shared `apple-cf` Core Foundation ABI surface for all four types.
- Widened the `apple-cf` dependency requirement to `>=0.9, <0.10` for the shared raw Core Foundation type surface.

## [0.8.0] - 2026-05-18

### Changed

- **Breaking — Core Foundation raw typedefs**: the raw `ffi` module now re-exports 11 `CF*Ref` aliases from `apple_cf::raw` instead of maintaining local duplicate `*const c_void` / `*mut c_void` definitions, so downstream users see the shared opaque Core Foundation pointer types from `apple-cf`.
- Added an explicit `apple-cf` dependency for the shared Core Foundation raw typedef surface used by `ffi`.

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
