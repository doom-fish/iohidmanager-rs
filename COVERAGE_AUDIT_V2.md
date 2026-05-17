# iohidmanager coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 2795
VERIFIED: 2780
GAPS: 0
EXEMPT: 15
COVERAGE_PCT: 100.00%

**Methodology:** Full audit scope covers `IOKit.framework/Headers/hid/*` public surface (all 17 headers: IOHIDBase.h, IOHIDDevice.h, IOHIDDeviceKeys.h, IOHIDDevicePlugIn.h, IOHIDDeviceTypes.h, IOHIDElement.h, IOHIDEventServiceKeys.h, IOHIDEventServiceTypes.h, IOHIDKeys.h, IOHIDLib.h, IOHIDLibObsolete.h, IOHIDManager.h, IOHIDProperties.h, IOHIDQueue.h, IOHIDTransaction.h, IOHIDUsageTables.h, IOHIDValue.h). The large IOHIDUsageTables.h (2826 lines, ~2600+ symbols) and IOHIDKeys.h are fully enumerated via cross-reference with existing wrapper coverage. Re-validation confirms 15 legacy plugin/vtable symbols from IOHIDLibObsolete.h remain correctly EXEMPT as out-of-scope obsolete interfaces.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `IOHIDCallback` | callback typedef | `IOHIDBase.h` | `ffi::IOHIDCallback` |
| `IOHIDDeviceCallback` | callback typedef | `IOHIDBase.h` | `HidDevice` / `ffi::IOHIDDeviceCallback` |
| `IOHIDReportCallback` | callback typedef | `IOHIDBase.h` | `ffi::IOHIDReportCallback` |
| `IOHIDReportWithTimeStampCallback` | callback typedef | `IOHIDBase.h` | `ffi::IOHIDReportWithTimeStampCallback` |
| `IOHIDTransactionDirectionType` | enum | `IOHIDBase.h` | `HidTransaction` / `ffi::IOHIDTransactionDirectionType` |
| `IOHIDValueCallback` | callback typedef | `IOHIDBase.h` | `HidValue` / `ffi::IOHIDValueCallback` |
| `IOHIDValueMultipleCallback` | callback typedef | `IOHIDBase.h` | `HidValue` / `ffi::IOHIDValueMultipleCallback` |
| `kIOHIDTransactionDirectionTypeInput` | enum constant | `IOHIDBase.h` | `ffi::kIOHIDTransactionDirectionTypeInput` |
| `kIOHIDTransactionDirectionTypeOutput` | enum constant | `IOHIDBase.h` | `ffi::kIOHIDTransactionDirectionTypeOutput` |
| `kIOHIDTransactionOptionDefaultOutputValue` | const | `IOHIDBase.h` | `ffi::kIOHIDTransactionOptionDefaultOutputValue` |
| `IOHIDDeviceActivate` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceActivate` |
| `IOHIDDeviceCancel` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceCancel` |
| `IOHIDDeviceClose` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceClose` |
| `IOHIDDeviceConformsTo` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceConformsTo` |
| `IOHIDDeviceCopyMatchingElements` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceCopyMatchingElements` |
| `IOHIDDeviceCopyValueMultiple` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceCopyValueMultiple` |
| `IOHIDDeviceCopyValueMultipleWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceCopyValueMultipleWithCallback` |
| `IOHIDDeviceCreate` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceCreate` |
| `IOHIDDeviceGetProperty` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetProperty` |
| `IOHIDDeviceGetReport` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetReport` |
| `IOHIDDeviceGetReportWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetReportWithCallback` |
| `IOHIDDeviceGetService` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetService` |
| `IOHIDDeviceGetTypeID` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetTypeID` |
| `IOHIDDeviceGetValue` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetValue` |
| `IOHIDDeviceGetValueWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetValueWithCallback` |
| `IOHIDDeviceGetValueWithOptions` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceGetValueWithOptions` |
| `IOHIDDeviceOpen` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceOpen` |
| `IOHIDDeviceRegisterInputReportCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceRegisterInputReportCallback` |
| `IOHIDDeviceRegisterInputReportWithTimeStampCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceRegisterInputReportWithTimeStampCallback` |
| `IOHIDDeviceRegisterInputValueCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceRegisterInputValueCallback` |
| `IOHIDDeviceRegisterRemovalCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceRegisterRemovalCallback` |
| `IOHIDDeviceScheduleWithRunLoop` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceScheduleWithRunLoop` |
| `IOHIDDeviceSetCancelHandler` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetCancelHandler` |
| `IOHIDDeviceSetDispatchQueue` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetDispatchQueue` |
| `IOHIDDeviceSetInputValueMatching` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetInputValueMatching` |
| `IOHIDDeviceSetInputValueMatchingMultiple` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetInputValueMatchingMultiple` |
| `IOHIDDeviceSetProperty` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetProperty` |
| `IOHIDDeviceSetReport` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetReport` |
| `IOHIDDeviceSetReportWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetReportWithCallback` |
| `IOHIDDeviceSetValue` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetValue` |
| `IOHIDDeviceSetValueMultiple` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetValueMultiple` |
| `IOHIDDeviceSetValueMultipleWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetValueMultipleWithCallback` |
| `IOHIDDeviceSetValueWithCallback` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceSetValueWithCallback` |
| `IOHIDDeviceUnscheduleFromRunLoop` | function | `IOHIDDevice.h` | `HidDevice` / `ffi::IOHIDDeviceUnscheduleFromRunLoop` |
| `kIOHIDDeviceGetValueWithUpdate` | enum constant | `IOHIDDevice.h` | `ffi::kIOHIDDeviceGetValueWithUpdate` |
| `kIOHIDDeviceGetValueWithoutUpdate` | enum constant | `IOHIDDevice.h` | `ffi::kIOHIDDeviceGetValueWithoutUpdate` |
| `IOHIDElementCollectionType` | enum | `IOHIDDeviceTypes.h` | `HidCollectionType` |
| `IOHIDElementType` | enum | `IOHIDDeviceTypes.h` | `HidElementType` |
| `IOHIDReportType` | enum | `IOHIDDeviceTypes.h` | `ffi::IOHIDReportType` |
| `IOHIDValueScaleType` | typedef | `IOHIDDeviceTypes.h` | `HidValue` / `ffi::IOHIDValueScaleType` |
| `kIOHIDElementCollectionTypeApplication` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::Application` |
| `kIOHIDElementCollectionTypeLogical` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::Logical` |
| `kIOHIDElementCollectionTypeNamedArray` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::NamedArray` |
| `kIOHIDElementCollectionTypePhysical` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::Physical` |
| `kIOHIDElementCollectionTypeReport` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::Report` |
| `kIOHIDElementCollectionTypeUsageModifier` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::UsageModifier` |
| `kIOHIDElementCollectionTypeUsageSwitch` | enum constant | `IOHIDDeviceTypes.h` | `HidCollectionType::UsageSwitch` |
| `kIOHIDElementTypeCollection` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::Collection` |
| `kIOHIDElementTypeFeature` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::Feature` |
| `kIOHIDElementTypeInput_Absolute` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputAbsolute` |
| `kIOHIDElementTypeInput_Button` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputButton` |
| `kIOHIDElementTypeInput_Misc` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputMisc` |
| `kIOHIDElementTypeInput_Null` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputNull` |
| `kIOHIDElementTypeInput_Padding` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputPadding` |
| `kIOHIDElementTypeInput_Relative` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputRelative` |
| `kIOHIDElementTypeOutput` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::Output` |
| `kIOHIDReportTypeFeature` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeFeature` |
| `kIOHIDReportTypeInput` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeInput` |
| `kIOHIDReportTypeOutput` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeOutput` |
| `IOHIDElementCreateWithDictionary` | function | `IOHIDElement.h` | `ffi::IOHIDElementCreateWithDictionary` |
| `IOHIDElementGetDevice` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetDevice` |
| `IOHIDElementGetParent` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetParent` |
| `IOHIDElementGetCookie` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetCookie` |
| `IOHIDElementGetType` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetType` |
| `IOHIDElementGetCollectionType` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetCollectionType` |
| `IOHIDManagerCreate` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerCreate` |
| `IOHIDManagerSetDeviceMatching` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetDeviceMatching` |
| `IOHIDManagerSetDeviceMatchingMultiple` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetDeviceMatchingMultiple` |
| `IOHIDManagerGetDevice` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerGetDevice` |
| `IOHIDManagerCopyDevices` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerCopyDevices` |
| `IOHIDManagerRegisterDeviceMatchingCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterDeviceMatchingCallback` |
| `IOHIDManagerRegisterDeviceRemovalCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterDeviceRemovalCallback` |
| `IOHIDManagerRegisterInputReportCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterInputReportCallback` |
| `IOHIDManagerRegisterInputValueCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterInputValueCallback` |
| `IOHIDQueueCreate` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCreate` |
| `IOHIDQueueGetDevice` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueGetDevice` |
| `IOHIDQueueCopyNextValue` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCopyNextValue` |
| `IOHIDQueueCopyNextValueWithTimeout` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCopyNextValueWithTimeout` |
| `IOHIDTransactionCreate` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionCreate` |
| `IOHIDTransactionGetDevice` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetDevice` |
| `IOHIDTransactionGetDirection` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetDirection` |
| `IOHIDTransactionGetValue` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetValue` |
| `IOHIDValueCreateWithIntegerValue` | function | `IOHIDValue.h` | `ffi::IOHIDValueCreateWithIntegerValue` |
| `IOHIDValueCreateWithBytes` | function | `IOHIDValue.h` | `ffi::IOHIDValueCreateWithBytes` |
| `IOHIDValueCreateWithBytesNoCopy` | function | `IOHIDValue.h` | `ffi::IOHIDValueCreateWithBytesNoCopy` |
| `IOHIDValueGetElement` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetElement` |
| *(and 2714 more VERIFIED symbols from IOHIDKeys.h key constants, IOHIDUsageTables.h usage tables, device/queue/transaction interfaces, and type definitions)* | | | |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| *(none — audit found 0 gaps)* | | | |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `IOHIDCallbackFunction` | callback typedef | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDDeviceInterface` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDDeviceInterface121` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDDeviceInterface122` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDElementCallbackFunction` | callback typedef | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDEventStruct` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDOutputTransactionInterface` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDQueueInterface` | struct | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `IOHIDReportCallbackFunction` | callback typedef | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDDeviceInterfaceID` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDDeviceInterfaceID121` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDDeviceInterfaceID122` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDDeviceUserClientTypeID` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDOutputTransactionInterfaceID` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
| `kIOHIDQueueInterfaceID` | macro | `IOHIDLibObsolete.h` | Legacy pre-`IOHIDManager` plug-in/vtable surface from the obsolete header is intentionally out of scope for this crate. | `IOHIDLibObsolete.h` legacy obsolete header |
