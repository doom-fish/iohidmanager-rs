# COVERAGE

Audit target: `IOKit.framework/Headers/hid/*` from the macOS 26.2 SDK.

Legend:

- ✅ implemented: reachable through the safe API
- 🟡 partial, 🟡 unsafe (safe wrapper is an `unsafe fn`), 🟡 internal (used only inside subscriptions), 🟡 raw only (declared in `iohidmanager::ffi` behind the default `raw-ffi` feature, no safe wrapper)
- ⏭️ skipped

## Summary

- Safe logical areas shipped in Rust: Manager, Device, Element, Value, Transaction, Queue, Keys, Usage, ServicePlugIn, EventSystem.
- Raw C declarations remain available behind the default-enabled `raw-ffi` feature (`iohidmanager::ffi`).
- `tests/api_coverage.rs` checks that every C function in the installed SDK's manager, device, element, value, transaction and queue headers has a raw declaration; that is raw-FFI coverage, not safe coverage. The rows below say which functions also have a safe wrapper.
- Every subscription (`on_*`, `async_api` streams) opens a private `IOHIDManager` or `IOHIDDevice` scheduled on the crate's event-loop thread; teardown unschedules and closes it there before the callback context is released.
- The handle types (`HidManager`, `HidDevice`, `HidElement`, `HidValue`, `HidQueue`, `HidTransaction`) are neither `Send` nor `Sync`, because IOKit does not lock the objects they wrap on macOS 12 and earlier (and never locks elements, queues or transactions). Compile-time assertions in `src/hid/thread_safety.rs` keep it that way.
- There is no Swift bridge; the stub bridge was removed in 0.11.
- Not covered: the `CoreHID` framework (macOS 15).
- Deferred: `IOHIDDevicePlugIn.h` COM-style vtable structs are not surfaced as safe Rust types; UUID discovery helpers are provided instead.

## Manager

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDManagerOptions` | ✅ implemented | Wrapped by `HidManagerOptions`; `HidManager::with_options` passes the bits to `IOHIDManagerCreate` only and always opens with `kIOHIDOptionsTypeNone`. |
| `IOHIDManagerRef` | ✅ implemented | Raw CFTypeRef available through `raw-ffi`; safe owner is `HidManager`. |
| `IOHIDManagerActivate` | 🟡 unsafe | `unsafe fn HidManager::activate`: IOKit aborts unless a dispatch queue was set, which needs the raw `ffi` module. |
| `IOHIDManagerCancel` | 🟡 unsafe | `unsafe fn HidManager::cancel`, for the same reason as `IOHIDManagerActivate`. |
| `IOHIDManagerClose` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerCopyDevices` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerCreate` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerGetProperty` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerOpen` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterDeviceMatchingCallback` | ✅ implemented | `HidManager::on_device_matching`; each subscription registers on a private manager scheduled on the crate event loop. |
| `IOHIDManagerRegisterDeviceRemovalCallback` | ✅ implemented | `HidManager::on_device_removal`; private manager on the crate event loop. |
| `IOHIDManagerRegisterInputReportCallback` | ✅ implemented | `HidManager::on_input_report`; private manager on the crate event loop. |
| `IOHIDManagerRegisterInputReportWithTimeStampCallback` | ✅ implemented | `HidManager::on_input_report_with_timestamp`; private manager on the crate event loop. |
| `IOHIDManagerRegisterInputValueCallback` | ✅ implemented | `HidManager::on_input_value`; private manager on the crate event loop. |
| `IOHIDManagerSaveToPropertyDomain` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerScheduleWithRunLoop` | 🟡 internal | Used only by subscriptions, which schedule their private managers on the crate event loop; there is no public scheduling call. |
| `IOHIDManagerSetCancelHandler` | 🟡 raw only | Raw declaration only (takes a block). |
| `IOHIDManagerSetDeviceMatching` | ✅ implemented | `HidManager::set_device_matching` / `set_device_matching_dict`, issued through `IOHIDManagerSetDeviceMatchingMultiple`. |
| `IOHIDManagerSetDeviceMatchingMultiple` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetDispatchQueue` | 🟡 raw only | Raw declaration only. |
| `IOHIDManagerSetInputValueMatching` | ✅ implemented | `HidManager::set_input_value_matching`, issued through `IOHIDManagerSetInputValueMatchingMultiple`. |
| `IOHIDManagerSetInputValueMatchingMultiple` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetProperty` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerUnscheduleFromRunLoop` | 🟡 raw only | Raw declaration only; subscriptions unschedule through `IOHIDManagerClose`. |

## Device

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDDeviceRef` | ✅ implemented | Raw handle via `raw-ffi`; safe owner is `HidDevice`. |
| `IOHIDDeviceActivate` | 🟡 unsafe | `unsafe fn HidDevice::activate`: IOKit aborts unless a dispatch queue was set, which needs the raw `ffi` module. |
| `IOHIDDeviceCancel` | 🟡 unsafe | `unsafe fn HidDevice::cancel`, for the same reason as `IOHIDDeviceActivate`. |
| `IOHIDDeviceClose` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceConformsTo` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyMatchingElements` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyValueMultiple` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyValueMultipleWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCreate` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetProperty` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetReport` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetReportWithCallback` | 🟡 raw only | Raw declaration only; the call writes the caller buffer asynchronously. |
| `IOHIDDeviceGetService` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetValue` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetValueWithCallback` | ✅ implemented | `HidDevice::get_value_with_timeout`; IOKit documents the timeout as unused and the call as synchronous. |
| `IOHIDDeviceGetValueWithOptions` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceOpen` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceRegisterInputReportCallback` | ✅ implemented | `HidDevice::on_input_report`; registered on a private `IOHIDDevice` for the same service, with a buffer sized from `kIOHIDMaxInputReportSizeKey`. |
| `IOHIDDeviceRegisterInputReportWithTimeStampCallback` | ✅ implemented | `HidDevice::on_input_report_with_timestamp`; private device, buffer sized from `kIOHIDMaxInputReportSizeKey`. |
| `IOHIDDeviceRegisterInputValueCallback` | ✅ implemented | `HidDevice::on_input_value`; private device on the crate event loop. |
| `IOHIDDeviceRegisterRemovalCallback` | ✅ implemented | `HidDevice::on_removal`; private device on the crate event loop, not opened. |
| `IOHIDDeviceScheduleWithRunLoop` | 🟡 internal | Used only by device subscriptions on their private devices; there is no public scheduling call. |
| `IOHIDDeviceSetCancelHandler` | 🟡 raw only | Raw declaration only (takes a block). |
| `IOHIDDeviceSetDispatchQueue` | 🟡 raw only | Raw declaration only. |
| `IOHIDDeviceSetInputValueMatching` | 🟡 raw only | Raw declaration only; `HidDevice::on_input_value` takes the element matches and applies them with `IOHIDDeviceSetInputValueMatchingMultiple`. |
| `IOHIDDeviceSetInputValueMatchingMultiple` | ✅ implemented | Applied to the private device of `HidDevice::on_input_value(matching, ..)`. |
| `IOHIDDeviceSetProperty` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetReport` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetReportWithCallback` | ✅ implemented | `HidDevice::set_report_with_timeout`. IOKit sends the report asynchronously, so the result only says whether it was queued. |
| `IOHIDDeviceSetValue` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueMultiple` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueMultipleWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueWithCallback` | ✅ implemented | `HidDevice::set_value_with_timeout`; IOKit documents the timeout as unused and the call as synchronous. |
| `IOHIDDeviceUnscheduleFromRunLoop` | 🟡 internal | Used only by device subscription teardown. |

## Element

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDElementRef` | ✅ implemented | Raw handle via `raw-ffi`; safe view is `HidElement`. |
| `IOHIDElementAttach` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementCopyAttached` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementCreateWithDictionary` | 🟡 raw only | Raw declaration only. |
| `IOHIDElementDetach` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetChildren` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetCollectionType` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetCookie` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetDevice` | ✅ implemented | `HidElement::device` returns the retained owning device instead of the +0 `IOHIDElementGetDevice` result. |
| `IOHIDElementGetLogicalMax` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetLogicalMin` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetName` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetParent` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetPhysicalMax` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetPhysicalMin` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetProperty` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetReportCount` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetReportID` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetReportSize` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetType` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetUnit` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetUnitExponent` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetUsage` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetUsagePage` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementHasNullState` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementHasPreferredState` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementIsArray` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementIsNonLinear` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementIsRelative` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementIsVirtual` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementIsWrapping` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementSetProperty` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |

## Value

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDValueRef` | ✅ implemented | Raw handle via `raw-ffi`; safe owner is `HidValue`. |
| `IOHIDValueCreateWithBytes` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueCreateWithBytesNoCopy` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueCreateWithIntegerValue` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetBytePtr` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetElement` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetIntegerValue` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetLength` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetScaledValue` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetTimeStamp` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |
| `IOHIDValueGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidValue / hid::value`; raw declaration available through `raw-ffi`. |

## Transaction

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDTransactionOptions` | ✅ implemented | Wrapped by `HidTransactionOptions` and re-exported in raw-ffi. |
| `IOHIDTransactionRef` | ✅ implemented | Safe owner is `HidTransaction`. |
| `IOHIDTransactionAddElement` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionClear` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionCommit` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionCommitWithCallback` | ✅ implemented | `HidTransaction::commit_with_timeout`. IOKit commits asynchronously, so the result only says whether the commit was queued. |
| `IOHIDTransactionContainsElement` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionCreate` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionGetDevice` | ✅ implemented | `HidTransaction::device` returns the retained device. |
| `IOHIDTransactionGetDirection` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionGetTypeID` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionGetValue` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionRemoveElement` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionScheduleWithRunLoop` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionSetDirection` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionSetValue` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionUnscheduleFromRunLoop` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |

## Queue

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDQueueRef` | ✅ implemented | Safe owner is `HidQueue`. |
| `IOHIDQueueActivate` | 🟡 unsafe | `unsafe fn HidQueue::activate`: IOKit aborts unless a dispatch queue was set, which needs the raw `ffi` module. |
| `IOHIDQueueAddElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCancel` | 🟡 unsafe | `unsafe fn HidQueue::cancel`, for the same reason as `IOHIDQueueActivate`. |
| `IOHIDQueueContainsElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCopyNextValue` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCopyNextValueWithTimeout` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCreate` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueGetDepth` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueGetDevice` | ✅ implemented | `HidQueue::device` returns the retained device; `IOHIDQueueCreate` does not retain it. |
| `IOHIDQueueGetTypeID` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueRegisterValueAvailableCallback` | ✅ implemented | `HidQueue::on_value_available`, one subscription per queue at a time. IOKit cannot unregister a queue callback, so teardown re-registers it with a NULL context. |
| `IOHIDQueueRemoveElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueScheduleWithRunLoop` | 🟡 internal | Used only by `HidQueue::on_value_available` on the crate event loop; the public scheduling methods were removed because scheduling twice aborts. |
| `IOHIDQueueSetCancelHandler` | 🟡 raw only | Raw declaration only (takes a block). |
| `IOHIDQueueSetDepth` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueSetDispatchQueue` | 🟡 raw only | Raw declaration only. |
| `IOHIDQueueStart` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueStop` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueUnscheduleFromRunLoop` | 🟡 internal | Used only by queue subscription teardown. |

## Keys

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDDeviceKeys.h` string keys | ✅ implemented | Generated into `hid::keys::ALL_STRING_KEYS`; includes transport, report, usage, and entitlement keys. |
| `IOHIDKeys.h` string keys | ✅ implemented | Generated into `hid::keys::ALL_STRING_KEYS`; includes element, transport, calibration, keyboard, and sensor property keys. |
| `IOHIDProperties.h` string keys | ✅ implemented | Folded into the same generated key catalog. |
| `IOHIDOptionsType` / `IOHIDQueueOptionsType` / `IOHIDStandardType` / `IOHIDKeyboardPhysicalLayoutType` / `IOHIDAccelerationAlgorithmType` | ✅ implemented | Available as numeric constants plus generated numeric catalog entries in `hid::keys`. |

## Usage

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDUsageTables.h` usage/page constants (2370 symbols) | ✅ implemented | Generated into `hid::usage::ALL_USAGE_CONSTANTS` with helper lookups and selected convenience constants. |

## ServicePlugIn

| API | Status | Notes |
| --- | --- | --- |
| `kIOHIDDeviceFactoryID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `kIOHIDDeviceTypeID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `kIOHIDDeviceDeviceInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `kIOHIDDeviceDeviceInterfaceID2` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `kIOHIDDeviceQueueInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `kIOHIDDeviceTransactionInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and checked against the SDK headers in tests. |
| `IOHIDDeviceDeviceInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceTimeStampedDeviceInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceQueueInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceTransactionInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |

## EventSystem

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDEventServiceKeys.h` string keys | ✅ implemented | Exposed in `hid::event_system::ALL_EVENT_SYSTEM_KEYS` and checked against the SDK headers in tests. |
| `IOHIDKeyboardEventOptions` / `IOHIDPointerEventOptions` / `IOHIDScrollEventOptions` / `IOHIDServiceSensorControlOptions` | ✅ implemented | Exposed as numeric constants in `hid::event_system` and checked against the SDK headers in tests. |

