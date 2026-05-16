# COVERAGE

Audit target: `IOKit.framework/Headers/hid/*` from the macOS 26.2 SDK.

Legend:

- ✅ implemented
- 🟡 partial
- ⏭️ skipped

## Summary

- Safe logical areas shipped in Rust: Manager, Device, Element, Value, Transaction, Queue, Keys, Usage, ServicePlugIn, EventSystem.
- Raw C declarations remain available behind the default-enabled `raw-ffi` feature (`iohidmanager::ffi`).
- Queue and transaction C-function coverage is validated alongside manager/device/element/value in `tests/api_coverage.rs`.
- Deferred: `IOHIDDevicePlugIn.h` COM-style vtable structs are not surfaced as safe Rust types; UUID discovery helpers are provided instead.

## Manager

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDManagerOptions` | ✅ implemented | Option bits exposed through raw-ffi; safe manager helpers use the default open path. |
| `IOHIDManagerRef` | ✅ implemented | Raw CFTypeRef available through `raw-ffi`; safe owner is `HidManager`. |
| `IOHIDManagerActivate` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerCancel` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerClose` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerCopyDevices` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerCreate` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerGetProperty` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerOpen` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterDeviceMatchingCallback` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterDeviceRemovalCallback` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterInputReportCallback` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterInputReportWithTimeStampCallback` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerRegisterInputValueCallback` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSaveToPropertyDomain` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerScheduleWithRunLoop` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetCancelHandler` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetDeviceMatching` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetDeviceMatchingMultiple` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetDispatchQueue` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetInputValueMatching` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetInputValueMatchingMultiple` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerSetProperty` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |
| `IOHIDManagerUnscheduleFromRunLoop` | ✅ implemented | Safe coverage lives in `hid::HidManager / hid::manager`; raw declaration available through `raw-ffi`. |

## Device

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDDeviceRef` | ✅ implemented | Raw handle via `raw-ffi`; safe owner is `HidDevice`. |
| `IOHIDDeviceActivate` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCancel` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceClose` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceConformsTo` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyMatchingElements` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyValueMultiple` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCopyValueMultipleWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceCreate` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetProperty` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetReport` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetReportWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetService` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetTypeID` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetValue` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetValueWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceGetValueWithOptions` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceOpen` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceRegisterInputReportCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceRegisterInputReportWithTimeStampCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceRegisterInputValueCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceRegisterRemovalCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceScheduleWithRunLoop` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetCancelHandler` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetDispatchQueue` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetInputValueMatching` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetInputValueMatchingMultiple` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetProperty` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetReport` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetReportWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValue` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueMultiple` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueMultipleWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceSetValueWithCallback` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |
| `IOHIDDeviceUnscheduleFromRunLoop` | ✅ implemented | Safe coverage lives in `hid::HidDevice / hid::device`; raw declaration available through `raw-ffi`. |

## Element

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDElementRef` | ✅ implemented | Raw handle via `raw-ffi`; safe view is `HidElement`. |
| `IOHIDElementAttach` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementCopyAttached` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementCreateWithDictionary` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementDetach` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetChildren` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetCollectionType` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetCookie` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
| `IOHIDElementGetDevice` | ✅ implemented | Safe coverage lives in `hid::HidElement / hid::element`; raw declaration available through `raw-ffi`. |
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
| `IOHIDTransactionCommitWithCallback` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionContainsElement` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionCreate` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
| `IOHIDTransactionGetDevice` | ✅ implemented | Safe coverage lives in `hid::transaction`; raw declaration available through `raw-ffi`. |
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
| `IOHIDQueueActivate` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueAddElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCancel` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueContainsElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCopyNextValue` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCopyNextValueWithTimeout` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueCreate` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueGetDepth` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueGetDevice` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueGetTypeID` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueRegisterValueAvailableCallback` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueRemoveElement` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueScheduleWithRunLoop` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueSetCancelHandler` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueSetDepth` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueSetDispatchQueue` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueStart` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueStop` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |
| `IOHIDQueueUnscheduleFromRunLoop` | ✅ implemented | Safe coverage lives in `hid::queue`; raw declaration available through `raw-ffi`. |

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
| `kIOHIDDeviceFactoryID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `kIOHIDDeviceTypeID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `kIOHIDDeviceDeviceInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `kIOHIDDeviceDeviceInterfaceID2` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `kIOHIDDeviceQueueInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `kIOHIDDeviceTransactionInterfaceID` | ✅ implemented | Exposed through `hid::service_plugin::ServicePlugInUuid` and validated against the Swift bridge. |
| `IOHIDDeviceDeviceInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceTimeStampedDeviceInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceQueueInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |
| `IOHIDDeviceTransactionInterface` | 🟡 partial | UUID discovery helpers are exposed, but the COM-style vtable struct itself is not wrapped as a safe Rust type. |

## EventSystem

| API | Status | Notes |
| --- | --- | --- |
| `IOHIDEventServiceKeys.h` string keys | ✅ implemented | Exposed in `hid::event_system::ALL_EVENT_SYSTEM_KEYS` and bridged sanity-checked in tests. |
| `IOHIDKeyboardEventOptions` / `IOHIDPointerEventOptions` / `IOHIDScrollEventOptions` / `IOHIDServiceSensorControlOptions` | ✅ implemented | Exposed as numeric constants in `hid::event_system` and validated through the Swift bridge. |

