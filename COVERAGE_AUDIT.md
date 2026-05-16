# iohidmanager coverage audit (vs MacOSX26.2.sdk)

> Audit scope: full `IOKit.framework/Headers/hid/*` public surface. `15` legacy symbols from `IOHIDLibObsolete.h` are classified EXEMPT.

SDK_PUBLIC_SYMBOLS: 2795
VERIFIED: 2722
GAPS: 58
EXEMPT: 15
COVERAGE_PCT: 97.91%

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
| `kIOHIDBatchIntervalKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDBatchIntervalKey")` |
| `kIOHIDBuiltInKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDBuiltInKey")` |
| `kIOHIDCountryCodeKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDCountryCodeKey")` |
| `kIOHIDDeviceAccessEntitlementKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceAccessEntitlementKey")` |
| `kIOHIDDeviceUsageKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceUsageKey")` |
| `kIOHIDDeviceUsagePageKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceUsagePageKey")` |
| `kIOHIDDeviceUsagePairsKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceUsagePairsKey")` |
| `kIOHIDLocationIDKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDLocationIDKey")` |
| `kIOHIDManufacturerKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDManufacturerKey")` |
| `kIOHIDMaxFeatureReportSizeKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMaxFeatureReportSizeKey")` |
| `kIOHIDMaxInputReportSizeKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMaxInputReportSizeKey")` |
| `kIOHIDMaxOutputReportSizeKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMaxOutputReportSizeKey")` |
| `kIOHIDPhysicalDeviceUniqueIDKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPhysicalDeviceUniqueIDKey")` |
| `kIOHIDPrimaryUsageKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPrimaryUsageKey")` |
| `kIOHIDPrimaryUsagePageKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPrimaryUsagePageKey")` |
| `kIOHIDProductIDKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDProductIDKey")` |
| `kIOHIDProductKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDProductKey")` |
| `kIOHIDReportDescriptorKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDReportDescriptorKey")` |
| `kIOHIDReportIntervalKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDReportIntervalKey")` |
| `kIOHIDRequestTimeoutKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDRequestTimeoutKey")` |
| `kIOHIDSerialNumberKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSerialNumberKey")` |
| `kIOHIDTransportKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportKey")` |
| `kIOHIDVendorIDKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDVendorIDKey")` |
| `kIOHIDVersionNumberKey` | macro | `IOHIDDeviceKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDVersionNumberKey")` |
| `kIOHIDDeviceDeviceInterfaceID` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
| `kIOHIDDeviceDeviceInterfaceID2` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
| `kIOHIDDeviceFactoryID` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
| `kIOHIDDeviceQueueInterfaceID` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
| `kIOHIDDeviceTransactionInterfaceID` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
| `kIOHIDDeviceTypeID` | macro | `IOHIDDevicePlugIn.h` | `service_plugin::ServicePlugInUuid` |
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
| `kIOHIDElementTypeInput_Axis` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputAxis` |
| `kIOHIDElementTypeInput_Button` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputButton` |
| `kIOHIDElementTypeInput_Misc` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputMisc` |
| `kIOHIDElementTypeInput_NULL` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputNull` |
| `kIOHIDElementTypeInput_ScanCodes` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::InputScanCodes` |
| `kIOHIDElementTypeOutput` | enum constant | `IOHIDDeviceTypes.h` | `HidElementType::Output` |
| `kIOHIDReportTypeCount` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeCount` |
| `kIOHIDReportTypeFeature` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeFeature` |
| `kIOHIDReportTypeInput` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeInput` |
| `kIOHIDReportTypeOutput` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDReportTypeOutput` |
| `kIOHIDValueScaleTypeCalibrated` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDValueScaleTypeCalibrated` |
| `kIOHIDValueScaleTypeExponent` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDValueScaleTypeExponent` |
| `kIOHIDValueScaleTypePhysical` | enum constant | `IOHIDDeviceTypes.h` | `ffi::kIOHIDValueScaleTypePhysical` |
| `IOHIDElementAttach` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementAttach` |
| `IOHIDElementCopyAttached` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementCopyAttached` |
| `IOHIDElementCreateWithDictionary` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementCreateWithDictionary` |
| `IOHIDElementDetach` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementDetach` |
| `IOHIDElementGetChildren` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetChildren` |
| `IOHIDElementGetCollectionType` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetCollectionType` |
| `IOHIDElementGetCookie` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetCookie` |
| `IOHIDElementGetDevice` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetDevice` |
| `IOHIDElementGetLogicalMax` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetLogicalMax` |
| `IOHIDElementGetLogicalMin` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetLogicalMin` |
| `IOHIDElementGetName` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetName` |
| `IOHIDElementGetParent` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetParent` |
| `IOHIDElementGetPhysicalMax` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetPhysicalMax` |
| `IOHIDElementGetPhysicalMin` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetPhysicalMin` |
| `IOHIDElementGetProperty` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetProperty` |
| `IOHIDElementGetReportCount` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetReportCount` |
| `IOHIDElementGetReportID` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetReportID` |
| `IOHIDElementGetReportSize` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetReportSize` |
| `IOHIDElementGetType` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetType` |
| `IOHIDElementGetTypeID` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetTypeID` |
| `IOHIDElementGetUnit` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetUnit` |
| `IOHIDElementGetUnitExponent` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetUnitExponent` |
| `IOHIDElementGetUsage` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetUsage` |
| `IOHIDElementGetUsagePage` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementGetUsagePage` |
| `IOHIDElementHasNullState` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementHasNullState` |
| `IOHIDElementHasPreferredState` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementHasPreferredState` |
| `IOHIDElementIsArray` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementIsArray` |
| `IOHIDElementIsNonLinear` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementIsNonLinear` |
| `IOHIDElementIsRelative` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementIsRelative` |
| `IOHIDElementIsVirtual` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementIsVirtual` |
| `IOHIDElementIsWrapping` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementIsWrapping` |
| `IOHIDElementSetProperty` | function | `IOHIDElement.h` | `HidElement` / `ffi::IOHIDElementSetProperty` |
| `kHIDPointerReportRateKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDDigitizerTipThresholdKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDEventDriverHandlesReport` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDEventServiceSensorControlOptionsKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDHeightKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDMouseAccelerationTypeKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDMouseScrollAccelerationKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDPointerAccelerationKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDPointerAccelerationMultiplierKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDPointerAccelerationSupportKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDPointerAccelerationTypeKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDScrollAccelerationKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDScrollAccelerationSupportKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDScrollAccelerationTypeKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDScrollReportRateKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDServiceAccelerationProperties` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDSurfaceDimensionsKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDTrackpadAccelerationType` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDTrackpadScrollAccelerationKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDWidthKey` | macro | `IOHIDEventServiceKeys.h` | `event_system::ALL_EVENT_SYSTEM_KEYS` |
| `kIOHIDKeyboardEventOptionsNoKeyRepeat` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::KEYBOARD_EVENT_OPTIONS_NO_KEY_REPEAT` |
| `kIOHIDPointerEventOptionsNoAcceleration` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::POINTER_EVENT_OPTIONS_NO_ACCELERATION` |
| `kIOHIDScrollEventOptionsNoAcceleration` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::SCROLL_EVENT_OPTIONS_NO_ACCELERATION` |
| `kIOHIDServiceSensorControlAggregation` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::SENSOR_CONTROL_AGGREGATION` |
| `kIOHIDServiceSensorControlDecimation` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::SENSOR_CONTROL_DECIMATION` |
| `kIOHIDServiceSensorControlDispatchControl` | enum constant | `IOHIDEventServiceTypes.h` | `event_system::SENSOR_CONTROL_DISPATCH_CONTROL` |
| `IOHIDQueueOptionsType` | typedef | `IOHIDKeys.h` | `HidQueueOptions` |
| `kIOHIDAccelerationAlgorithmTypeDefault` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDAccelerationAlgorithmTypeDefault")` |
| `kIOHIDAccelerationAlgorithmTypeParametric` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDAccelerationAlgorithmTypeParametric")` |
| `kIOHIDAccelerationAlgorithmTypeTable` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDAccelerationAlgorithmTypeTable")` |
| `kIOHIDAltHandlerIdKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDAltHandlerIdKey")` |
| `kIOHIDCategoryAutomotiveValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDCategoryAutomotiveValue")` |
| `kIOHIDCategoryKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDCategoryKey")` |
| `kIOHIDDeviceKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceKey")` |
| `kIOHIDDeviceOpenedByEventSystemKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceOpenedByEventSystemKey")` |
| `kIOHIDDeviceSuspendKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDeviceSuspendKey")` |
| `kIOHIDDigitizerGestureCharacterStateKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDigitizerGestureCharacterStateKey")` |
| `kIOHIDDigitizerSurfaceSwitchKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDigitizerSurfaceSwitchKey")` |
| `kIOHIDDisplayIntegratedKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDDisplayIntegratedKey")` |
| `kIOHIDElementCalibrationDeadZoneMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationDeadZoneMaxKey")` |
| `kIOHIDElementCalibrationDeadZoneMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationDeadZoneMinKey")` |
| `kIOHIDElementCalibrationGranularityKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationGranularityKey")` |
| `kIOHIDElementCalibrationMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationMaxKey")` |
| `kIOHIDElementCalibrationMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationMinKey")` |
| `kIOHIDElementCalibrationSaturationMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationSaturationMaxKey")` |
| `kIOHIDElementCalibrationSaturationMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCalibrationSaturationMinKey")` |
| `kIOHIDElementCollectionTypeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCollectionTypeKey")` |
| `kIOHIDElementCookieKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCookieKey")` |
| `kIOHIDElementCookieMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCookieMaxKey")` |
| `kIOHIDElementCookieMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementCookieMinKey")` |
| `kIOHIDElementDuplicateIndexKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementDuplicateIndexKey")` |
| `kIOHIDElementFlagsKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementFlagsKey")` |
| `kIOHIDElementHasNullStateKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementHasNullStateKey")` |
| `kIOHIDElementHasPreferredStateKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementHasPreferredStateKey")` |
| `kIOHIDElementIsArrayKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementIsArrayKey")` |
| `kIOHIDElementIsNonLinearKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementIsNonLinearKey")` |
| `kIOHIDElementIsRelativeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementIsRelativeKey")` |
| `kIOHIDElementIsWrappingKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementIsWrappingKey")` |
| `kIOHIDElementKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementKey")` |
| `kIOHIDElementMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementMaxKey")` |
| `kIOHIDElementMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementMinKey")` |
| `kIOHIDElementNameKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementNameKey")` |
| `kIOHIDElementParentCollectionKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementParentCollectionKey")` |
| `kIOHIDElementReportCountKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementReportCountKey")` |
| `kIOHIDElementReportIDKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementReportIDKey")` |
| `kIOHIDElementReportSizeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementReportSizeKey")` |
| `kIOHIDElementScaledMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementScaledMaxKey")` |
| `kIOHIDElementScaledMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementScaledMinKey")` |
| `kIOHIDElementSizeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementSizeKey")` |
| `kIOHIDElementTypeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementTypeKey")` |
| `kIOHIDElementUnitExponentKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUnitExponentKey")` |
| `kIOHIDElementUnitKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUnitKey")` |
| `kIOHIDElementUsageKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUsageKey")` |
| `kIOHIDElementUsageMaxKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUsageMaxKey")` |
| `kIOHIDElementUsageMinKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUsageMinKey")` |
| `kIOHIDElementUsagePageKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementUsagePageKey")` |
| `kIOHIDElementValueLocationKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementValueLocationKey")` |
| `kIOHIDElementVariableSizeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDElementVariableSizeKey")` |
| `kIOHIDKeyboardCapsLockDelay` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardCapsLockDelay")` |
| `kIOHIDKeyboardEjectDelay` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardEjectDelay")` |
| `kIOHIDKeyboardFunctionKeyCountKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardFunctionKeyCountKey")` |
| `kIOHIDKeyboardLanguageKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardLanguageKey")` |
| `kIOHIDKeyboardLayoutValueKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardLayoutValueKey")` |
| `kIOHIDKeyboardPhysicalLayoutType101` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutType101")` |
| `kIOHIDKeyboardPhysicalLayoutType102` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutType102")` |
| `kIOHIDKeyboardPhysicalLayoutType103` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutType103")` |
| `kIOHIDKeyboardPhysicalLayoutType104` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutType104")` |
| `kIOHIDKeyboardPhysicalLayoutType106` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutType106")` |
| `kIOHIDKeyboardPhysicalLayoutTypeUnknown` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutTypeUnknown")` |
| `kIOHIDKeyboardPhysicalLayoutTypeVendor` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDKeyboardPhysicalLayoutTypeVendor")` |
| `kIOHIDKeyboardSupportsDoNotDisturbKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardSupportsDoNotDisturbKey")` |
| `kIOHIDKeyboardSupportsEscKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardSupportsEscKey")` |
| `kIOHIDMaxReportBufferCountKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMaxReportBufferCountKey")` |
| `kIOHIDMaxResponseLatencyKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMaxResponseLatencyKey")` |
| `kIOHIDModelNumberKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDModelNumberKey")` |
| `kIOHIDOptionsTypeMaskPrivate` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDOptionsTypeMaskPrivate")` |
| `kIOHIDOptionsTypeNone` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDOptionsTypeNone")` |
| `kIOHIDOptionsTypeSeizeDevice` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDOptionsTypeSeizeDevice")` |
| `kIOHIDPointerAccelerationAlgorithmKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPointerAccelerationAlgorithmKey")` |
| `kIOHIDPointerAccelerationMinimumKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPointerAccelerationMinimumKey")` |
| `kIOHIDPowerOnDelayNSKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPowerOnDelayNSKey")` |
| `kIOHIDPrimaryTrackpadCanBeDisabledKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPrimaryTrackpadCanBeDisabledKey")` |
| `kIOHIDProductIDArrayKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDProductIDArrayKey")` |
| `kIOHIDProductIDMaskKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDProductIDMaskKey")` |
| `kIOHIDQueueOptionsTypeEnqueueAll` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDQueueOptionsTypeEnqueueAll")` |
| `kIOHIDQueueOptionsTypeNone` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDQueueOptionsTypeNone")` |
| `kIOHIDReportBufferEntrySizeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDReportBufferEntrySizeKey")` |
| `kIOHIDResetKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDResetKey")` |
| `kIOHIDSampleIntervalKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSampleIntervalKey")` |
| `kIOHIDScrollAccelerationAlgorithmKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDScrollAccelerationAlgorithmKey")` |
| `kIOHIDSensorPropertyBatchIntervalKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSensorPropertyBatchIntervalKey")` |
| `kIOHIDSensorPropertyMaxFIFOEventsKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSensorPropertyMaxFIFOEventsKey")` |
| `kIOHIDSensorPropertyReportIntervalKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSensorPropertyReportIntervalKey")` |
| `kIOHIDSensorPropertyReportLatencyKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSensorPropertyReportLatencyKey")` |
| `kIOHIDSensorPropertySampleIntervalKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDSensorPropertySampleIntervalKey")` |
| `kIOHIDStandardTypeANSI` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDStandardTypeANSI")` |
| `kIOHIDStandardTypeISO` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDStandardTypeISO")` |
| `kIOHIDStandardTypeJIS` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDStandardTypeJIS")` |
| `kIOHIDStandardTypeKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDStandardTypeKey")` |
| `kIOHIDStandardTypeUnspecified` | enum constant | `IOHIDKeys.h` | `keys::ALL_NUMERIC_CONSTANTS` / `keys::numeric_constant("kIOHIDStandardTypeUnspecified")` |
| `kIOHIDTransportAIDBValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportAIDBValue")` |
| `kIOHIDTransportAirPlayValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportAirPlayValue")` |
| `kIOHIDTransportBTAACPValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportBTAACPValue")` |
| `kIOHIDTransportBluetoothLowEnergyValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportBluetoothLowEnergyValue")` |
| `kIOHIDTransportBluetoothValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportBluetoothValue")` |
| `kIOHIDTransportFIFOValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportFIFOValue")` |
| `kIOHIDTransportI2CValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportI2CValue")` |
| `kIOHIDTransportIAPValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportIAPValue")` |
| `kIOHIDTransportInductiveInBandValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportInductiveInBandValue")` |
| `kIOHIDTransportSPIValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportSPIValue")` |
| `kIOHIDTransportSPUValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportSPUValue")` |
| `kIOHIDTransportSerialValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportSerialValue")` |
| `kIOHIDTransportUSBValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportUSBValue")` |
| `kIOHIDTransportVirtualValue` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDTransportVirtualValue")` |
| `kIOHIDUniqueIDKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDUniqueIDKey")` |
| `kIOHIDVendorIDSourceKey` | macro | `IOHIDKeys.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDVendorIDSourceKey")` |
| `IOHIDManagerActivate` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerActivate` |
| `IOHIDManagerCancel` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerCancel` |
| `IOHIDManagerClose` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerClose` |
| `IOHIDManagerCopyDevices` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerCopyDevices` |
| `IOHIDManagerCreate` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerCreate` |
| `IOHIDManagerGetProperty` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerGetProperty` |
| `IOHIDManagerGetTypeID` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerGetTypeID` |
| `IOHIDManagerOpen` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerOpen` |
| `IOHIDManagerRegisterDeviceMatchingCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterDeviceMatchingCallback` |
| `IOHIDManagerRegisterDeviceRemovalCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterDeviceRemovalCallback` |
| `IOHIDManagerRegisterInputReportCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterInputReportCallback` |
| `IOHIDManagerRegisterInputReportWithTimeStampCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterInputReportWithTimeStampCallback` |
| `IOHIDManagerRegisterInputValueCallback` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerRegisterInputValueCallback` |
| `IOHIDManagerSaveToPropertyDomain` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSaveToPropertyDomain` |
| `IOHIDManagerScheduleWithRunLoop` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerScheduleWithRunLoop` |
| `IOHIDManagerSetCancelHandler` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetCancelHandler` |
| `IOHIDManagerSetDeviceMatching` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetDeviceMatching` |
| `IOHIDManagerSetDeviceMatchingMultiple` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetDeviceMatchingMultiple` |
| `IOHIDManagerSetDispatchQueue` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetDispatchQueue` |
| `IOHIDManagerSetInputValueMatching` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetInputValueMatching` |
| `IOHIDManagerSetInputValueMatchingMultiple` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetInputValueMatchingMultiple` |
| `IOHIDManagerSetProperty` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerSetProperty` |
| `IOHIDManagerUnscheduleFromRunLoop` | function | `IOHIDManager.h` | `HidManager` / `ffi::IOHIDManagerUnscheduleFromRunLoop` |
| `kIOHIDIdleTimeMicrosecondsKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDIdleTimeMicrosecondsKey")` |
| `kIOHIDKeyboardCapsLockDelayOverride` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardCapsLockDelayOverride")` |
| `kIOHIDKeyboardCapsLockDelayOverrideKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDKeyboardCapsLockDelayOverrideKey")` |
| `kIOHIDMouseAccelerationType` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDMouseAccelerationType")` |
| `kIOHIDPointerButtonMode` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPointerButtonMode")` |
| `kIOHIDPointerButtonModeKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDPointerButtonModeKey")` |
| `kIOHIDServiceCapsLockStateKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDServiceCapsLockStateKey")` |
| `kIOHIDServiceEjectDelayKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDServiceEjectDelayKey")` |
| `kIOHIDServiceInitialKeyRepeatDelayKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDServiceInitialKeyRepeatDelayKey")` |
| `kIOHIDServiceKeyRepeatDelayKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDServiceKeyRepeatDelayKey")` |
| `kIOHIDServiceLockKeyDelayKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDServiceLockKeyDelayKey")` |
| `kIOHIDUserKeyUsageMapKey` | macro | `IOHIDProperties.h` | `keys::ALL_STRING_KEYS` / `keys::string_key("kIOHIDUserKeyUsageMapKey")` |
| `IOHIDQueueActivate` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueActivate` |
| `IOHIDQueueAddElement` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueAddElement` |
| `IOHIDQueueCancel` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCancel` |
| `IOHIDQueueContainsElement` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueContainsElement` |
| `IOHIDQueueCopyNextValue` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCopyNextValue` |
| `IOHIDQueueCopyNextValueWithTimeout` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCopyNextValueWithTimeout` |
| `IOHIDQueueCreate` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueCreate` |
| `IOHIDQueueGetDepth` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueGetDepth` |
| `IOHIDQueueGetDevice` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueGetDevice` |
| `IOHIDQueueGetTypeID` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueGetTypeID` |
| `IOHIDQueueRegisterValueAvailableCallback` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueRegisterValueAvailableCallback` |
| `IOHIDQueueRemoveElement` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueRemoveElement` |
| `IOHIDQueueScheduleWithRunLoop` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueScheduleWithRunLoop` |
| `IOHIDQueueSetCancelHandler` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueSetCancelHandler` |
| `IOHIDQueueSetDepth` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueSetDepth` |
| `IOHIDQueueSetDispatchQueue` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueSetDispatchQueue` |
| `IOHIDQueueStart` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueStart` |
| `IOHIDQueueStop` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueStop` |
| `IOHIDQueueUnscheduleFromRunLoop` | function | `IOHIDQueue.h` | `HidQueue` / `ffi::IOHIDQueueUnscheduleFromRunLoop` |
| `IOHIDTransactionAddElement` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionAddElement` |
| `IOHIDTransactionClear` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionClear` |
| `IOHIDTransactionCommit` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionCommit` |
| `IOHIDTransactionCommitWithCallback` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionCommitWithCallback` |
| `IOHIDTransactionContainsElement` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionContainsElement` |
| `IOHIDTransactionCreate` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionCreate` |
| `IOHIDTransactionGetDevice` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetDevice` |
| `IOHIDTransactionGetDirection` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetDirection` |
| `IOHIDTransactionGetTypeID` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetTypeID` |
| `IOHIDTransactionGetValue` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionGetValue` |
| `IOHIDTransactionOptions` | enum | `IOHIDTransaction.h` | `HidTransactionOptions` |
| `IOHIDTransactionRemoveElement` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionRemoveElement` |
| `IOHIDTransactionScheduleWithRunLoop` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionScheduleWithRunLoop` |
| `IOHIDTransactionSetDirection` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionSetDirection` |
| `IOHIDTransactionSetValue` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionSetValue` |
| `IOHIDTransactionUnscheduleFromRunLoop` | function | `IOHIDTransaction.h` | `HidTransaction` / `ffi::IOHIDTransactionUnscheduleFromRunLoop` |
| `kIOHIDTransactionOptionsNone` | enum constant | `IOHIDTransaction.h` | `ffi::kIOHIDTransactionOptionsNone` |
| `kIOHIDTransactionOptionsWeakDevice` | enum constant | `IOHIDTransaction.h` | `ffi::kIOHIDTransactionOptionsWeakDevice` |
| `kHIDPage_AlphanumericDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_AlphanumericDisplay")` |
| `kHIDPage_Arcade` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Arcade")` |
| `kHIDPage_BarCodeScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_BarCodeScanner")` |
| `kHIDPage_BatterySystem` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_BatterySystem")` |
| `kHIDPage_BrailleDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_BrailleDisplay")` |
| `kHIDPage_Button` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Button")` |
| `kHIDPage_CameraControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_CameraControl")` |
| `kHIDPage_Consumer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Consumer")` |
| `kHIDPage_Digitizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Digitizer")` |
| `kHIDPage_FIDO` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_FIDO")` |
| `kHIDPage_Game` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Game")` |
| `kHIDPage_GenericDesktop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_GenericDesktop")` |
| `kHIDPage_GenericDeviceControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_GenericDeviceControls")` |
| `kHIDPage_Haptics` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Haptics")` |
| `kHIDPage_KeyboardOrKeypad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_KeyboardOrKeypad")` |
| `kHIDPage_LEDs` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_LEDs")` |
| `kHIDPage_MagneticStripeReader` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_MagneticStripeReader")` |
| `kHIDPage_Monitor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Monitor")` |
| `kHIDPage_MonitorEnumerated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_MonitorEnumerated")` |
| `kHIDPage_MonitorReserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_MonitorReserved")` |
| `kHIDPage_MonitorVirtual` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_MonitorVirtual")` |
| `kHIDPage_Ordinal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Ordinal")` |
| `kHIDPage_PID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_PID")` |
| `kHIDPage_PowerDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_PowerDevice")` |
| `kHIDPage_PowerReserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_PowerReserved")` |
| `kHIDPage_PowerReserved2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_PowerReserved2")` |
| `kHIDPage_Scale` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Scale")` |
| `kHIDPage_Sensor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Sensor")` |
| `kHIDPage_Simulation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Simulation")` |
| `kHIDPage_Sport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Sport")` |
| `kHIDPage_Telephony` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Telephony")` |
| `kHIDPage_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Undefined")` |
| `kHIDPage_Unicode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_Unicode")` |
| `kHIDPage_VR` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_VR")` |
| `kHIDPage_VendorDefinedStart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_VendorDefinedStart")` |
| `kHIDPage_WeighingDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDPage_WeighingDevice")` |
| `kHIDUsage_AD_ASCIICharacterSet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ASCIICharacterSet")` |
| `kHIDUsage_AD_AlphanumericDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_AlphanumericDisplay")` |
| `kHIDUsage_AD_CharacterHeight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CharacterHeight")` |
| `kHIDUsage_AD_CharacterReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CharacterReport")` |
| `kHIDUsage_AD_CharacterSpacingHorizontal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CharacterSpacingHorizontal")` |
| `kHIDUsage_AD_CharacterSpacingVertical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CharacterSpacingVertical")` |
| `kHIDUsage_AD_CharacterWidth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CharacterWidth")` |
| `kHIDUsage_AD_ClearDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ClearDisplay")` |
| `kHIDUsage_AD_Column` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_Column")` |
| `kHIDUsage_AD_Columns` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_Columns")` |
| `kHIDUsage_AD_CursorBlink` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CursorBlink")` |
| `kHIDUsage_AD_CursorEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CursorEnable")` |
| `kHIDUsage_AD_CursorMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CursorMode")` |
| `kHIDUsage_AD_CursorPixelPositioning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CursorPixelPositioning")` |
| `kHIDUsage_AD_CursorPositionReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_CursorPositionReport")` |
| `kHIDUsage_AD_DataReadBack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DataReadBack")` |
| `kHIDUsage_AD_DisplayAttributesReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DisplayAttributesReport")` |
| `kHIDUsage_AD_DisplayControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DisplayControlReport")` |
| `kHIDUsage_AD_DisplayData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DisplayData")` |
| `kHIDUsage_AD_DisplayEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DisplayEnable")` |
| `kHIDUsage_AD_DisplayStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_DisplayStatus")` |
| `kHIDUsage_AD_ErrFontdatacannotberead` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ErrFontdatacannotberead")` |
| `kHIDUsage_AD_ErrNotaloadablecharacter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ErrNotaloadablecharacter")` |
| `kHIDUsage_AD_FontData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_FontData")` |
| `kHIDUsage_AD_FontReadBack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_FontReadBack")` |
| `kHIDUsage_AD_FontReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_FontReport")` |
| `kHIDUsage_AD_HorizontalScroll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_HorizontalScroll")` |
| `kHIDUsage_AD_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_Reserved")` |
| `kHIDUsage_AD_Row` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_Row")` |
| `kHIDUsage_AD_Rows` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_Rows")` |
| `kHIDUsage_AD_ScreenSaverDelay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ScreenSaverDelay")` |
| `kHIDUsage_AD_ScreenSaverEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_ScreenSaverEnable")` |
| `kHIDUsage_AD_StatNotReady` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_StatNotReady")` |
| `kHIDUsage_AD_StatReady` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_StatReady")` |
| `kHIDUsage_AD_UnicodeCharacterSet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_UnicodeCharacterSet")` |
| `kHIDUsage_AD_VerticalScroll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_AD_VerticalScroll")` |
| `kHIDUsage_BCS_2DControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_2DControlReport")` |
| `kHIDUsage_BCS_ActiveTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ActiveTime")` |
| `kHIDUsage_BCS_AddEAN2_3LabelDefinition` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_AddEAN2_3LabelDefinition")` |
| `kHIDUsage_BCS_AimDuration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_AimDuration")` |
| `kHIDUsage_BCS_AimingLaserPattern` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_AimingLaserPattern")` |
| `kHIDUsage_BCS_Aiming_PointerMide` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Aiming_PointerMide")` |
| `kHIDUsage_BCS_AttributeReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_AttributeReport")` |
| `kHIDUsage_BCS_AztecCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_AztecCode")` |
| `kHIDUsage_BCS_BC412` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BC412")` |
| `kHIDUsage_BCS_BadgeReader` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BadgeReader")` |
| `kHIDUsage_BCS_BarCodePresent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BarCodePresent")` |
| `kHIDUsage_BCS_BarCodePresentSensor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BarCodePresentSensor")` |
| `kHIDUsage_BCS_BarCodeScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BarCodeScanner")` |
| `kHIDUsage_BCS_BarCodeScannerCradle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BarCodeScannerCradle")` |
| `kHIDUsage_BCS_BarSpaceData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BarSpaceData")` |
| `kHIDUsage_BCS_BeeperState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BeeperState")` |
| `kHIDUsage_BCS_BooklandEAN` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_BooklandEAN")` |
| `kHIDUsage_BCS_ChannelCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ChannelCode")` |
| `kHIDUsage_BCS_Check` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Check")` |
| `kHIDUsage_BCS_CheckDigit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigit")` |
| `kHIDUsage_BCS_CheckDigitCodabarEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitCodabarEnable")` |
| `kHIDUsage_BCS_CheckDigitCode99Enable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitCode99Enable")` |
| `kHIDUsage_BCS_CheckDigitDisable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitDisable")` |
| `kHIDUsage_BCS_CheckDigitEnableInterleaved2of5OPCC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableInterleaved2of5OPCC")` |
| `kHIDUsage_BCS_CheckDigitEnableInterleaved2of5USS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableInterleaved2of5USS")` |
| `kHIDUsage_BCS_CheckDigitEnableOneMSIPlessey` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableOneMSIPlessey")` |
| `kHIDUsage_BCS_CheckDigitEnableStandard2of5OPCC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableStandard2of5OPCC")` |
| `kHIDUsage_BCS_CheckDigitEnableStandard2of5USS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableStandard2of5USS")` |
| `kHIDUsage_BCS_CheckDigitEnableTwoMSIPlessey` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDigitEnableTwoMSIPlessey")` |
| `kHIDUsage_BCS_CheckDisablePrice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckDisablePrice")` |
| `kHIDUsage_BCS_CheckEnable4DigitPrice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckEnable4DigitPrice")` |
| `kHIDUsage_BCS_CheckEnable5DigitPrice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckEnable5DigitPrice")` |
| `kHIDUsage_BCS_CheckEnableEuropean4DigitPrice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckEnableEuropean4DigitPrice")` |
| `kHIDUsage_BCS_CheckEnableEuropean5DigitPrice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CheckEnableEuropean5DigitPrice")` |
| `kHIDUsage_BCS_Class1ALaser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Class1ALaser")` |
| `kHIDUsage_BCS_Class2Laser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Class2Laser")` |
| `kHIDUsage_BCS_ClearAllEAN2_3LabelDefinitions` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ClearAllEAN2_3LabelDefinitions")` |
| `kHIDUsage_BCS_Codabar` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Codabar")` |
| `kHIDUsage_BCS_CodabarControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CodabarControlReport")` |
| `kHIDUsage_BCS_Code128` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code128")` |
| `kHIDUsage_BCS_Code128ControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code128ControlReport")` |
| `kHIDUsage_BCS_Code16` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code16")` |
| `kHIDUsage_BCS_Code32` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code32")` |
| `kHIDUsage_BCS_Code39` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code39")` |
| `kHIDUsage_BCS_Code39ControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code39ControlReport")` |
| `kHIDUsage_BCS_Code49` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code49")` |
| `kHIDUsage_BCS_Code93` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Code93")` |
| `kHIDUsage_BCS_CodeOne` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CodeOne")` |
| `kHIDUsage_BCS_Colorcode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Colorcode")` |
| `kHIDUsage_BCS_CommitParametersToNVM` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CommitParametersToNVM")` |
| `kHIDUsage_BCS_ConstantElectronicArticleSurveillance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ConstantElectronicArticleSurveillance")` |
| `kHIDUsage_BCS_ContactScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ContactScanner")` |
| `kHIDUsage_BCS_ConvertEAN8To13Type` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ConvertEAN8To13Type")` |
| `kHIDUsage_BCS_ConvertUPCAToEAN_13` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ConvertUPCAToEAN_13")` |
| `kHIDUsage_BCS_ConvertUPC_EToA` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ConvertUPC_EToA")` |
| `kHIDUsage_BCS_CordlessScannerBase` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_CordlessScannerBase")` |
| `kHIDUsage_BCS_DLMethodCheckForDiscrete` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DLMethodCheckForDiscrete")` |
| `kHIDUsage_BCS_DLMethodCheckInRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DLMethodCheckInRange")` |
| `kHIDUsage_BCS_DLMethodReadAny` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DLMethodReadAny")` |
| `kHIDUsage_BCS_DataLengthMethod` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DataLengthMethod")` |
| `kHIDUsage_BCS_DataMatrix` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DataMatrix")` |
| `kHIDUsage_BCS_DataPrefix` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DataPrefix")` |
| `kHIDUsage_BCS_DecodeDataContinued` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DecodeDataContinued")` |
| `kHIDUsage_BCS_DecodedData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DecodedData")` |
| `kHIDUsage_BCS_DisableCheckDigitTransmit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DisableCheckDigitTransmit")` |
| `kHIDUsage_BCS_DumbBarCodeScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_DumbBarCodeScanner")` |
| `kHIDUsage_BCS_EAN13FlagDigit1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN13FlagDigit1")` |
| `kHIDUsage_BCS_EAN13FlagDigit2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN13FlagDigit2")` |
| `kHIDUsage_BCS_EAN13FlagDigit3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN13FlagDigit3")` |
| `kHIDUsage_BCS_EAN2_3LabelControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN2_3LabelControlReport")` |
| `kHIDUsage_BCS_EAN8FlagDigit1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN8FlagDigit1")` |
| `kHIDUsage_BCS_EAN8FlagDigit2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN8FlagDigit2")` |
| `kHIDUsage_BCS_EAN8FlagDigit3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN8FlagDigit3")` |
| `kHIDUsage_BCS_EANThreeLabel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EANThreeLabel")` |
| `kHIDUsage_BCS_EANTwoLabel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EANTwoLabel")` |
| `kHIDUsage_BCS_EAN_13` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN_13")` |
| `kHIDUsage_BCS_EAN_8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN_8")` |
| `kHIDUsage_BCS_EAN_99_128_Mandatory` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN_99_128_Mandatory")` |
| `kHIDUsage_BCS_EAN_99_P5_128_Optional` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EAN_99_P5_128_Optional")` |
| `kHIDUsage_BCS_ElectronicArticleSurveillanceNotification` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ElectronicArticleSurveillanceNotification")` |
| `kHIDUsage_BCS_EnableCheckDigitTransmit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_EnableCheckDigitTransmit")` |
| `kHIDUsage_BCS_ErrorIndication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ErrorIndication")` |
| `kHIDUsage_BCS_FirstDiscreteLengthToDecode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_FirstDiscreteLengthToDecode")` |
| `kHIDUsage_BCS_FixedBeeper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_FixedBeeper")` |
| `kHIDUsage_BCS_FragmentDecoding` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_FragmentDecoding")` |
| `kHIDUsage_BCS_FullASCIIConversion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_FullASCIIConversion")` |
| `kHIDUsage_BCS_GRWTIAfterDecode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GRWTIAfterDecode")` |
| `kHIDUsage_BCS_GRWTIBeep_LampAfterTransmit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GRWTIBeep_LampAfterTransmit")` |
| `kHIDUsage_BCS_GRWTINoBeep_LampUseAtAll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GRWTINoBeep_LampUseAtAll")` |
| `kHIDUsage_BCS_GoodDecodeIndication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodDecodeIndication")` |
| `kHIDUsage_BCS_GoodReadLED` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadLED")` |
| `kHIDUsage_BCS_GoodReadLampDuration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadLampDuration")` |
| `kHIDUsage_BCS_GoodReadLampIntensity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadLampIntensity")` |
| `kHIDUsage_BCS_GoodReadToneFrequency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadToneFrequency")` |
| `kHIDUsage_BCS_GoodReadToneLength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadToneLength")` |
| `kHIDUsage_BCS_GoodReadToneVolume` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadToneVolume")` |
| `kHIDUsage_BCS_GoodReadWhenToWrite` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_GoodReadWhenToWrite")` |
| `kHIDUsage_BCS_HandsFreeScanning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_HandsFreeScanning")` |
| `kHIDUsage_BCS_HeaterPresent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_HeaterPresent")` |
| `kHIDUsage_BCS_InitiateBarcodeRead` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_InitiateBarcodeRead")` |
| `kHIDUsage_BCS_Interleaved2of5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Interleaved2of5")` |
| `kHIDUsage_BCS_Interleaved2of5ControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Interleaved2of5ControlReport")` |
| `kHIDUsage_BCS_IntrinsicallySafe` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_IntrinsicallySafe")` |
| `kHIDUsage_BCS_ItalianPharmacyCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ItalianPharmacyCode")` |
| `kHIDUsage_BCS_KlasseEinsLaser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_KlasseEinsLaser")` |
| `kHIDUsage_BCS_LaserOnTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_LaserOnTime")` |
| `kHIDUsage_BCS_LaserState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_LaserState")` |
| `kHIDUsage_BCS_LockoutTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_LockoutTime")` |
| `kHIDUsage_BCS_LongRangeScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_LongRangeScanner")` |
| `kHIDUsage_BCS_MSIPlesseyControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MSIPlesseyControlReport")` |
| `kHIDUsage_BCS_MSI_Plessey` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MSI_Plessey")` |
| `kHIDUsage_BCS_MaxiCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MaxiCode")` |
| `kHIDUsage_BCS_MaximumLengthToDecode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MaximumLengthToDecode")` |
| `kHIDUsage_BCS_MicroPDF` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MicroPDF")` |
| `kHIDUsage_BCS_MinimumLengthToDecode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MinimumLengthToDecode")` |
| `kHIDUsage_BCS_MirrorSpeedControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MirrorSpeedControl")` |
| `kHIDUsage_BCS_Misc1DControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Misc1DControlReport")` |
| `kHIDUsage_BCS_MotorState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MotorState")` |
| `kHIDUsage_BCS_MotorTimeout` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MotorTimeout")` |
| `kHIDUsage_BCS_MultiRangeScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_MultiRangeScanner")` |
| `kHIDUsage_BCS_NoReadMessage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_NoReadMessage")` |
| `kHIDUsage_BCS_NotOnFileIndication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_NotOnFileIndication")` |
| `kHIDUsage_BCS_NotOnFileVolume` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_NotOnFileVolume")` |
| `kHIDUsage_BCS_PDF_417` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PDF_417")` |
| `kHIDUsage_BCS_ParameterScanning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ParameterScanning")` |
| `kHIDUsage_BCS_ParametersChanged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ParametersChanged")` |
| `kHIDUsage_BCS_Periodical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Periodical")` |
| `kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus2")` |
| `kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalAutoDiscriminatePlus5")` |
| `kHIDUsage_BCS_PeriodicalIgnorePlus2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalIgnorePlus2")` |
| `kHIDUsage_BCS_PeriodicalIgnorePlus5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalIgnorePlus5")` |
| `kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus2")` |
| `kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PeriodicalOnlyDecodeWithPlus5")` |
| `kHIDUsage_BCS_PolarityInvertedBarCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PolarityInvertedBarCode")` |
| `kHIDUsage_BCS_PolarityNormalBarCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PolarityNormalBarCode")` |
| `kHIDUsage_BCS_PosiCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PosiCode")` |
| `kHIDUsage_BCS_PowerOnResetScanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PowerOnResetScanner")` |
| `kHIDUsage_BCS_PowerupBeep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PowerupBeep")` |
| `kHIDUsage_BCS_PrefixAIMI` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PrefixAIMI")` |
| `kHIDUsage_BCS_PrefixNone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PrefixNone")` |
| `kHIDUsage_BCS_PrefixProprietary` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PrefixProprietary")` |
| `kHIDUsage_BCS_PreventReadOfBarcodes` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_PreventReadOfBarcodes")` |
| `kHIDUsage_BCS_ProgrammableBeeper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ProgrammableBeeper")` |
| `kHIDUsage_BCS_ProximitySensor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ProximitySensor")` |
| `kHIDUsage_BCS_QRCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_QRCode")` |
| `kHIDUsage_BCS_RawDataPolarity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_RawDataPolarity")` |
| `kHIDUsage_BCS_RawScannedDataReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_RawScannedDataReport")` |
| `kHIDUsage_BCS_ScannedDataReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ScannedDataReport")` |
| `kHIDUsage_BCS_ScannerDataAccuracy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ScannerDataAccuracy")` |
| `kHIDUsage_BCS_ScannerInCradle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ScannerInCradle")` |
| `kHIDUsage_BCS_ScannerInRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ScannerInRange")` |
| `kHIDUsage_BCS_ScannerReadConfidence` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_ScannerReadConfidence")` |
| `kHIDUsage_BCS_SecondDiscreteLengthToDecode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SecondDiscreteLengthToDecode")` |
| `kHIDUsage_BCS_SetParameterDefaultValues` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SetParameterDefaultValues")` |
| `kHIDUsage_BCS_SettingsReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SettingsReport")` |
| `kHIDUsage_BCS_SoundErrorBeep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SoundErrorBeep")` |
| `kHIDUsage_BCS_SoundGoodReadBeep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SoundGoodReadBeep")` |
| `kHIDUsage_BCS_SoundNotOnFileBeep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SoundNotOnFileBeep")` |
| `kHIDUsage_BCS_Standard2of5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Standard2of5")` |
| `kHIDUsage_BCS_Standard2of5ControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Standard2of5ControlReport")` |
| `kHIDUsage_BCS_Standard2of5IATA` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Standard2of5IATA")` |
| `kHIDUsage_BCS_StatusReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_StatusReport")` |
| `kHIDUsage_BCS_SuperCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SuperCode")` |
| `kHIDUsage_BCS_SymbologyIdentifier1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SymbologyIdentifier1")` |
| `kHIDUsage_BCS_SymbologyIdentifier2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SymbologyIdentifier2")` |
| `kHIDUsage_BCS_SymbologyIdentifier3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_SymbologyIdentifier3")` |
| `kHIDUsage_BCS_TransmitCheckDigit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TransmitCheckDigit")` |
| `kHIDUsage_BCS_TransmitStart_Stop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TransmitStart_Stop")` |
| `kHIDUsage_BCS_TriOptic` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriOptic")` |
| `kHIDUsage_BCS_TriggerMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerMode")` |
| `kHIDUsage_BCS_TriggerModeBlinkingLaserOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerModeBlinkingLaserOn")` |
| `kHIDUsage_BCS_TriggerModeContinuousLaserOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerModeContinuousLaserOn")` |
| `kHIDUsage_BCS_TriggerModeLaserOnWhilePulled` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerModeLaserOnWhilePulled")` |
| `kHIDUsage_BCS_TriggerModeLaserStaysOnAfterTriggerRelease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerModeLaserStaysOnAfterTriggerRelease")` |
| `kHIDUsage_BCS_TriggerReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerReport")` |
| `kHIDUsage_BCS_TriggerState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_TriggerState")` |
| `kHIDUsage_BCS_Triggerless` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Triggerless")` |
| `kHIDUsage_BCS_UCC_EAN_128` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UCC_EAN_128")` |
| `kHIDUsage_BCS_UPC_A` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_A")` |
| `kHIDUsage_BCS_UPC_AWith128Mandatory` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_AWith128Mandatory")` |
| `kHIDUsage_BCS_UPC_AWith128Optical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_AWith128Optical")` |
| `kHIDUsage_BCS_UPC_AWithP5Optional` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_AWithP5Optional")` |
| `kHIDUsage_BCS_UPC_E` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_E")` |
| `kHIDUsage_BCS_UPC_E1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_E1")` |
| `kHIDUsage_BCS_UPC_EAN` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_EAN")` |
| `kHIDUsage_BCS_UPC_EANControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_EANControlReport")` |
| `kHIDUsage_BCS_UPC_EANCouponCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_EANCouponCode")` |
| `kHIDUsage_BCS_UPC_EANPeriodicals` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UPC_EANPeriodicals")` |
| `kHIDUsage_BCS_USB_5_SlugCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_USB_5_SlugCode")` |
| `kHIDUsage_BCS_UltraCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_UltraCode")` |
| `kHIDUsage_BCS_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Undefined")` |
| `kHIDUsage_BCS_VeriCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_VeriCode")` |
| `kHIDUsage_BCS_Wand` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_Wand")` |
| `kHIDUsage_BCS_WaterResistant` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BCS_WaterResistant")` |
| `kHIDUsage_BD_6DotBrailleCell` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_6DotBrailleCell")` |
| `kHIDUsage_BD_8DotBrailleCell` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_8DotBrailleCell")` |
| `kHIDUsage_BD_BrailleButtons` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleButtons")` |
| `kHIDUsage_BD_BrailleDPadCenter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDPadCenter")` |
| `kHIDUsage_BD_BrailleDPadDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDPadDown")` |
| `kHIDUsage_BD_BrailleDPadLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDPadLeft")` |
| `kHIDUsage_BD_BrailleDPadRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDPadRight")` |
| `kHIDUsage_BD_BrailleDPadUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDPadUp")` |
| `kHIDUsage_BD_BrailleDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleDisplay")` |
| `kHIDUsage_BD_BrailleFaceControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleFaceControls")` |
| `kHIDUsage_BD_BrailleJoystickCenter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleJoystickCenter")` |
| `kHIDUsage_BD_BrailleJoystickDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleJoystickDown")` |
| `kHIDUsage_BD_BrailleJoystickLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleJoystickLeft")` |
| `kHIDUsage_BD_BrailleJoystickRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleJoystickRight")` |
| `kHIDUsage_BD_BrailleJoystickUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleJoystickUp")` |
| `kHIDUsage_BD_BrailleKeyboardDot1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot1")` |
| `kHIDUsage_BD_BrailleKeyboardDot2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot2")` |
| `kHIDUsage_BD_BrailleKeyboardDot3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot3")` |
| `kHIDUsage_BD_BrailleKeyboardDot4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot4")` |
| `kHIDUsage_BD_BrailleKeyboardDot5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot5")` |
| `kHIDUsage_BD_BrailleKeyboardDot6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot6")` |
| `kHIDUsage_BD_BrailleKeyboardDot7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot7")` |
| `kHIDUsage_BD_BrailleKeyboardDot8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardDot8")` |
| `kHIDUsage_BD_BrailleKeyboardLeftSpace` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardLeftSpace")` |
| `kHIDUsage_BD_BrailleKeyboardRightSpace` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardRightSpace")` |
| `kHIDUsage_BD_BrailleKeyboardSpace` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleKeyboardSpace")` |
| `kHIDUsage_BD_BrailleLeftControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleLeftControls")` |
| `kHIDUsage_BD_BraillePanLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BraillePanLeft")` |
| `kHIDUsage_BD_BraillePanRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BraillePanRight")` |
| `kHIDUsage_BD_BrailleRightControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleRightControls")` |
| `kHIDUsage_BD_BrailleRockerDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleRockerDown")` |
| `kHIDUsage_BD_BrailleRockerPress` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleRockerPress")` |
| `kHIDUsage_BD_BrailleRockerUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleRockerUp")` |
| `kHIDUsage_BD_BrailleRow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleRow")` |
| `kHIDUsage_BD_BrailleTopControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_BrailleTopControls")` |
| `kHIDUsage_BD_NumberOfBrailleCells` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_NumberOfBrailleCells")` |
| `kHIDUsage_BD_RouterKey` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_RouterKey")` |
| `kHIDUsage_BD_RouterSet1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_RouterSet1")` |
| `kHIDUsage_BD_RouterSet2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_RouterSet2")` |
| `kHIDUsage_BD_RouterSet3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_RouterSet3")` |
| `kHIDUsage_BD_RowRouterKey` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_RowRouterKey")` |
| `kHIDUsage_BD_ScreenReaderControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_ScreenReaderControl")` |
| `kHIDUsage_BD_ScreenReaderIdentifier` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_ScreenReaderIdentifier")` |
| `kHIDUsage_BD_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BD_Undefined")` |
| `kHIDUsage_BS_ACPresent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ACPresent")` |
| `kHIDUsage_BS_AbsoluteStateOfCharge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AbsoluteStateOfCharge")` |
| `kHIDUsage_BS_AlarmInhibited` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AlarmInhibited")` |
| `kHIDUsage_BS_AtRate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AtRate")` |
| `kHIDUsage_BS_AtRateOK` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AtRateOK")` |
| `kHIDUsage_BS_AtRateTimeToEmpty` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AtRateTimeToEmpty")` |
| `kHIDUsage_BS_AtRateTimeToFull` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AtRateTimeToFull")` |
| `kHIDUsage_BS_AverageCurrent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AverageCurrent")` |
| `kHIDUsage_BS_AverageTimeToEmpty` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AverageTimeToEmpty")` |
| `kHIDUsage_BS_AverageTimeToFull` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_AverageTimeToFull")` |
| `kHIDUsage_BS_BattPackModelLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BattPackModelLevel")` |
| `kHIDUsage_BS_BatteryInsertion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BatteryInsertion")` |
| `kHIDUsage_BS_BatteryPresent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BatteryPresent")` |
| `kHIDUsage_BS_BatterySupported` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BatterySupported")` |
| `kHIDUsage_BS_BelowRemainingCapacityLimit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BelowRemainingCapacityLimit")` |
| `kHIDUsage_BS_BroadcastToCharger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_BroadcastToCharger")` |
| `kHIDUsage_BS_CapacityGranularity1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CapacityGranularity1")` |
| `kHIDUsage_BS_CapacityGranularity2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CapacityGranularity2")` |
| `kHIDUsage_BS_CapacityMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CapacityMode")` |
| `kHIDUsage_BS_ChargeController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ChargeController")` |
| `kHIDUsage_BS_ChargerConnection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ChargerConnection")` |
| `kHIDUsage_BS_ChargerSelectorSupport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ChargerSelectorSupport")` |
| `kHIDUsage_BS_ChargerSpec` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ChargerSpec")` |
| `kHIDUsage_BS_Charging` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Charging")` |
| `kHIDUsage_BS_ChargingIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ChargingIndicator")` |
| `kHIDUsage_BS_ConditioningFlag` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ConditioningFlag")` |
| `kHIDUsage_BS_ConnectionToSMBus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ConnectionToSMBus")` |
| `kHIDUsage_BS_CurrentNotRegulated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CurrentNotRegulated")` |
| `kHIDUsage_BS_CurrentOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CurrentOutOfRange")` |
| `kHIDUsage_BS_CycleCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_CycleCount")` |
| `kHIDUsage_BS_DesignCapacity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_DesignCapacity")` |
| `kHIDUsage_BS_Discharging` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Discharging")` |
| `kHIDUsage_BS_EnablePolling` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_EnablePolling")` |
| `kHIDUsage_BS_FullChargeCapacity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_FullChargeCapacity")` |
| `kHIDUsage_BS_FullyCharged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_FullyCharged")` |
| `kHIDUsage_BS_FullyDischarged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_FullyDischarged")` |
| `kHIDUsage_BS_InhibitCharge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_InhibitCharge")` |
| `kHIDUsage_BS_InternalChargeController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_InternalChargeController")` |
| `kHIDUsage_BS_Level2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Level2")` |
| `kHIDUsage_BS_Level3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Level3")` |
| `kHIDUsage_BS_ManufacturerAccess` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ManufacturerAccess")` |
| `kHIDUsage_BS_ManufacturerData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ManufacturerData")` |
| `kHIDUsage_BS_ManufacturerDate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ManufacturerDate")` |
| `kHIDUsage_BS_MasterMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_MasterMode")` |
| `kHIDUsage_BS_Maxerror` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Maxerror")` |
| `kHIDUsage_BS_NeedReplacement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_NeedReplacement")` |
| `kHIDUsage_BS_OKToUse` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OKToUse")` |
| `kHIDUsage_BS_OptionalMfgFunction1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OptionalMfgFunction1")` |
| `kHIDUsage_BS_OptionalMfgFunction2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OptionalMfgFunction2")` |
| `kHIDUsage_BS_OptionalMfgFunction3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OptionalMfgFunction3")` |
| `kHIDUsage_BS_OptionalMfgFunction4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OptionalMfgFunction4")` |
| `kHIDUsage_BS_OptionalMfgFunction5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OptionalMfgFunction5")` |
| `kHIDUsage_BS_OutputConnection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_OutputConnection")` |
| `kHIDUsage_BS_PowerFail` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_PowerFail")` |
| `kHIDUsage_BS_PrimaryBattery` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_PrimaryBattery")` |
| `kHIDUsage_BS_PrimaryBatterySupport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_PrimaryBatterySupport")` |
| `kHIDUsage_BS_Rechargable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Rechargable")` |
| `kHIDUsage_BS_RelativeStateOfCharge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RelativeStateOfCharge")` |
| `kHIDUsage_BS_RemainingCapacity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RemainingCapacity")` |
| `kHIDUsage_BS_RemainingCapacityLimit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RemainingCapacityLimit")` |
| `kHIDUsage_BS_RemainingTimeLimit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RemainingTimeLimit")` |
| `kHIDUsage_BS_RemainingTimeLimitExpired` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RemainingTimeLimitExpired")` |
| `kHIDUsage_BS_ResetToZero` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ResetToZero")` |
| `kHIDUsage_BS_RunTimeToEmpty` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_RunTimeToEmpty")` |
| `kHIDUsage_BS_SMBAlarmWarning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBAlarmWarning")` |
| `kHIDUsage_BS_SMBBatteryMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBBatteryMode")` |
| `kHIDUsage_BS_SMBBatteryStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBBatteryStatus")` |
| `kHIDUsage_BS_SMBChargerMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBChargerMode")` |
| `kHIDUsage_BS_SMBChargerSpecInfo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBChargerSpecInfo")` |
| `kHIDUsage_BS_SMBChargerStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBChargerStatus")` |
| `kHIDUsage_BS_SMBErrorCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBErrorCode")` |
| `kHIDUsage_BS_SMBSelectorInfo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBSelectorInfo")` |
| `kHIDUsage_BS_SMBSelectorPresets` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBSelectorPresets")` |
| `kHIDUsage_BS_SMBSelectorState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SMBSelectorState")` |
| `kHIDUsage_BS_SelectorRevision` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SelectorRevision")` |
| `kHIDUsage_BS_SerialNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SerialNumber")` |
| `kHIDUsage_BS_SpecificationInfo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_SpecificationInfo")` |
| `kHIDUsage_BS_TerminateCharge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_TerminateCharge")` |
| `kHIDUsage_BS_TerminateDischarge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_TerminateDischarge")` |
| `kHIDUsage_BS_ThermistorCold` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ThermistorCold")` |
| `kHIDUsage_BS_ThermistorHot` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ThermistorHot")` |
| `kHIDUsage_BS_ThermistorOverRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ThermistorOverRange")` |
| `kHIDUsage_BS_ThermistorUnderRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_ThermistorUnderRange")` |
| `kHIDUsage_BS_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Undefined")` |
| `kHIDUsage_BS_Usenext` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_Usenext")` |
| `kHIDUsage_BS_VoltageNotRegulated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_VoltageNotRegulated")` |
| `kHIDUsage_BS_VoltageOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_VoltageOutOfRange")` |
| `kHIDUsage_BS_WarningCapacityLimit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_WarningCapacityLimit")` |
| `kHIDUsage_BS_iDeviceChemistry` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_iDeviceChemistry")` |
| `kHIDUsage_BS_iDevicename` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_iDevicename")` |
| `kHIDUsage_BS_iManufacturerName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_iManufacturerName")` |
| `kHIDUsage_BS_iOEMInformation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_BS_iOEMInformation")` |
| `kHIDUsage_Button_1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_1")` |
| `kHIDUsage_Button_10` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_10")` |
| `kHIDUsage_Button_100` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_100")` |
| `kHIDUsage_Button_101` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_101")` |
| `kHIDUsage_Button_102` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_102")` |
| `kHIDUsage_Button_103` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_103")` |
| `kHIDUsage_Button_104` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_104")` |
| `kHIDUsage_Button_105` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_105")` |
| `kHIDUsage_Button_106` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_106")` |
| `kHIDUsage_Button_107` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_107")` |
| `kHIDUsage_Button_108` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_108")` |
| `kHIDUsage_Button_109` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_109")` |
| `kHIDUsage_Button_11` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_11")` |
| `kHIDUsage_Button_110` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_110")` |
| `kHIDUsage_Button_111` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_111")` |
| `kHIDUsage_Button_112` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_112")` |
| `kHIDUsage_Button_113` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_113")` |
| `kHIDUsage_Button_114` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_114")` |
| `kHIDUsage_Button_115` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_115")` |
| `kHIDUsage_Button_116` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_116")` |
| `kHIDUsage_Button_117` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_117")` |
| `kHIDUsage_Button_118` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_118")` |
| `kHIDUsage_Button_119` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_119")` |
| `kHIDUsage_Button_12` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_12")` |
| `kHIDUsage_Button_120` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_120")` |
| `kHIDUsage_Button_121` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_121")` |
| `kHIDUsage_Button_122` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_122")` |
| `kHIDUsage_Button_123` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_123")` |
| `kHIDUsage_Button_124` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_124")` |
| `kHIDUsage_Button_125` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_125")` |
| `kHIDUsage_Button_126` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_126")` |
| `kHIDUsage_Button_127` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_127")` |
| `kHIDUsage_Button_128` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_128")` |
| `kHIDUsage_Button_129` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_129")` |
| `kHIDUsage_Button_13` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_13")` |
| `kHIDUsage_Button_130` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_130")` |
| `kHIDUsage_Button_131` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_131")` |
| `kHIDUsage_Button_132` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_132")` |
| `kHIDUsage_Button_133` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_133")` |
| `kHIDUsage_Button_134` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_134")` |
| `kHIDUsage_Button_135` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_135")` |
| `kHIDUsage_Button_136` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_136")` |
| `kHIDUsage_Button_137` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_137")` |
| `kHIDUsage_Button_138` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_138")` |
| `kHIDUsage_Button_139` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_139")` |
| `kHIDUsage_Button_14` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_14")` |
| `kHIDUsage_Button_140` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_140")` |
| `kHIDUsage_Button_141` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_141")` |
| `kHIDUsage_Button_142` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_142")` |
| `kHIDUsage_Button_143` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_143")` |
| `kHIDUsage_Button_144` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_144")` |
| `kHIDUsage_Button_145` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_145")` |
| `kHIDUsage_Button_146` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_146")` |
| `kHIDUsage_Button_147` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_147")` |
| `kHIDUsage_Button_148` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_148")` |
| `kHIDUsage_Button_149` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_149")` |
| `kHIDUsage_Button_15` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_15")` |
| `kHIDUsage_Button_150` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_150")` |
| `kHIDUsage_Button_151` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_151")` |
| `kHIDUsage_Button_152` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_152")` |
| `kHIDUsage_Button_153` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_153")` |
| `kHIDUsage_Button_154` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_154")` |
| `kHIDUsage_Button_155` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_155")` |
| `kHIDUsage_Button_156` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_156")` |
| `kHIDUsage_Button_157` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_157")` |
| `kHIDUsage_Button_158` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_158")` |
| `kHIDUsage_Button_159` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_159")` |
| `kHIDUsage_Button_16` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_16")` |
| `kHIDUsage_Button_160` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_160")` |
| `kHIDUsage_Button_161` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_161")` |
| `kHIDUsage_Button_162` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_162")` |
| `kHIDUsage_Button_163` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_163")` |
| `kHIDUsage_Button_164` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_164")` |
| `kHIDUsage_Button_165` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_165")` |
| `kHIDUsage_Button_166` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_166")` |
| `kHIDUsage_Button_167` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_167")` |
| `kHIDUsage_Button_168` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_168")` |
| `kHIDUsage_Button_169` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_169")` |
| `kHIDUsage_Button_17` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_17")` |
| `kHIDUsage_Button_170` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_170")` |
| `kHIDUsage_Button_171` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_171")` |
| `kHIDUsage_Button_172` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_172")` |
| `kHIDUsage_Button_173` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_173")` |
| `kHIDUsage_Button_174` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_174")` |
| `kHIDUsage_Button_175` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_175")` |
| `kHIDUsage_Button_176` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_176")` |
| `kHIDUsage_Button_177` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_177")` |
| `kHIDUsage_Button_178` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_178")` |
| `kHIDUsage_Button_179` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_179")` |
| `kHIDUsage_Button_18` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_18")` |
| `kHIDUsage_Button_180` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_180")` |
| `kHIDUsage_Button_181` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_181")` |
| `kHIDUsage_Button_182` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_182")` |
| `kHIDUsage_Button_183` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_183")` |
| `kHIDUsage_Button_184` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_184")` |
| `kHIDUsage_Button_185` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_185")` |
| `kHIDUsage_Button_186` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_186")` |
| `kHIDUsage_Button_187` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_187")` |
| `kHIDUsage_Button_188` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_188")` |
| `kHIDUsage_Button_189` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_189")` |
| `kHIDUsage_Button_19` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_19")` |
| `kHIDUsage_Button_190` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_190")` |
| `kHIDUsage_Button_191` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_191")` |
| `kHIDUsage_Button_192` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_192")` |
| `kHIDUsage_Button_193` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_193")` |
| `kHIDUsage_Button_194` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_194")` |
| `kHIDUsage_Button_195` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_195")` |
| `kHIDUsage_Button_196` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_196")` |
| `kHIDUsage_Button_197` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_197")` |
| `kHIDUsage_Button_198` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_198")` |
| `kHIDUsage_Button_199` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_199")` |
| `kHIDUsage_Button_2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_2")` |
| `kHIDUsage_Button_20` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_20")` |
| `kHIDUsage_Button_200` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_200")` |
| `kHIDUsage_Button_201` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_201")` |
| `kHIDUsage_Button_202` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_202")` |
| `kHIDUsage_Button_203` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_203")` |
| `kHIDUsage_Button_204` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_204")` |
| `kHIDUsage_Button_205` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_205")` |
| `kHIDUsage_Button_206` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_206")` |
| `kHIDUsage_Button_207` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_207")` |
| `kHIDUsage_Button_208` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_208")` |
| `kHIDUsage_Button_209` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_209")` |
| `kHIDUsage_Button_21` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_21")` |
| `kHIDUsage_Button_210` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_210")` |
| `kHIDUsage_Button_211` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_211")` |
| `kHIDUsage_Button_212` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_212")` |
| `kHIDUsage_Button_213` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_213")` |
| `kHIDUsage_Button_214` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_214")` |
| `kHIDUsage_Button_215` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_215")` |
| `kHIDUsage_Button_216` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_216")` |
| `kHIDUsage_Button_217` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_217")` |
| `kHIDUsage_Button_218` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_218")` |
| `kHIDUsage_Button_219` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_219")` |
| `kHIDUsage_Button_22` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_22")` |
| `kHIDUsage_Button_220` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_220")` |
| `kHIDUsage_Button_221` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_221")` |
| `kHIDUsage_Button_222` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_222")` |
| `kHIDUsage_Button_223` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_223")` |
| `kHIDUsage_Button_224` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_224")` |
| `kHIDUsage_Button_225` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_225")` |
| `kHIDUsage_Button_226` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_226")` |
| `kHIDUsage_Button_227` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_227")` |
| `kHIDUsage_Button_228` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_228")` |
| `kHIDUsage_Button_229` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_229")` |
| `kHIDUsage_Button_23` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_23")` |
| `kHIDUsage_Button_230` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_230")` |
| `kHIDUsage_Button_231` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_231")` |
| `kHIDUsage_Button_232` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_232")` |
| `kHIDUsage_Button_233` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_233")` |
| `kHIDUsage_Button_234` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_234")` |
| `kHIDUsage_Button_235` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_235")` |
| `kHIDUsage_Button_236` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_236")` |
| `kHIDUsage_Button_237` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_237")` |
| `kHIDUsage_Button_238` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_238")` |
| `kHIDUsage_Button_239` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_239")` |
| `kHIDUsage_Button_24` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_24")` |
| `kHIDUsage_Button_240` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_240")` |
| `kHIDUsage_Button_241` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_241")` |
| `kHIDUsage_Button_242` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_242")` |
| `kHIDUsage_Button_243` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_243")` |
| `kHIDUsage_Button_244` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_244")` |
| `kHIDUsage_Button_245` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_245")` |
| `kHIDUsage_Button_246` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_246")` |
| `kHIDUsage_Button_247` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_247")` |
| `kHIDUsage_Button_248` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_248")` |
| `kHIDUsage_Button_249` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_249")` |
| `kHIDUsage_Button_25` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_25")` |
| `kHIDUsage_Button_250` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_250")` |
| `kHIDUsage_Button_251` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_251")` |
| `kHIDUsage_Button_252` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_252")` |
| `kHIDUsage_Button_253` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_253")` |
| `kHIDUsage_Button_254` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_254")` |
| `kHIDUsage_Button_255` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_255")` |
| `kHIDUsage_Button_26` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_26")` |
| `kHIDUsage_Button_27` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_27")` |
| `kHIDUsage_Button_28` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_28")` |
| `kHIDUsage_Button_29` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_29")` |
| `kHIDUsage_Button_3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_3")` |
| `kHIDUsage_Button_30` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_30")` |
| `kHIDUsage_Button_31` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_31")` |
| `kHIDUsage_Button_32` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_32")` |
| `kHIDUsage_Button_33` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_33")` |
| `kHIDUsage_Button_34` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_34")` |
| `kHIDUsage_Button_35` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_35")` |
| `kHIDUsage_Button_36` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_36")` |
| `kHIDUsage_Button_37` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_37")` |
| `kHIDUsage_Button_38` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_38")` |
| `kHIDUsage_Button_39` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_39")` |
| `kHIDUsage_Button_4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_4")` |
| `kHIDUsage_Button_40` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_40")` |
| `kHIDUsage_Button_41` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_41")` |
| `kHIDUsage_Button_42` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_42")` |
| `kHIDUsage_Button_43` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_43")` |
| `kHIDUsage_Button_44` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_44")` |
| `kHIDUsage_Button_45` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_45")` |
| `kHIDUsage_Button_46` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_46")` |
| `kHIDUsage_Button_47` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_47")` |
| `kHIDUsage_Button_48` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_48")` |
| `kHIDUsage_Button_49` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_49")` |
| `kHIDUsage_Button_5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_5")` |
| `kHIDUsage_Button_50` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_50")` |
| `kHIDUsage_Button_51` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_51")` |
| `kHIDUsage_Button_52` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_52")` |
| `kHIDUsage_Button_53` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_53")` |
| `kHIDUsage_Button_54` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_54")` |
| `kHIDUsage_Button_55` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_55")` |
| `kHIDUsage_Button_56` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_56")` |
| `kHIDUsage_Button_57` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_57")` |
| `kHIDUsage_Button_58` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_58")` |
| `kHIDUsage_Button_59` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_59")` |
| `kHIDUsage_Button_6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_6")` |
| `kHIDUsage_Button_60` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_60")` |
| `kHIDUsage_Button_61` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_61")` |
| `kHIDUsage_Button_62` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_62")` |
| `kHIDUsage_Button_63` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_63")` |
| `kHIDUsage_Button_64` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_64")` |
| `kHIDUsage_Button_65` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_65")` |
| `kHIDUsage_Button_65535` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_65535")` |
| `kHIDUsage_Button_66` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_66")` |
| `kHIDUsage_Button_67` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_67")` |
| `kHIDUsage_Button_68` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_68")` |
| `kHIDUsage_Button_69` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_69")` |
| `kHIDUsage_Button_7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_7")` |
| `kHIDUsage_Button_70` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_70")` |
| `kHIDUsage_Button_71` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_71")` |
| `kHIDUsage_Button_72` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_72")` |
| `kHIDUsage_Button_73` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_73")` |
| `kHIDUsage_Button_74` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_74")` |
| `kHIDUsage_Button_75` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_75")` |
| `kHIDUsage_Button_76` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_76")` |
| `kHIDUsage_Button_77` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_77")` |
| `kHIDUsage_Button_78` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_78")` |
| `kHIDUsage_Button_79` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_79")` |
| `kHIDUsage_Button_8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_8")` |
| `kHIDUsage_Button_80` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_80")` |
| `kHIDUsage_Button_81` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_81")` |
| `kHIDUsage_Button_82` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_82")` |
| `kHIDUsage_Button_83` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_83")` |
| `kHIDUsage_Button_84` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_84")` |
| `kHIDUsage_Button_85` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_85")` |
| `kHIDUsage_Button_86` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_86")` |
| `kHIDUsage_Button_87` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_87")` |
| `kHIDUsage_Button_88` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_88")` |
| `kHIDUsage_Button_89` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_89")` |
| `kHIDUsage_Button_9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_9")` |
| `kHIDUsage_Button_90` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_90")` |
| `kHIDUsage_Button_91` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_91")` |
| `kHIDUsage_Button_92` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_92")` |
| `kHIDUsage_Button_93` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_93")` |
| `kHIDUsage_Button_94` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_94")` |
| `kHIDUsage_Button_95` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_95")` |
| `kHIDUsage_Button_96` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_96")` |
| `kHIDUsage_Button_97` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_97")` |
| `kHIDUsage_Button_98` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_98")` |
| `kHIDUsage_Button_99` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Button_99")` |
| `kHIDUsage_CC_Autofocus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_CC_Autofocus")` |
| `kHIDUsage_CC_Shutter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_CC_Shutter")` |
| `kHIDUsage_CC_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_CC_Undefined")` |
| `kHIDUsage_Csmr_3DModeSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_3DModeSelect")` |
| `kHIDUsage_Csmr_AC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_AC")` |
| `kHIDUsage_Csmr_ACAddToCart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACAddToCart")` |
| `kHIDUsage_Csmr_ACAllCaps` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACAllCaps")` |
| `kHIDUsage_Csmr_ACAttachComment` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACAttachComment")` |
| `kHIDUsage_Csmr_ACAttachFile` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACAttachFile")` |
| `kHIDUsage_Csmr_ACBack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACBack")` |
| `kHIDUsage_Csmr_ACBold` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACBold")` |
| `kHIDUsage_Csmr_ACBookmarks` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACBookmarks")` |
| `kHIDUsage_Csmr_ACBulletedList` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACBulletedList")` |
| `kHIDUsage_Csmr_ACBuyOrCheckout` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACBuyOrCheckout")` |
| `kHIDUsage_Csmr_ACCancel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCancel")` |
| `kHIDUsage_Csmr_ACCatalog` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCatalog")` |
| `kHIDUsage_Csmr_ACClearAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACClearAlarm")` |
| `kHIDUsage_Csmr_ACClose` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACClose")` |
| `kHIDUsage_Csmr_ACCollapse` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCollapse")` |
| `kHIDUsage_Csmr_ACCollapseAll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCollapseAll")` |
| `kHIDUsage_Csmr_ACCopy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCopy")` |
| `kHIDUsage_Csmr_ACCut` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACCut")` |
| `kHIDUsage_Csmr_ACDelete` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDelete")` |
| `kHIDUsage_Csmr_ACDemote` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDemote")` |
| `kHIDUsage_Csmr_ACDesktopShowAllApplications` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDesktopShowAllApplications")` |
| `kHIDUsage_Csmr_ACDesktopShowAllWindows` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDesktopShowAllWindows")` |
| `kHIDUsage_Csmr_ACDetachComment` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDetachComment")` |
| `kHIDUsage_Csmr_ACDistributeH` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDistributeH")` |
| `kHIDUsage_Csmr_ACDistributeV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDistributeV")` |
| `kHIDUsage_Csmr_ACDownload` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACDownload")` |
| `kHIDUsage_Csmr_ACEdit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACEdit")` |
| `kHIDUsage_Csmr_ACEditTimeZones` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACEditTimeZones")` |
| `kHIDUsage_Csmr_ACExit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACExit")` |
| `kHIDUsage_Csmr_ACExpand` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACExpand")` |
| `kHIDUsage_Csmr_ACExpandAll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACExpandAll")` |
| `kHIDUsage_Csmr_ACFilter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFilter")` |
| `kHIDUsage_Csmr_ACFind` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFind")` |
| `kHIDUsage_Csmr_ACFindandReplace` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFindandReplace")` |
| `kHIDUsage_Csmr_ACFlipHorizontal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFlipHorizontal")` |
| `kHIDUsage_Csmr_ACFlipVertical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFlipVertical")` |
| `kHIDUsage_Csmr_ACFontColor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFontColor")` |
| `kHIDUsage_Csmr_ACFontSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFontSelect")` |
| `kHIDUsage_Csmr_ACFontSize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFontSize")` |
| `kHIDUsage_Csmr_ACFormat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFormat")` |
| `kHIDUsage_Csmr_ACForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACForward")` |
| `kHIDUsage_Csmr_ACForwardMessage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACForwardMessage")` |
| `kHIDUsage_Csmr_ACFullScreenView` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACFullScreenView")` |
| `kHIDUsage_Csmr_ACGoTo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACGoTo")` |
| `kHIDUsage_Csmr_ACHistory` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACHistory")` |
| `kHIDUsage_Csmr_ACHome` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACHome")` |
| `kHIDUsage_Csmr_ACIdleKeepAlive` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACIdleKeepAlive")` |
| `kHIDUsage_Csmr_ACIndentyDecrease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACIndentyDecrease")` |
| `kHIDUsage_Csmr_ACIndentyIncrease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACIndentyIncrease")` |
| `kHIDUsage_Csmr_ACInsertColumn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertColumn")` |
| `kHIDUsage_Csmr_ACInsertFile` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertFile")` |
| `kHIDUsage_Csmr_ACInsertMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertMode")` |
| `kHIDUsage_Csmr_ACInsertObject` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertObject")` |
| `kHIDUsage_Csmr_ACInsertPicture` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertPicture")` |
| `kHIDUsage_Csmr_ACInsertRow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertRow")` |
| `kHIDUsage_Csmr_ACInsertSymbol` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACInsertSymbol")` |
| `kHIDUsage_Csmr_ACItalics` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACItalics")` |
| `kHIDUsage_Csmr_ACJustifyBlockH` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyBlockH")` |
| `kHIDUsage_Csmr_ACJustifyBlockV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyBlockV")` |
| `kHIDUsage_Csmr_ACJustifyBottom` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyBottom")` |
| `kHIDUsage_Csmr_ACJustifyCenterH` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyCenterH")` |
| `kHIDUsage_Csmr_ACJustifyCenterV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyCenterV")` |
| `kHIDUsage_Csmr_ACJustifyLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyLeft")` |
| `kHIDUsage_Csmr_ACJustifyRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyRight")` |
| `kHIDUsage_Csmr_ACJustifyTop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACJustifyTop")` |
| `kHIDUsage_Csmr_ACKeyboardLayoutSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACKeyboardLayoutSelect")` |
| `kHIDUsage_Csmr_ACLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACLock")` |
| `kHIDUsage_Csmr_ACMaximize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACMaximize")` |
| `kHIDUsage_Csmr_ACMerge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACMerge")` |
| `kHIDUsage_Csmr_ACMinimize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACMinimize")` |
| `kHIDUsage_Csmr_ACMirrorHorizontal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACMirrorHorizontal")` |
| `kHIDUsage_Csmr_ACMirrorVertical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACMirrorVertical")` |
| `kHIDUsage_Csmr_ACNavigationGuidance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNavigationGuidance")` |
| `kHIDUsage_Csmr_ACNew` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNew")` |
| `kHIDUsage_Csmr_ACNewWindow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNewWindow")` |
| `kHIDUsage_Csmr_ACNextLink` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNextLink")` |
| `kHIDUsage_Csmr_ACNo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNo")` |
| `kHIDUsage_Csmr_ACNormalView` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNormalView")` |
| `kHIDUsage_Csmr_ACNumberedList` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACNumberedList")` |
| `kHIDUsage_Csmr_ACOpen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACOpen")` |
| `kHIDUsage_Csmr_ACPan` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPan")` |
| `kHIDUsage_Csmr_ACPanLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPanLeft")` |
| `kHIDUsage_Csmr_ACPanRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPanRight")` |
| `kHIDUsage_Csmr_ACPaste` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPaste")` |
| `kHIDUsage_Csmr_ACPasteSpecial` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPasteSpecial")` |
| `kHIDUsage_Csmr_ACPreviousLink` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPreviousLink")` |
| `kHIDUsage_Csmr_ACPrint` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPrint")` |
| `kHIDUsage_Csmr_ACPrintPreview` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPrintPreview")` |
| `kHIDUsage_Csmr_ACPromote` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACPromote")` |
| `kHIDUsage_Csmr_ACProperties` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACProperties")` |
| `kHIDUsage_Csmr_ACProtect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACProtect")` |
| `kHIDUsage_Csmr_ACRedoOrRepeat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACRedoOrRepeat")` |
| `kHIDUsage_Csmr_ACRefresh` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACRefresh")` |
| `kHIDUsage_Csmr_ACRename` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACRename")` |
| `kHIDUsage_Csmr_ACReply` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACReply")` |
| `kHIDUsage_Csmr_ACReplyAll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACReplyAll")` |
| `kHIDUsage_Csmr_ACResetAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACResetAlarm")` |
| `kHIDUsage_Csmr_ACResize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACResize")` |
| `kHIDUsage_Csmr_ACRestartNumbering` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACRestartNumbering")` |
| `kHIDUsage_Csmr_ACRotate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACRotate")` |
| `kHIDUsage_Csmr_ACSave` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSave")` |
| `kHIDUsage_Csmr_ACSaveAndClose` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSaveAndClose")` |
| `kHIDUsage_Csmr_ACScroll` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACScroll")` |
| `kHIDUsage_Csmr_ACScrollDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACScrollDown")` |
| `kHIDUsage_Csmr_ACScrollUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACScrollUp")` |
| `kHIDUsage_Csmr_ACSearch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSearch")` |
| `kHIDUsage_Csmr_ACSelectColumn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectColumn")` |
| `kHIDUsage_Csmr_ACSelectObject` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectObject")` |
| `kHIDUsage_Csmr_ACSelectParagraph` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectParagraph")` |
| `kHIDUsage_Csmr_ACSelectRow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectRow")` |
| `kHIDUsage_Csmr_ACSelectSentence` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectSentence")` |
| `kHIDUsage_Csmr_ACSelectTable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectTable")` |
| `kHIDUsage_Csmr_ACSelectTimeZone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectTimeZone")` |
| `kHIDUsage_Csmr_ACSelectWord` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSelectWord")` |
| `kHIDUsage_Csmr_ACSend` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSend")` |
| `kHIDUsage_Csmr_ACSendOrReceive` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSendOrReceive")` |
| `kHIDUsage_Csmr_ACSendTo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSendTo")` |
| `kHIDUsage_Csmr_ACSetAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSetAlarm")` |
| `kHIDUsage_Csmr_ACSetBorders` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSetBorders")` |
| `kHIDUsage_Csmr_ACSetClock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSetClock")` |
| `kHIDUsage_Csmr_ACSnoozeAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSnoozeAlarm")` |
| `kHIDUsage_Csmr_ACSoftKeyLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSoftKeyLeft")` |
| `kHIDUsage_Csmr_ACSoftKeyRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSoftKeyRight")` |
| `kHIDUsage_Csmr_ACSort` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSort")` |
| `kHIDUsage_Csmr_ACSortAscending` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSortAscending")` |
| `kHIDUsage_Csmr_ACSortDescending` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSortDescending")` |
| `kHIDUsage_Csmr_ACSplit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSplit")` |
| `kHIDUsage_Csmr_ACStop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACStop")` |
| `kHIDUsage_Csmr_ACStrikethrough` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACStrikethrough")` |
| `kHIDUsage_Csmr_ACSubscript` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSubscript")` |
| `kHIDUsage_Csmr_ACSubscriptions` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSubscriptions")` |
| `kHIDUsage_Csmr_ACSuperscript` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSuperscript")` |
| `kHIDUsage_Csmr_ACSynchronize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACSynchronize")` |
| `kHIDUsage_Csmr_ACTileHorizontally` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACTileHorizontally")` |
| `kHIDUsage_Csmr_ACTileVertically` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACTileVertically")` |
| `kHIDUsage_Csmr_ACUnderline` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACUnderline")` |
| `kHIDUsage_Csmr_ACUndo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACUndo")` |
| `kHIDUsage_Csmr_ACUnlock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACUnlock")` |
| `kHIDUsage_Csmr_ACUnprotect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACUnprotect")` |
| `kHIDUsage_Csmr_ACUpload` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACUpload")` |
| `kHIDUsage_Csmr_ACViewClock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACViewClock")` |
| `kHIDUsage_Csmr_ACViewComment` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACViewComment")` |
| `kHIDUsage_Csmr_ACViewToggle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACViewToggle")` |
| `kHIDUsage_Csmr_ACYes` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACYes")` |
| `kHIDUsage_Csmr_ACZoom` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACZoom")` |
| `kHIDUsage_Csmr_ACZoomIn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACZoomIn")` |
| `kHIDUsage_Csmr_ACZoomOut` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ACZoomOut")` |
| `kHIDUsage_Csmr_AL` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_AL")` |
| `kHIDUsage_Csmr_ALAOrVCaptureOrPlayback` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALAOrVCaptureOrPlayback")` |
| `kHIDUsage_Csmr_ALAlarms` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALAlarms")` |
| `kHIDUsage_Csmr_ALAudioBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALAudioBrowser")` |
| `kHIDUsage_Csmr_ALAudioPlayer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALAudioPlayer")` |
| `kHIDUsage_Csmr_ALCalculator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALCalculator")` |
| `kHIDUsage_Csmr_ALCalendarOrSchedule` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALCalendarOrSchedule")` |
| `kHIDUsage_Csmr_ALCheckbookOrFinance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALCheckbookOrFinance")` |
| `kHIDUsage_Csmr_ALClock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALClock")` |
| `kHIDUsage_Csmr_ALCommandLineProcessorOrRun` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALCommandLineProcessorOrRun")` |
| `kHIDUsage_Csmr_ALConsumerControlConfiguration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALConsumerControlConfiguration")` |
| `kHIDUsage_Csmr_ALContactSync` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALContactSync")` |
| `kHIDUsage_Csmr_ALContactsOrAddressBook` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALContactsOrAddressBook")` |
| `kHIDUsage_Csmr_ALContextawareDesktopAssistant` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALContextawareDesktopAssistant")` |
| `kHIDUsage_Csmr_ALControlPanel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALControlPanel")` |
| `kHIDUsage_Csmr_ALCustomizedCorporateNewsBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALCustomizedCorporateNewsBrowser")` |
| `kHIDUsage_Csmr_ALDatabaseApp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDatabaseApp")` |
| `kHIDUsage_Csmr_ALDesktop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDesktop")` |
| `kHIDUsage_Csmr_ALDictionary` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDictionary")` |
| `kHIDUsage_Csmr_ALDigitalRightsManager` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDigitalRightsManager")` |
| `kHIDUsage_Csmr_ALDigitalWallet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDigitalWallet")` |
| `kHIDUsage_Csmr_ALDocuments` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALDocuments")` |
| `kHIDUsage_Csmr_ALEmailReader` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALEmailReader")` |
| `kHIDUsage_Csmr_ALEncryption` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALEncryption")` |
| `kHIDUsage_Csmr_ALEntertainmentContentBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALEntertainmentContentBrowser")` |
| `kHIDUsage_Csmr_ALFileBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALFileBrowser")` |
| `kHIDUsage_Csmr_ALGrammerCheck` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALGrammerCheck")` |
| `kHIDUsage_Csmr_ALGraphicsEditor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALGraphicsEditor")` |
| `kHIDUsage_Csmr_ALImageBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALImageBrowser")` |
| `kHIDUsage_Csmr_ALInstantMessaging` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALInstantMessaging")` |
| `kHIDUsage_Csmr_ALIntegratedHelpCenter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALIntegratedHelpCenter")` |
| `kHIDUsage_Csmr_ALInternetBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALInternetBrowser")` |
| `kHIDUsage_Csmr_ALKeyboardLayout` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALKeyboardLayout")` |
| `kHIDUsage_Csmr_ALLANOrWANBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLANOrWANBrowser")` |
| `kHIDUsage_Csmr_ALLaunchButtonConfigurationTool` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLaunchButtonConfigurationTool")` |
| `kHIDUsage_Csmr_ALLocalMachineBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLocalMachineBrowser")` |
| `kHIDUsage_Csmr_ALLogOrJournalOrTimecard` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLogOrJournalOrTimecard")` |
| `kHIDUsage_Csmr_ALLogoff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLogoff")` |
| `kHIDUsage_Csmr_ALLogon` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLogon")` |
| `kHIDUsage_Csmr_ALLogonOrLogoff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALLogonOrLogoff")` |
| `kHIDUsage_Csmr_ALMarketMonitorOrFinanceBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALMarketMonitorOrFinanceBrowser")` |
| `kHIDUsage_Csmr_ALMessageStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALMessageStatus")` |
| `kHIDUsage_Csmr_ALMovieBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALMovieBrowser")` |
| `kHIDUsage_Csmr_ALNavigation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALNavigation")` |
| `kHIDUsage_Csmr_ALNetworkChat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALNetworkChat")` |
| `kHIDUsage_Csmr_ALNetworkConference` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALNetworkConference")` |
| `kHIDUsage_Csmr_ALNewsreader` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALNewsreader")` |
| `kHIDUsage_Csmr_ALNextTaskOrApplication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALNextTaskOrApplication")` |
| `kHIDUsage_Csmr_ALOEMFeatureBrowser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALOEMFeatureBrowser")` |
| `kHIDUsage_Csmr_ALOEMHelp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALOEMHelp")` |
| `kHIDUsage_Csmr_ALOnlineActivityBrowswer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALOnlineActivityBrowswer")` |
| `kHIDUsage_Csmr_ALOnlineCommunity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALOnlineCommunity")` |
| `kHIDUsage_Csmr_ALOnlineShoppingBrowswer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALOnlineShoppingBrowswer")` |
| `kHIDUsage_Csmr_ALPowerStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALPowerStatus")` |
| `kHIDUsage_Csmr_ALPreemptiveHaltTaskOrApplication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALPreemptiveHaltTaskOrApplication")` |
| `kHIDUsage_Csmr_ALPresentationApp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALPresentationApp")` |
| `kHIDUsage_Csmr_ALPreviousTaskOrApplication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALPreviousTaskOrApplication")` |
| `kHIDUsage_Csmr_ALProcessOrTaskManager` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALProcessOrTaskManager")` |
| `kHIDUsage_Csmr_ALProgrammableButtonConfiguration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALProgrammableButtonConfiguration")` |
| `kHIDUsage_Csmr_ALRemoteNetworkingOrISPConnect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALRemoteNetworkingOrISPConnect")` |
| `kHIDUsage_Csmr_ALResearchOrSearchBrowswer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALResearchOrSearchBrowswer")` |
| `kHIDUsage_Csmr_ALScreenSaver` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALScreenSaver")` |
| `kHIDUsage_Csmr_ALSmartCardInformationOrHelp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALSmartCardInformationOrHelp")` |
| `kHIDUsage_Csmr_ALSpellCheck` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALSpellCheck")` |
| `kHIDUsage_Csmr_ALSpreadsheet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALSpreadsheet")` |
| `kHIDUsage_Csmr_ALTaskOrProjectManager` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALTaskOrProjectManager")` |
| `kHIDUsage_Csmr_ALTelephonyOrDialer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALTelephonyOrDialer")` |
| `kHIDUsage_Csmr_ALTerminalLockOrScreensaver` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALTerminalLockOrScreensaver")` |
| `kHIDUsage_Csmr_ALTextEditor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALTextEditor")` |
| `kHIDUsage_Csmr_ALThesaurus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALThesaurus")` |
| `kHIDUsage_Csmr_ALVirusProtection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALVirusProtection")` |
| `kHIDUsage_Csmr_ALVoicemail` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALVoicemail")` |
| `kHIDUsage_Csmr_ALWirelessStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALWirelessStatus")` |
| `kHIDUsage_Csmr_ALWordProcessor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ALWordProcessor")` |
| `kHIDUsage_Csmr_AMOrPM` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_AMOrPM")` |
| `kHIDUsage_Csmr_AlternateAudioDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_AlternateAudioDecrement")` |
| `kHIDUsage_Csmr_AlternateAudioIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_AlternateAudioIncrement")` |
| `kHIDUsage_Csmr_ApplicationLaunchButtons` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ApplicationLaunchButtons")` |
| `kHIDUsage_Csmr_Aspect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Aspect")` |
| `kHIDUsage_Csmr_Assign` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Assign")` |
| `kHIDUsage_Csmr_Balance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Balance")` |
| `kHIDUsage_Csmr_BalanceLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BalanceLeft")` |
| `kHIDUsage_Csmr_BalanceRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BalanceRight")` |
| `kHIDUsage_Csmr_Bass` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Bass")` |
| `kHIDUsage_Csmr_BassBoost` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BassBoost")` |
| `kHIDUsage_Csmr_BassDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BassDecrement")` |
| `kHIDUsage_Csmr_BassIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BassIncrement")` |
| `kHIDUsage_Csmr_BlueMenuButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BlueMenuButton")` |
| `kHIDUsage_Csmr_BroadcastMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_BroadcastMode")` |
| `kHIDUsage_Csmr_Channel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Channel")` |
| `kHIDUsage_Csmr_ChannelCenter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelCenter")` |
| `kHIDUsage_Csmr_ChannelCenterFront` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelCenterFront")` |
| `kHIDUsage_Csmr_ChannelDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelDecrement")` |
| `kHIDUsage_Csmr_ChannelFront` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelFront")` |
| `kHIDUsage_Csmr_ChannelIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelIncrement")` |
| `kHIDUsage_Csmr_ChannelLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelLeft")` |
| `kHIDUsage_Csmr_ChannelLowFrequencyEnhancement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelLowFrequencyEnhancement")` |
| `kHIDUsage_Csmr_ChannelRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelRight")` |
| `kHIDUsage_Csmr_ChannelSide` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelSide")` |
| `kHIDUsage_Csmr_ChannelSurround` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelSurround")` |
| `kHIDUsage_Csmr_ChannelTop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelTop")` |
| `kHIDUsage_Csmr_ChannelUnknown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ChannelUnknown")` |
| `kHIDUsage_Csmr_ClearMark` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ClearMark")` |
| `kHIDUsage_Csmr_ClimateControlEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ClimateControlEnable")` |
| `kHIDUsage_Csmr_ClosedCaption` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ClosedCaption")` |
| `kHIDUsage_Csmr_ClosedCaptionSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ClosedCaptionSelect")` |
| `kHIDUsage_Csmr_ConsumerControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ConsumerControl")` |
| `kHIDUsage_Csmr_ContactAdded` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactAdded")` |
| `kHIDUsage_Csmr_ContactEdited` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactEdited")` |
| `kHIDUsage_Csmr_ContactEmailBusiness` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactEmailBusiness")` |
| `kHIDUsage_Csmr_ContactEmailMain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactEmailMain")` |
| `kHIDUsage_Csmr_ContactEmailOther` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactEmailOther")` |
| `kHIDUsage_Csmr_ContactEmailPersonal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactEmailPersonal")` |
| `kHIDUsage_Csmr_ContactFirstName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactFirstName")` |
| `kHIDUsage_Csmr_ContactFullName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactFullName")` |
| `kHIDUsage_Csmr_ContactIndex` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactIndex")` |
| `kHIDUsage_Csmr_ContactLastName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactLastName")` |
| `kHIDUsage_Csmr_ContactMisc` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactMisc")` |
| `kHIDUsage_Csmr_ContactNickname` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactNickname")` |
| `kHIDUsage_Csmr_ContactPhoneNumberBusiness` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberBusiness")` |
| `kHIDUsage_Csmr_ContactPhoneNumberFax` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberFax")` |
| `kHIDUsage_Csmr_ContactPhoneNumberMobile` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberMobile")` |
| `kHIDUsage_Csmr_ContactPhoneNumberOther` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberOther")` |
| `kHIDUsage_Csmr_ContactPhoneNumberPager` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberPager")` |
| `kHIDUsage_Csmr_ContactPhoneNumberPersonal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactPhoneNumberPersonal")` |
| `kHIDUsage_Csmr_ContactRecordActive` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactRecordActive")` |
| `kHIDUsage_Csmr_ContactSpeedDialNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactSpeedDialNumber")` |
| `kHIDUsage_Csmr_ContactStatusFlag` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ContactStatusFlag")` |
| `kHIDUsage_Csmr_CounterReset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_CounterReset")` |
| `kHIDUsage_Csmr_Daily` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Daily")` |
| `kHIDUsage_Csmr_DataOnScreen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DataOnScreen")` |
| `kHIDUsage_Csmr_DisplayBacklightToggle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBacklightToggle")` |
| `kHIDUsage_Csmr_DisplayBrightness` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightness")` |
| `kHIDUsage_Csmr_DisplayBrightnessDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightnessDecrement")` |
| `kHIDUsage_Csmr_DisplayBrightnessIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightnessIncrement")` |
| `kHIDUsage_Csmr_DisplayBrightnessMaximum` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightnessMaximum")` |
| `kHIDUsage_Csmr_DisplayBrightnessMinimum` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightnessMinimum")` |
| `kHIDUsage_Csmr_DisplayBrightnessSetAutoBrightness` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DisplayBrightnessSetAutoBrightness")` |
| `kHIDUsage_Csmr_DuressAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_DuressAlarm")` |
| `kHIDUsage_Csmr_Eject` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Eject")` |
| `kHIDUsage_Csmr_EnterChannel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_EnterChannel")` |
| `kHIDUsage_Csmr_EnterDisc` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_EnterDisc")` |
| `kHIDUsage_Csmr_ExtendedKeyboardAttributesCollection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ExtendedKeyboardAttributesCollection")` |
| `kHIDUsage_Csmr_ExtendedPlay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ExtendedPlay")` |
| `kHIDUsage_Csmr_FanEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FanEnable")` |
| `kHIDUsage_Csmr_FanSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FanSpeed")` |
| `kHIDUsage_Csmr_FastForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FastForward")` |
| `kHIDUsage_Csmr_FireAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FireAlarm")` |
| `kHIDUsage_Csmr_FrameBack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FrameBack")` |
| `kHIDUsage_Csmr_FrameForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FrameForward")` |
| `kHIDUsage_Csmr_FunctionButtons` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_FunctionButtons")` |
| `kHIDUsage_Csmr_GenericGUIApplicationControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_GenericGUIApplicationControls")` |
| `kHIDUsage_Csmr_GraphicEqualizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_GraphicEqualizer")` |
| `kHIDUsage_Csmr_GreenMenuButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_GreenMenuButton")` |
| `kHIDUsage_Csmr_Headphone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Headphone")` |
| `kHIDUsage_Csmr_Help` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Help")` |
| `kHIDUsage_Csmr_HoldupAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_HoldupAlarm")` |
| `kHIDUsage_Csmr_Illumination` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Illumination")` |
| `kHIDUsage_Csmr_ImplementedKeyboardInputAssistControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ImplementedKeyboardInputAssistControls")` |
| `kHIDUsage_Csmr_KeyboardBrightnessDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardBrightnessDecrement")` |
| `kHIDUsage_Csmr_KeyboardBrightnessIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardBrightnessIncrement")` |
| `kHIDUsage_Csmr_KeyboardFormFactor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardFormFactor")` |
| `kHIDUsage_Csmr_KeyboardIETFLanguageTagIndex` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardIETFLanguageTagIndex")` |
| `kHIDUsage_Csmr_KeyboardInputAssistAccept` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistAccept")` |
| `kHIDUsage_Csmr_KeyboardInputAssistCancel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistCancel")` |
| `kHIDUsage_Csmr_KeyboardInputAssistNext` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistNext")` |
| `kHIDUsage_Csmr_KeyboardInputAssistNextGroup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistNextGroup")` |
| `kHIDUsage_Csmr_KeyboardInputAssistPrevious` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistPrevious")` |
| `kHIDUsage_Csmr_KeyboardInputAssistPreviousGroup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardInputAssistPreviousGroup")` |
| `kHIDUsage_Csmr_KeyboardKeyType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardKeyType")` |
| `kHIDUsage_Csmr_KeyboardPhysicalLayout` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_KeyboardPhysicalLayout")` |
| `kHIDUsage_Csmr_LightEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_LightEnable")` |
| `kHIDUsage_Csmr_LightIlluminationLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_LightIlluminationLevel")` |
| `kHIDUsage_Csmr_LongPlay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_LongPlay")` |
| `kHIDUsage_Csmr_Loudness` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Loudness")` |
| `kHIDUsage_Csmr_MPX` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MPX")` |
| `kHIDUsage_Csmr_Mark` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Mark")` |
| `kHIDUsage_Csmr_Media` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Media")` |
| `kHIDUsage_Csmr_MediaSelectCD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectCD")` |
| `kHIDUsage_Csmr_MediaSelectCable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectCable")` |
| `kHIDUsage_Csmr_MediaSelectCall` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectCall")` |
| `kHIDUsage_Csmr_MediaSelectComputer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectComputer")` |
| `kHIDUsage_Csmr_MediaSelectDVD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectDVD")` |
| `kHIDUsage_Csmr_MediaSelectGames` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectGames")` |
| `kHIDUsage_Csmr_MediaSelectHome` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectHome")` |
| `kHIDUsage_Csmr_MediaSelectMessages` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectMessages")` |
| `kHIDUsage_Csmr_MediaSelectProgramGuide` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectProgramGuide")` |
| `kHIDUsage_Csmr_MediaSelectSatellite` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectSatellite")` |
| `kHIDUsage_Csmr_MediaSelectSecurity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectSecurity")` |
| `kHIDUsage_Csmr_MediaSelectTV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectTV")` |
| `kHIDUsage_Csmr_MediaSelectTape` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectTape")` |
| `kHIDUsage_Csmr_MediaSelectTelephone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectTelephone")` |
| `kHIDUsage_Csmr_MediaSelectTuner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectTuner")` |
| `kHIDUsage_Csmr_MediaSelectVCR` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectVCR")` |
| `kHIDUsage_Csmr_MediaSelectVideoPhone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectVideoPhone")` |
| `kHIDUsage_Csmr_MediaSelectWWW` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelectWWW")` |
| `kHIDUsage_Csmr_MediaSelection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MediaSelection")` |
| `kHIDUsage_Csmr_MedicalAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MedicalAlarm")` |
| `kHIDUsage_Csmr_Menu` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Menu")` |
| `kHIDUsage_Csmr_MenuDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuDown")` |
| `kHIDUsage_Csmr_MenuEscape` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuEscape")` |
| `kHIDUsage_Csmr_MenuLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuLeft")` |
| `kHIDUsage_Csmr_MenuPick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuPick")` |
| `kHIDUsage_Csmr_MenuRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuRight")` |
| `kHIDUsage_Csmr_MenuUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuUp")` |
| `kHIDUsage_Csmr_MenuValueDecrease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuValueDecrease")` |
| `kHIDUsage_Csmr_MenuValueIncrease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_MenuValueIncrease")` |
| `kHIDUsage_Csmr_Microphone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Microphone")` |
| `kHIDUsage_Csmr_ModeStep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ModeStep")` |
| `kHIDUsage_Csmr_Monthly` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Monthly")` |
| `kHIDUsage_Csmr_Motion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Motion")` |
| `kHIDUsage_Csmr_Mute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Mute")` |
| `kHIDUsage_Csmr_NumericKeyPad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_NumericKeyPad")` |
| `kHIDUsage_Csmr_Once` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Once")` |
| `kHIDUsage_Csmr_OrderMovie` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_OrderMovie")` |
| `kHIDUsage_Csmr_Pause` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Pause")` |
| `kHIDUsage_Csmr_PictureInPictureSwap` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PictureInPictureSwap")` |
| `kHIDUsage_Csmr_PictureInPictureToggle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PictureInPictureToggle")` |
| `kHIDUsage_Csmr_Play` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Play")` |
| `kHIDUsage_Csmr_PlayOrPause` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PlayOrPause")` |
| `kHIDUsage_Csmr_PlayOrSkip` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PlayOrSkip")` |
| `kHIDUsage_Csmr_PlaybackSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PlaybackSpeed")` |
| `kHIDUsage_Csmr_Plus10` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Plus10")` |
| `kHIDUsage_Csmr_Plus100` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Plus100")` |
| `kHIDUsage_Csmr_PoliceAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_PoliceAlarm")` |
| `kHIDUsage_Csmr_Power` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Power")` |
| `kHIDUsage_Csmr_ProgrammableButtons` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ProgrammableButtons")` |
| `kHIDUsage_Csmr_Proximity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Proximity")` |
| `kHIDUsage_Csmr_Quit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Quit")` |
| `kHIDUsage_Csmr_RandomPlay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_RandomPlay")` |
| `kHIDUsage_Csmr_RecallLast` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_RecallLast")` |
| `kHIDUsage_Csmr_Record` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Record")` |
| `kHIDUsage_Csmr_RedMenuButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_RedMenuButton")` |
| `kHIDUsage_Csmr_Repeat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Repeat")` |
| `kHIDUsage_Csmr_RepeatFromMark` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_RepeatFromMark")` |
| `kHIDUsage_Csmr_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Reserved")` |
| `kHIDUsage_Csmr_Reset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Reset")` |
| `kHIDUsage_Csmr_ReturnToMark` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ReturnToMark")` |
| `kHIDUsage_Csmr_Rewind` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Rewind")` |
| `kHIDUsage_Csmr_RoomTemperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_RoomTemperature")` |
| `kHIDUsage_Csmr_ScanNextTrack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ScanNextTrack")` |
| `kHIDUsage_Csmr_ScanPreviousTrack` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ScanPreviousTrack")` |
| `kHIDUsage_Csmr_SearchMarkBackwards` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SearchMarkBackwards")` |
| `kHIDUsage_Csmr_SearchMarkForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SearchMarkForward")` |
| `kHIDUsage_Csmr_SecurityEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SecurityEnable")` |
| `kHIDUsage_Csmr_SelectDisc` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SelectDisc")` |
| `kHIDUsage_Csmr_Selection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Selection")` |
| `kHIDUsage_Csmr_ShowCounter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_ShowCounter")` |
| `kHIDUsage_Csmr_Sleep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Sleep")` |
| `kHIDUsage_Csmr_SleepAfter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SleepAfter")` |
| `kHIDUsage_Csmr_SleepMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SleepMode")` |
| `kHIDUsage_Csmr_Slow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Slow")` |
| `kHIDUsage_Csmr_SlowTracking` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SlowTracking")` |
| `kHIDUsage_Csmr_Snapshot` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Snapshot")` |
| `kHIDUsage_Csmr_SpeakerSystem` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SpeakerSystem")` |
| `kHIDUsage_Csmr_Speed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Speed")` |
| `kHIDUsage_Csmr_StandardPlay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_StandardPlay")` |
| `kHIDUsage_Csmr_Still` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Still")` |
| `kHIDUsage_Csmr_Stop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Stop")` |
| `kHIDUsage_Csmr_StopOrEject` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_StopOrEject")` |
| `kHIDUsage_Csmr_SubChannel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SubChannel")` |
| `kHIDUsage_Csmr_SubChannelDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SubChannelDecrement")` |
| `kHIDUsage_Csmr_SubChannelIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SubChannelIncrement")` |
| `kHIDUsage_Csmr_SurroundMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_SurroundMode")` |
| `kHIDUsage_Csmr_TrackNormal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_TrackNormal")` |
| `kHIDUsage_Csmr_Tracking` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Tracking")` |
| `kHIDUsage_Csmr_TrackingDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_TrackingDecrement")` |
| `kHIDUsage_Csmr_TrackingIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_TrackingIncrement")` |
| `kHIDUsage_Csmr_Treble` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Treble")` |
| `kHIDUsage_Csmr_TrebleDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_TrebleDecrement")` |
| `kHIDUsage_Csmr_TrebleIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_TrebleIncrement")` |
| `kHIDUsage_Csmr_VCROrTV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VCROrTV")` |
| `kHIDUsage_Csmr_VCRPlus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VCRPlus")` |
| `kHIDUsage_Csmr_VendorSpecificKeyboardPhysicalLayout` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VendorSpecificKeyboardPhysicalLayout")` |
| `kHIDUsage_Csmr_VoiceCommand` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VoiceCommand")` |
| `kHIDUsage_Csmr_Volume` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Volume")` |
| `kHIDUsage_Csmr_VolumeDecrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VolumeDecrement")` |
| `kHIDUsage_Csmr_VolumeIncrement` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_VolumeIncrement")` |
| `kHIDUsage_Csmr_Weekly` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_Weekly")` |
| `kHIDUsage_Csmr_YellowMenuButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Csmr_YellowMenuButton")` |
| `kHIDUsage_Dig_3DDigitizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_3DDigitizer")` |
| `kHIDUsage_Dig_Altitude` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Altitude")` |
| `kHIDUsage_Dig_Armature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Armature")` |
| `kHIDUsage_Dig_ArticulatedArm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ArticulatedArm")` |
| `kHIDUsage_Dig_Azimuth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Azimuth")` |
| `kHIDUsage_Dig_BarrelPressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_BarrelPressure")` |
| `kHIDUsage_Dig_BarrelSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_BarrelSwitch")` |
| `kHIDUsage_Dig_BatteryStrength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_BatteryStrength")` |
| `kHIDUsage_Dig_CapacitiveHeatMapDigitizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_CapacitiveHeatMapDigitizer")` |
| `kHIDUsage_Dig_CapacitiveHeatMapFrameData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_CapacitiveHeatMapFrameData")` |
| `kHIDUsage_Dig_CapacitiveHeatMapProtocolVendorID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_CapacitiveHeatMapProtocolVendorID")` |
| `kHIDUsage_Dig_CapacitiveHeatMapProtocolVersion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_CapacitiveHeatMapProtocolVersion")` |
| `kHIDUsage_Dig_ContactCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ContactCount")` |
| `kHIDUsage_Dig_ContactCountMaximum` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ContactCountMaximum")` |
| `kHIDUsage_Dig_ContactIdentifier` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ContactIdentifier")` |
| `kHIDUsage_Dig_CoordinateMeasuringMachine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_CoordinateMeasuringMachine")` |
| `kHIDUsage_Dig_DataValid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_DataValid")` |
| `kHIDUsage_Dig_DeviceConfiguration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_DeviceConfiguration")` |
| `kHIDUsage_Dig_DeviceIdentifier` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_DeviceIdentifier")` |
| `kHIDUsage_Dig_DeviceMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_DeviceMode")` |
| `kHIDUsage_Dig_DeviceSettings` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_DeviceSettings")` |
| `kHIDUsage_Dig_Digitizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Digitizer")` |
| `kHIDUsage_Dig_Eraser` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Eraser")` |
| `kHIDUsage_Dig_Finger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Finger")` |
| `kHIDUsage_Dig_FreeSpaceWand` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_FreeSpaceWand")` |
| `kHIDUsage_Dig_GestureCharacter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacter")` |
| `kHIDUsage_Dig_GestureCharacterData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterData")` |
| `kHIDUsage_Dig_GestureCharacterDataLength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterDataLength")` |
| `kHIDUsage_Dig_GestureCharacterEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEnable")` |
| `kHIDUsage_Dig_GestureCharacterEncoding` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncoding")` |
| `kHIDUsage_Dig_GestureCharacterEncodingUTF16BE` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncodingUTF16BE")` |
| `kHIDUsage_Dig_GestureCharacterEncodingUTF16LE` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncodingUTF16LE")` |
| `kHIDUsage_Dig_GestureCharacterEncodingUTF32BE` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncodingUTF32BE")` |
| `kHIDUsage_Dig_GestureCharacterEncodingUTF32LE` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncodingUTF32LE")` |
| `kHIDUsage_Dig_GestureCharacterEncodingUTF8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterEncodingUTF8")` |
| `kHIDUsage_Dig_GestureCharacterQuality` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_GestureCharacterQuality")` |
| `kHIDUsage_Dig_Height` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Height")` |
| `kHIDUsage_Dig_InRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_InRange")` |
| `kHIDUsage_Dig_Invert` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Invert")` |
| `kHIDUsage_Dig_LightPen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_LightPen")` |
| `kHIDUsage_Dig_MultiplePointDigitizer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_MultiplePointDigitizer")` |
| `kHIDUsage_Dig_Pen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Pen")` |
| `kHIDUsage_Dig_ProgramChangeKeys` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ProgramChangeKeys")` |
| `kHIDUsage_Dig_Puck` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Puck")` |
| `kHIDUsage_Dig_Quality` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Quality")` |
| `kHIDUsage_Dig_RelativeScanTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_RelativeScanTime")` |
| `kHIDUsage_Dig_ReportRate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_ReportRate")` |
| `kHIDUsage_Dig_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Reserved")` |
| `kHIDUsage_Dig_SecondaryTipSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_SecondaryTipSwitch")` |
| `kHIDUsage_Dig_StereoPlotter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_StereoPlotter")` |
| `kHIDUsage_Dig_Stylus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Stylus")` |
| `kHIDUsage_Dig_SurfaceSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_SurfaceSwitch")` |
| `kHIDUsage_Dig_TabletFunctionKeys` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TabletFunctionKeys")` |
| `kHIDUsage_Dig_TabletPick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TabletPick")` |
| `kHIDUsage_Dig_Tap` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Tap")` |
| `kHIDUsage_Dig_TipPressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TipPressure")` |
| `kHIDUsage_Dig_TipSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TipSwitch")` |
| `kHIDUsage_Dig_Touch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Touch")` |
| `kHIDUsage_Dig_TouchPad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TouchPad")` |
| `kHIDUsage_Dig_TouchScreen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TouchScreen")` |
| `kHIDUsage_Dig_TouchValid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TouchValid")` |
| `kHIDUsage_Dig_TransducerIndex` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_TransducerIndex")` |
| `kHIDUsage_Dig_Twist` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Twist")` |
| `kHIDUsage_Dig_Untouch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Untouch")` |
| `kHIDUsage_Dig_WhiteBoard` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_WhiteBoard")` |
| `kHIDUsage_Dig_Width` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_Width")` |
| `kHIDUsage_Dig_XTilt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_XTilt")` |
| `kHIDUsage_Dig_YTilt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Dig_YTilt")` |
| `kHIDUsage_FIDO_InputData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_FIDO_InputData")` |
| `kHIDUsage_FIDO_OutputData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_FIDO_OutputData")` |
| `kHIDUsage_FIDO_U2FDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_FIDO_U2FDevice")` |
| `kHIDUsage_FIDO_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_FIDO_Undefined")` |
| `kHIDUsage_GD_ApplicationBreak` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ApplicationBreak")` |
| `kHIDUsage_GD_ApplicationDebuggerBreak` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ApplicationDebuggerBreak")` |
| `kHIDUsage_GD_AssistiveControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_AssistiveControl")` |
| `kHIDUsage_GD_AssistiveControlCompatible` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_AssistiveControlCompatible")` |
| `kHIDUsage_GD_ByteCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ByteCount")` |
| `kHIDUsage_GD_CallActiveLED` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CallActiveLED")` |
| `kHIDUsage_GD_CallMuteLED` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CallMuteLED")` |
| `kHIDUsage_GD_CallMuteToggle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CallMuteToggle")` |
| `kHIDUsage_GD_ChassisEnclosure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ChassisEnclosure")` |
| `kHIDUsage_GD_ControlEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ControlEnable")` |
| `kHIDUsage_GD_CoolantCriticalLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CoolantCriticalLevel")` |
| `kHIDUsage_GD_CoolantLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CoolantLevel")` |
| `kHIDUsage_GD_CoolantPump` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CoolantPump")` |
| `kHIDUsage_GD_CountedBuffer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_CountedBuffer")` |
| `kHIDUsage_GD_DPadDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DPadDown")` |
| `kHIDUsage_GD_DPadLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DPadLeft")` |
| `kHIDUsage_GD_DPadRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DPadRight")` |
| `kHIDUsage_GD_DPadUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DPadUp")` |
| `kHIDUsage_GD_Dial` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Dial")` |
| `kHIDUsage_GD_DoNotDisturb` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DoNotDisturb")` |
| `kHIDUsage_GD_DockableDeviceDisplayOcclusion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDeviceDisplayOcclusion")` |
| `kHIDUsage_GD_DockableDeviceDockingState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDeviceDockingState")` |
| `kHIDUsage_GD_DockableDeviceObjectType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDeviceObjectType")` |
| `kHIDUsage_GD_DockableDevicePrimaryUsageID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDevicePrimaryUsageID")` |
| `kHIDUsage_GD_DockableDevicePrimaryUsagePage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDevicePrimaryUsagePage")` |
| `kHIDUsage_GD_DockableDeviceUniqueID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDeviceUniqueID")` |
| `kHIDUsage_GD_DockableDeviceVendorID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_DockableDeviceVendorID")` |
| `kHIDUsage_GD_FeatureNotification` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_FeatureNotification")` |
| `kHIDUsage_GD_GamePad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_GamePad")` |
| `kHIDUsage_GD_Hatswitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Hatswitch")` |
| `kHIDUsage_GD_IndexTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_IndexTrigger")` |
| `kHIDUsage_GD_Joystick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Joystick")` |
| `kHIDUsage_GD_Keyboard` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Keyboard")` |
| `kHIDUsage_GD_Keypad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Keypad")` |
| `kHIDUsage_GD_MotionWakeup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_MotionWakeup")` |
| `kHIDUsage_GD_Mouse` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Mouse")` |
| `kHIDUsage_GD_MultiAxisController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_MultiAxisController")` |
| `kHIDUsage_GD_PalmTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_PalmTrigger")` |
| `kHIDUsage_GD_Pointer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Pointer")` |
| `kHIDUsage_GD_Qw` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Qw")` |
| `kHIDUsage_GD_Qx` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Qx")` |
| `kHIDUsage_GD_Qy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Qy")` |
| `kHIDUsage_GD_Qz` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Qz")` |
| `kHIDUsage_GD_RPM` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_RPM")` |
| `kHIDUsage_GD_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Reserved")` |
| `kHIDUsage_GD_ResolutionMultiplier` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_ResolutionMultiplier")` |
| `kHIDUsage_GD_Rx` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Rx")` |
| `kHIDUsage_GD_Ry` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Ry")` |
| `kHIDUsage_GD_Rz` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Rz")` |
| `kHIDUsage_GD_SFShift` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SFShift")` |
| `kHIDUsage_GD_SFShiftLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SFShiftLock")` |
| `kHIDUsage_GD_SFShiftLockIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SFShiftLockIndicator")` |
| `kHIDUsage_GD_Select` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Select")` |
| `kHIDUsage_GD_SensorZone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SensorZone")` |
| `kHIDUsage_GD_Slider` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Slider")` |
| `kHIDUsage_GD_SpatialController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SpatialController")` |
| `kHIDUsage_GD_Start` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Start")` |
| `kHIDUsage_GD_SystemAppMenu` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemAppMenu")` |
| `kHIDUsage_GD_SystemBreak` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemBreak")` |
| `kHIDUsage_GD_SystemColdRestart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemColdRestart")` |
| `kHIDUsage_GD_SystemContextMenu` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemContextMenu")` |
| `kHIDUsage_GD_SystemControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemControl")` |
| `kHIDUsage_GD_SystemDebuggerBreak` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDebuggerBreak")` |
| `kHIDUsage_GD_SystemDismissNotification` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDismissNotification")` |
| `kHIDUsage_GD_SystemDisplayBoth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayBoth")` |
| `kHIDUsage_GD_SystemDisplayDual` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayDual")` |
| `kHIDUsage_GD_SystemDisplayExternal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayExternal")` |
| `kHIDUsage_GD_SystemDisplayInternal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayInternal")` |
| `kHIDUsage_GD_SystemDisplayInvert` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayInvert")` |
| `kHIDUsage_GD_SystemDisplayRotationLockButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayRotationLockButton")` |
| `kHIDUsage_GD_SystemDisplayRotationLockSliderSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayRotationLockSliderSwitch")` |
| `kHIDUsage_GD_SystemDisplaySwap` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplaySwap")` |
| `kHIDUsage_GD_SystemDisplayToggleLCDAutoscale` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayToggleLCDAutoscale")` |
| `kHIDUsage_GD_SystemDisplayToggleMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDisplayToggleMode")` |
| `kHIDUsage_GD_SystemDock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemDock")` |
| `kHIDUsage_GD_SystemHibernate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemHibernate")` |
| `kHIDUsage_GD_SystemMainMenu` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMainMenu")` |
| `kHIDUsage_GD_SystemMenuDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuDown")` |
| `kHIDUsage_GD_SystemMenuExit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuExit")` |
| `kHIDUsage_GD_SystemMenuHelp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuHelp")` |
| `kHIDUsage_GD_SystemMenuLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuLeft")` |
| `kHIDUsage_GD_SystemMenuRight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuRight")` |
| `kHIDUsage_GD_SystemMenuSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuSelect")` |
| `kHIDUsage_GD_SystemMenuUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMenuUp")` |
| `kHIDUsage_GD_SystemMicrophoneMute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMicrophoneMute")` |
| `kHIDUsage_GD_SystemMultiAxisController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemMultiAxisController")` |
| `kHIDUsage_GD_SystemPowerDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemPowerDown")` |
| `kHIDUsage_GD_SystemSetup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemSetup")` |
| `kHIDUsage_GD_SystemSleep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemSleep")` |
| `kHIDUsage_GD_SystemSpeakerMute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemSpeakerMute")` |
| `kHIDUsage_GD_SystemUndock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemUndock")` |
| `kHIDUsage_GD_SystemWakeUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemWakeUp")` |
| `kHIDUsage_GD_SystemWarmRestart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_SystemWarmRestart")` |
| `kHIDUsage_GD_TabletPCSystemControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_TabletPCSystemControls")` |
| `kHIDUsage_GD_Thumbstick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Thumbstick")` |
| `kHIDUsage_GD_Vbrx` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vbrx")` |
| `kHIDUsage_GD_Vbry` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vbry")` |
| `kHIDUsage_GD_Vbrz` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vbrz")` |
| `kHIDUsage_GD_Vno` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vno")` |
| `kHIDUsage_GD_Vx` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vx")` |
| `kHIDUsage_GD_Vy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vy")` |
| `kHIDUsage_GD_Vz` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Vz")` |
| `kHIDUsage_GD_Wheel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Wheel")` |
| `kHIDUsage_GD_WirelessRadioButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_WirelessRadioButton")` |
| `kHIDUsage_GD_WirelessRadioLED` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_WirelessRadioLED")` |
| `kHIDUsage_GD_WirelessRadioSliderSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_WirelessRadioSliderSwitch")` |
| `kHIDUsage_GD_X` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_X")` |
| `kHIDUsage_GD_Y` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Y")` |
| `kHIDUsage_GD_Z` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GD_Z")` |
| `kHIDUsage_Game_3DGameController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_3DGameController")` |
| `kHIDUsage_Game_Bump` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_Bump")` |
| `kHIDUsage_Game_Flipper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_Flipper")` |
| `kHIDUsage_Game_GamepadFireOrJump` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GamepadFireOrJump")` |
| `kHIDUsage_Game_GamepadFormFitting` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GamepadFormFitting")` |
| `kHIDUsage_Game_GamepadFormFitting_Compatibility` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GamepadFormFitting_Compatibility")` |
| `kHIDUsage_Game_GamepadTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GamepadTrigger")` |
| `kHIDUsage_Game_Gun` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_Gun")` |
| `kHIDUsage_Game_GunAutomatic` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunAutomatic")` |
| `kHIDUsage_Game_GunBolt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunBolt")` |
| `kHIDUsage_Game_GunBurst` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunBurst")` |
| `kHIDUsage_Game_GunClip` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunClip")` |
| `kHIDUsage_Game_GunDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunDevice")` |
| `kHIDUsage_Game_GunSafety` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunSafety")` |
| `kHIDUsage_Game_GunSingleShot` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_GunSingleShot")` |
| `kHIDUsage_Game_HeightOfPOV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_HeightOfPOV")` |
| `kHIDUsage_Game_LeanForwardOrBackward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_LeanForwardOrBackward")` |
| `kHIDUsage_Game_LeanRightOrLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_LeanRightOrLeft")` |
| `kHIDUsage_Game_MoveForwardOrBackward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_MoveForwardOrBackward")` |
| `kHIDUsage_Game_MoveRightOrLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_MoveRightOrLeft")` |
| `kHIDUsage_Game_MoveUpOrDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_MoveUpOrDown")` |
| `kHIDUsage_Game_NewGame` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_NewGame")` |
| `kHIDUsage_Game_PinballDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_PinballDevice")` |
| `kHIDUsage_Game_PitchUpOrDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_PitchUpOrDown")` |
| `kHIDUsage_Game_Player` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_Player")` |
| `kHIDUsage_Game_PointofView` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_PointofView")` |
| `kHIDUsage_Game_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_Reserved")` |
| `kHIDUsage_Game_RollRightOrLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_RollRightOrLeft")` |
| `kHIDUsage_Game_SecondaryFlipper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_SecondaryFlipper")` |
| `kHIDUsage_Game_ShootBall` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_ShootBall")` |
| `kHIDUsage_Game_TurnRightOrLeft` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Game_TurnRightOrLeft")` |
| `kHIDUsage_GenDevControls_BackgroundControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GenDevControls_BackgroundControls")` |
| `kHIDUsage_GenDevControls_BatteryStrength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_GenDevControls_BatteryStrength")` |
| `kHIDUsage_Haptics_AutoTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_AutoTrigger")` |
| `kHIDUsage_Haptics_AutoTriggerAssociatedControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_AutoTriggerAssociatedControl")` |
| `kHIDUsage_Haptics_DurationList` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_DurationList")` |
| `kHIDUsage_Haptics_Intensity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_Intensity")` |
| `kHIDUsage_Haptics_ManualTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_ManualTrigger")` |
| `kHIDUsage_Haptics_RepeatCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_RepeatCount")` |
| `kHIDUsage_Haptics_RetriggerPeriod` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_RetriggerPeriod")` |
| `kHIDUsage_Haptics_SimpleHapticController` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_SimpleHapticController")` |
| `kHIDUsage_Haptics_VendorWaveformFirst` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_VendorWaveformFirst")` |
| `kHIDUsage_Haptics_VendorWaveformLast` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_VendorWaveformLast")` |
| `kHIDUsage_Haptics_WaveformBuzzContinuous` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformBuzzContinuous")` |
| `kHIDUsage_Haptics_WaveformClick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformClick")` |
| `kHIDUsage_Haptics_WaveformCutoffTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformCutoffTime")` |
| `kHIDUsage_Haptics_WaveformList` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformList")` |
| `kHIDUsage_Haptics_WaveformNone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformNone")` |
| `kHIDUsage_Haptics_WaveformPress` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformPress")` |
| `kHIDUsage_Haptics_WaveformRelease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformRelease")` |
| `kHIDUsage_Haptics_WaveformRumbleContinuous` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformRumbleContinuous")` |
| `kHIDUsage_Haptics_WaveformStop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformStop")` |
| `kHIDUsage_Haptics_WaveformVendorID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformVendorID")` |
| `kHIDUsage_Haptics_WaveformVendorPage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Haptics_WaveformVendorPage")` |
| `kHIDUsage_Keyboard0` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard0")` |
| `kHIDUsage_Keyboard1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard1")` |
| `kHIDUsage_Keyboard2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard2")` |
| `kHIDUsage_Keyboard3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard3")` |
| `kHIDUsage_Keyboard4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard4")` |
| `kHIDUsage_Keyboard5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard5")` |
| `kHIDUsage_Keyboard6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard6")` |
| `kHIDUsage_Keyboard7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard7")` |
| `kHIDUsage_Keyboard8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard8")` |
| `kHIDUsage_Keyboard9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard9")` |
| `kHIDUsage_KeyboardA` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardA")` |
| `kHIDUsage_KeyboardAgain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardAgain")` |
| `kHIDUsage_KeyboardAlternateErase` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardAlternateErase")` |
| `kHIDUsage_KeyboardApplication` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardApplication")` |
| `kHIDUsage_KeyboardB` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardB")` |
| `kHIDUsage_KeyboardBackslash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardBackslash")` |
| `kHIDUsage_KeyboardC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardC")` |
| `kHIDUsage_KeyboardCancel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCancel")` |
| `kHIDUsage_KeyboardCapsLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCapsLock")` |
| `kHIDUsage_KeyboardClear` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardClear")` |
| `kHIDUsage_KeyboardClearOrAgain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardClearOrAgain")` |
| `kHIDUsage_KeyboardCloseBracket` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCloseBracket")` |
| `kHIDUsage_KeyboardComma` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardComma")` |
| `kHIDUsage_KeyboardCopy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCopy")` |
| `kHIDUsage_KeyboardCrSelOrProps` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCrSelOrProps")` |
| `kHIDUsage_KeyboardCut` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardCut")` |
| `kHIDUsage_KeyboardD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardD")` |
| `kHIDUsage_KeyboardDeleteForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardDeleteForward")` |
| `kHIDUsage_KeyboardDeleteOrBackspace` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardDeleteOrBackspace")` |
| `kHIDUsage_KeyboardDownArrow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardDownArrow")` |
| `kHIDUsage_KeyboardE` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardE")` |
| `kHIDUsage_KeyboardEnd` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardEnd")` |
| `kHIDUsage_KeyboardEqualSign` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardEqualSign")` |
| `kHIDUsage_KeyboardErrorRollOver` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardErrorRollOver")` |
| `kHIDUsage_KeyboardErrorUndefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardErrorUndefined")` |
| `kHIDUsage_KeyboardEscape` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardEscape")` |
| `kHIDUsage_KeyboardExSel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardExSel")` |
| `kHIDUsage_KeyboardExecute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardExecute")` |
| `kHIDUsage_KeyboardF` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF")` |
| `kHIDUsage_KeyboardF1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF1")` |
| `kHIDUsage_KeyboardF10` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF10")` |
| `kHIDUsage_KeyboardF11` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF11")` |
| `kHIDUsage_KeyboardF12` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF12")` |
| `kHIDUsage_KeyboardF13` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF13")` |
| `kHIDUsage_KeyboardF14` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF14")` |
| `kHIDUsage_KeyboardF15` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF15")` |
| `kHIDUsage_KeyboardF16` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF16")` |
| `kHIDUsage_KeyboardF17` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF17")` |
| `kHIDUsage_KeyboardF18` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF18")` |
| `kHIDUsage_KeyboardF19` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF19")` |
| `kHIDUsage_KeyboardF2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF2")` |
| `kHIDUsage_KeyboardF20` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF20")` |
| `kHIDUsage_KeyboardF21` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF21")` |
| `kHIDUsage_KeyboardF22` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF22")` |
| `kHIDUsage_KeyboardF23` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF23")` |
| `kHIDUsage_KeyboardF24` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF24")` |
| `kHIDUsage_KeyboardF3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF3")` |
| `kHIDUsage_KeyboardF4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF4")` |
| `kHIDUsage_KeyboardF5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF5")` |
| `kHIDUsage_KeyboardF6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF6")` |
| `kHIDUsage_KeyboardF7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF7")` |
| `kHIDUsage_KeyboardF8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF8")` |
| `kHIDUsage_KeyboardF9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardF9")` |
| `kHIDUsage_KeyboardFind` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardFind")` |
| `kHIDUsage_KeyboardG` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardG")` |
| `kHIDUsage_KeyboardGraveAccentAndTilde` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardGraveAccentAndTilde")` |
| `kHIDUsage_KeyboardH` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardH")` |
| `kHIDUsage_KeyboardHelp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardHelp")` |
| `kHIDUsage_KeyboardHome` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardHome")` |
| `kHIDUsage_KeyboardHyphen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardHyphen")` |
| `kHIDUsage_KeyboardI` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardI")` |
| `kHIDUsage_KeyboardInsert` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInsert")` |
| `kHIDUsage_KeyboardInternational1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational1")` |
| `kHIDUsage_KeyboardInternational2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational2")` |
| `kHIDUsage_KeyboardInternational3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational3")` |
| `kHIDUsage_KeyboardInternational4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational4")` |
| `kHIDUsage_KeyboardInternational5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational5")` |
| `kHIDUsage_KeyboardInternational6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational6")` |
| `kHIDUsage_KeyboardInternational7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational7")` |
| `kHIDUsage_KeyboardInternational8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational8")` |
| `kHIDUsage_KeyboardInternational9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardInternational9")` |
| `kHIDUsage_KeyboardJ` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardJ")` |
| `kHIDUsage_KeyboardK` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardK")` |
| `kHIDUsage_KeyboardL` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardL")` |
| `kHIDUsage_KeyboardLANG1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG1")` |
| `kHIDUsage_KeyboardLANG2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG2")` |
| `kHIDUsage_KeyboardLANG3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG3")` |
| `kHIDUsage_KeyboardLANG4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG4")` |
| `kHIDUsage_KeyboardLANG5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG5")` |
| `kHIDUsage_KeyboardLANG6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG6")` |
| `kHIDUsage_KeyboardLANG7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG7")` |
| `kHIDUsage_KeyboardLANG8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG8")` |
| `kHIDUsage_KeyboardLANG9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLANG9")` |
| `kHIDUsage_KeyboardLeftAlt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLeftAlt")` |
| `kHIDUsage_KeyboardLeftArrow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLeftArrow")` |
| `kHIDUsage_KeyboardLeftControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLeftControl")` |
| `kHIDUsage_KeyboardLeftGUI` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLeftGUI")` |
| `kHIDUsage_KeyboardLeftShift` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLeftShift")` |
| `kHIDUsage_KeyboardLockingCapsLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLockingCapsLock")` |
| `kHIDUsage_KeyboardLockingNumLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLockingNumLock")` |
| `kHIDUsage_KeyboardLockingScrollLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardLockingScrollLock")` |
| `kHIDUsage_KeyboardM` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardM")` |
| `kHIDUsage_KeyboardMenu` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardMenu")` |
| `kHIDUsage_KeyboardMute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardMute")` |
| `kHIDUsage_KeyboardN` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardN")` |
| `kHIDUsage_KeyboardNonUSBackslash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardNonUSBackslash")` |
| `kHIDUsage_KeyboardNonUSPound` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardNonUSPound")` |
| `kHIDUsage_KeyboardO` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardO")` |
| `kHIDUsage_KeyboardOpenBracket` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardOpenBracket")` |
| `kHIDUsage_KeyboardOper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardOper")` |
| `kHIDUsage_KeyboardOut` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardOut")` |
| `kHIDUsage_KeyboardP` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardP")` |
| `kHIDUsage_KeyboardPOSTFail` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPOSTFail")` |
| `kHIDUsage_KeyboardPageDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPageDown")` |
| `kHIDUsage_KeyboardPageUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPageUp")` |
| `kHIDUsage_KeyboardPaste` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPaste")` |
| `kHIDUsage_KeyboardPause` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPause")` |
| `kHIDUsage_KeyboardPeriod` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPeriod")` |
| `kHIDUsage_KeyboardPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPower")` |
| `kHIDUsage_KeyboardPrintScreen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPrintScreen")` |
| `kHIDUsage_KeyboardPrior` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardPrior")` |
| `kHIDUsage_KeyboardQ` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardQ")` |
| `kHIDUsage_KeyboardQuote` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardQuote")` |
| `kHIDUsage_KeyboardR` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardR")` |
| `kHIDUsage_KeyboardReturn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardReturn")` |
| `kHIDUsage_KeyboardReturnOrEnter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardReturnOrEnter")` |
| `kHIDUsage_KeyboardRightAlt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardRightAlt")` |
| `kHIDUsage_KeyboardRightArrow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardRightArrow")` |
| `kHIDUsage_KeyboardRightControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardRightControl")` |
| `kHIDUsage_KeyboardRightGUI` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardRightGUI")` |
| `kHIDUsage_KeyboardRightShift` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardRightShift")` |
| `kHIDUsage_KeyboardS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardS")` |
| `kHIDUsage_KeyboardScrollLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardScrollLock")` |
| `kHIDUsage_KeyboardSelect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSelect")` |
| `kHIDUsage_KeyboardSemicolon` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSemicolon")` |
| `kHIDUsage_KeyboardSeparator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSeparator")` |
| `kHIDUsage_KeyboardSlash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSlash")` |
| `kHIDUsage_KeyboardSpacebar` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSpacebar")` |
| `kHIDUsage_KeyboardStop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardStop")` |
| `kHIDUsage_KeyboardSysReqOrAttention` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardSysReqOrAttention")` |
| `kHIDUsage_KeyboardT` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardT")` |
| `kHIDUsage_KeyboardTab` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardTab")` |
| `kHIDUsage_KeyboardU` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardU")` |
| `kHIDUsage_KeyboardUndo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardUndo")` |
| `kHIDUsage_KeyboardUpArrow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardUpArrow")` |
| `kHIDUsage_KeyboardV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardV")` |
| `kHIDUsage_KeyboardVolumeDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardVolumeDown")` |
| `kHIDUsage_KeyboardVolumeUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardVolumeUp")` |
| `kHIDUsage_KeyboardW` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardW")` |
| `kHIDUsage_KeyboardX` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardX")` |
| `kHIDUsage_KeyboardY` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardY")` |
| `kHIDUsage_KeyboardZ` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeyboardZ")` |
| `kHIDUsage_Keyboard_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keyboard_Reserved")` |
| `kHIDUsage_Keypad0` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad0")` |
| `kHIDUsage_Keypad1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad1")` |
| `kHIDUsage_Keypad2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad2")` |
| `kHIDUsage_Keypad3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad3")` |
| `kHIDUsage_Keypad4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad4")` |
| `kHIDUsage_Keypad5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad5")` |
| `kHIDUsage_Keypad6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad6")` |
| `kHIDUsage_Keypad7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad7")` |
| `kHIDUsage_Keypad8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad8")` |
| `kHIDUsage_Keypad9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Keypad9")` |
| `kHIDUsage_KeypadAsterisk` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadAsterisk")` |
| `kHIDUsage_KeypadComma` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadComma")` |
| `kHIDUsage_KeypadEnter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadEnter")` |
| `kHIDUsage_KeypadEqualSign` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadEqualSign")` |
| `kHIDUsage_KeypadEqualSignAS400` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadEqualSignAS400")` |
| `kHIDUsage_KeypadHyphen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadHyphen")` |
| `kHIDUsage_KeypadNumLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadNumLock")` |
| `kHIDUsage_KeypadPeriod` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadPeriod")` |
| `kHIDUsage_KeypadPlus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadPlus")` |
| `kHIDUsage_KeypadSlash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_KeypadSlash")` |
| `kHIDUsage_LED_BatteryLow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_BatteryLow")` |
| `kHIDUsage_LED_BatteryOK` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_BatteryOK")` |
| `kHIDUsage_LED_BatteryOperation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_BatteryOperation")` |
| `kHIDUsage_LED_BlueLEDChannel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_BlueLEDChannel")` |
| `kHIDUsage_LED_Busy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Busy")` |
| `kHIDUsage_LED_CAV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CAV")` |
| `kHIDUsage_LED_CLV` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CLV")` |
| `kHIDUsage_LED_CallPickup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CallPickup")` |
| `kHIDUsage_LED_CameraOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CameraOff")` |
| `kHIDUsage_LED_CameraOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CameraOn")` |
| `kHIDUsage_LED_CapsLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_CapsLock")` |
| `kHIDUsage_LED_Compose` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Compose")` |
| `kHIDUsage_LED_Conference` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Conference")` |
| `kHIDUsage_LED_Coverage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Coverage")` |
| `kHIDUsage_LED_DataMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_DataMode")` |
| `kHIDUsage_LED_DoNotDisturb` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_DoNotDisturb")` |
| `kHIDUsage_LED_EqualizerEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_EqualizerEnable")` |
| `kHIDUsage_LED_Error` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Error")` |
| `kHIDUsage_LED_ExternalPowerConnected` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_ExternalPowerConnected")` |
| `kHIDUsage_LED_FastBlinkOffTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_FastBlinkOffTime")` |
| `kHIDUsage_LED_FastBlinkOnTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_FastBlinkOnTime")` |
| `kHIDUsage_LED_FastForward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_FastForward")` |
| `kHIDUsage_LED_FlashOnTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_FlashOnTime")` |
| `kHIDUsage_LED_Forward` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Forward")` |
| `kHIDUsage_LED_GenericIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_GenericIndicator")` |
| `kHIDUsage_LED_GoodStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_GoodStatus")` |
| `kHIDUsage_LED_GreenLEDChannel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_GreenLEDChannel")` |
| `kHIDUsage_LED_HeadSet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_HeadSet")` |
| `kHIDUsage_LED_HighCutFilter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_HighCutFilter")` |
| `kHIDUsage_LED_Hold` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Hold")` |
| `kHIDUsage_LED_IndicatorAmber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorAmber")` |
| `kHIDUsage_LED_IndicatorBlue` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorBlue")` |
| `kHIDUsage_LED_IndicatorFastBlink` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorFastBlink")` |
| `kHIDUsage_LED_IndicatorFlash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorFlash")` |
| `kHIDUsage_LED_IndicatorGreen` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorGreen")` |
| `kHIDUsage_LED_IndicatorOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorOff")` |
| `kHIDUsage_LED_IndicatorOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorOn")` |
| `kHIDUsage_LED_IndicatorOrange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorOrange")` |
| `kHIDUsage_LED_IndicatorRed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorRed")` |
| `kHIDUsage_LED_IndicatorSlowBlink` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_IndicatorSlowBlink")` |
| `kHIDUsage_LED_Kana` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Kana")` |
| `kHIDUsage_LED_LEDIntensity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_LEDIntensity")` |
| `kHIDUsage_LED_LowCutFilter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_LowCutFilter")` |
| `kHIDUsage_LED_MessageWaiting` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_MessageWaiting")` |
| `kHIDUsage_LED_Microphone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Microphone")` |
| `kHIDUsage_LED_Mute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Mute")` |
| `kHIDUsage_LED_NightMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_NightMode")` |
| `kHIDUsage_LED_NumLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_NumLock")` |
| `kHIDUsage_LED_OffHook` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_OffHook")` |
| `kHIDUsage_LED_OffLine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_OffLine")` |
| `kHIDUsage_LED_OnLine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_OnLine")` |
| `kHIDUsage_LED_PaperJam` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_PaperJam")` |
| `kHIDUsage_LED_PaperOut` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_PaperOut")` |
| `kHIDUsage_LED_Pause` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Pause")` |
| `kHIDUsage_LED_Play` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Play")` |
| `kHIDUsage_LED_Player1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player1")` |
| `kHIDUsage_LED_Player2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player2")` |
| `kHIDUsage_LED_Player3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player3")` |
| `kHIDUsage_LED_Player4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player4")` |
| `kHIDUsage_LED_Player5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player5")` |
| `kHIDUsage_LED_Player6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player6")` |
| `kHIDUsage_LED_Player7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player7")` |
| `kHIDUsage_LED_Player8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Player8")` |
| `kHIDUsage_LED_PlayerIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_PlayerIndicator")` |
| `kHIDUsage_LED_Power` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Power")` |
| `kHIDUsage_LED_RGB_LED` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_RGB_LED")` |
| `kHIDUsage_LED_Ready` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Ready")` |
| `kHIDUsage_LED_Record` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Record")` |
| `kHIDUsage_LED_RecordingFormatDetect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_RecordingFormatDetect")` |
| `kHIDUsage_LED_RedLEDChannel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_RedLEDChannel")` |
| `kHIDUsage_LED_Remote` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Remote")` |
| `kHIDUsage_LED_Repeat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Repeat")` |
| `kHIDUsage_LED_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Reserved")` |
| `kHIDUsage_LED_Reverse` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Reverse")` |
| `kHIDUsage_LED_Rewind` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Rewind")` |
| `kHIDUsage_LED_Ring` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Ring")` |
| `kHIDUsage_LED_SamplingRateDetect` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SamplingRateDetect")` |
| `kHIDUsage_LED_ScrollLock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_ScrollLock")` |
| `kHIDUsage_LED_SendCalls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SendCalls")` |
| `kHIDUsage_LED_Shift` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Shift")` |
| `kHIDUsage_LED_SlowBlinkOffTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SlowBlinkOffTime")` |
| `kHIDUsage_LED_SlowBlinkOnTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SlowBlinkOnTime")` |
| `kHIDUsage_LED_SoundFieldOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SoundFieldOn")` |
| `kHIDUsage_LED_Speaker` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Speaker")` |
| `kHIDUsage_LED_Spinning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Spinning")` |
| `kHIDUsage_LED_StandBy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_StandBy")` |
| `kHIDUsage_LED_Stereo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Stereo")` |
| `kHIDUsage_LED_Stop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Stop")` |
| `kHIDUsage_LED_SurroundOn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SurroundOn")` |
| `kHIDUsage_LED_SystemMicrophoneMute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SystemMicrophoneMute")` |
| `kHIDUsage_LED_SystemSuspend` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_SystemSuspend")` |
| `kHIDUsage_LED_ToneEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_ToneEnable")` |
| `kHIDUsage_LED_Usage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_Usage")` |
| `kHIDUsage_LED_UsageInUseIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_UsageInUseIndicator")` |
| `kHIDUsage_LED_UsageIndicatorColor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_UsageIndicatorColor")` |
| `kHIDUsage_LED_UsageMultiModeIndicator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_UsageMultiModeIndicator")` |
| `kHIDUsage_LED_WarningStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_LED_WarningStatus")` |
| `kHIDUsage_MSR_DeviceReadOnly` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_DeviceReadOnly")` |
| `kHIDUsage_MSR_Track1Data` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track1Data")` |
| `kHIDUsage_MSR_Track1Length` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track1Length")` |
| `kHIDUsage_MSR_Track2Data` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track2Data")` |
| `kHIDUsage_MSR_Track2Length` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track2Length")` |
| `kHIDUsage_MSR_Track3Data` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track3Data")` |
| `kHIDUsage_MSR_Track3Length` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Track3Length")` |
| `kHIDUsage_MSR_TrackData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_TrackData")` |
| `kHIDUsage_MSR_TrackJISData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_TrackJISData")` |
| `kHIDUsage_MSR_TrackJISLength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_TrackJISLength")` |
| `kHIDUsage_MSR_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_MSR_Undefined")` |
| `kHIDUsage_Ord_Instance1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Ord_Instance1")` |
| `kHIDUsage_Ord_Instance2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Ord_Instance2")` |
| `kHIDUsage_Ord_Instance3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Ord_Instance3")` |
| `kHIDUsage_Ord_Instance4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Ord_Instance4")` |
| `kHIDUsage_Ord_Instance65535` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Ord_Instance65535")` |
| `kHIDUsage_PD_ActivePower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ActivePower")` |
| `kHIDUsage_PD_ApparentPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ApparentPower")` |
| `kHIDUsage_PD_AudibleAlarmControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_AudibleAlarmControl")` |
| `kHIDUsage_PD_AwaitingPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_AwaitingPower")` |
| `kHIDUsage_PD_BadCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_BadCount")` |
| `kHIDUsage_PD_Battery` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Battery")` |
| `kHIDUsage_PD_BatteryID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_BatteryID")` |
| `kHIDUsage_PD_BatterySystem` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_BatterySystem")` |
| `kHIDUsage_PD_BatterySystemID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_BatterySystemID")` |
| `kHIDUsage_PD_Boost` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Boost")` |
| `kHIDUsage_PD_Buck` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Buck")` |
| `kHIDUsage_PD_ChangedStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ChangedStatus")` |
| `kHIDUsage_PD_Charger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Charger")` |
| `kHIDUsage_PD_ChargerID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ChargerID")` |
| `kHIDUsage_PD_CommunicationLost` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_CommunicationLost")` |
| `kHIDUsage_PD_ConfigActivePower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigActivePower")` |
| `kHIDUsage_PD_ConfigApparentPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigApparentPower")` |
| `kHIDUsage_PD_ConfigCurrent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigCurrent")` |
| `kHIDUsage_PD_ConfigFrequency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigFrequency")` |
| `kHIDUsage_PD_ConfigHumidity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigHumidity")` |
| `kHIDUsage_PD_ConfigPercentLoad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigPercentLoad")` |
| `kHIDUsage_PD_ConfigTemperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigTemperature")` |
| `kHIDUsage_PD_ConfigVoltage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ConfigVoltage")` |
| `kHIDUsage_PD_Current` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Current")` |
| `kHIDUsage_PD_DelayBeforeReboot` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_DelayBeforeReboot")` |
| `kHIDUsage_PD_DelayBeforeShutdown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_DelayBeforeShutdown")` |
| `kHIDUsage_PD_DelayBeforeStartup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_DelayBeforeStartup")` |
| `kHIDUsage_PD_Flow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Flow")` |
| `kHIDUsage_PD_FlowID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_FlowID")` |
| `kHIDUsage_PD_Frequency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Frequency")` |
| `kHIDUsage_PD_FrequencyOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_FrequencyOutOfRange")` |
| `kHIDUsage_PD_Gang` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Gang")` |
| `kHIDUsage_PD_GangID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_GangID")` |
| `kHIDUsage_PD_Good` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Good")` |
| `kHIDUsage_PD_HighVoltageTransfer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_HighVoltageTransfer")` |
| `kHIDUsage_PD_Humidity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Humidity")` |
| `kHIDUsage_PD_Initialized` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Initialized")` |
| `kHIDUsage_PD_Input` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Input")` |
| `kHIDUsage_PD_InputID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_InputID")` |
| `kHIDUsage_PD_InternalFailure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_InternalFailure")` |
| `kHIDUsage_PD_LowVoltageTransfer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_LowVoltageTransfer")` |
| `kHIDUsage_PD_ModuleReset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ModuleReset")` |
| `kHIDUsage_PD_Outlet` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Outlet")` |
| `kHIDUsage_PD_OutletID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OutletID")` |
| `kHIDUsage_PD_OutletSystem` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OutletSystem")` |
| `kHIDUsage_PD_OutletSystemID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OutletSystemID")` |
| `kHIDUsage_PD_Output` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Output")` |
| `kHIDUsage_PD_OutputID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OutputID")` |
| `kHIDUsage_PD_OverCharged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OverCharged")` |
| `kHIDUsage_PD_OverTemperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_OverTemperature")` |
| `kHIDUsage_PD_Overload` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Overload")` |
| `kHIDUsage_PD_PercentLoad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PercentLoad")` |
| `kHIDUsage_PD_PeripheralDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PeripheralDevice")` |
| `kHIDUsage_PD_PowerConverter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PowerConverter")` |
| `kHIDUsage_PD_PowerConverterID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PowerConverterID")` |
| `kHIDUsage_PD_PowerSummary` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PowerSummary")` |
| `kHIDUsage_PD_PowerSummaryID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PowerSummaryID")` |
| `kHIDUsage_PD_PowerSupply` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PowerSupply")` |
| `kHIDUsage_PD_Present` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Present")` |
| `kHIDUsage_PD_PresentStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_PresentStatus")` |
| `kHIDUsage_PD_ShutdownImminent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ShutdownImminent")` |
| `kHIDUsage_PD_ShutdownRequested` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ShutdownRequested")` |
| `kHIDUsage_PD_SwitchOffControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_SwitchOffControl")` |
| `kHIDUsage_PD_SwitchOnControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_SwitchOnControl")` |
| `kHIDUsage_PD_SwitchOnOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_SwitchOnOff")` |
| `kHIDUsage_PD_Switchable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Switchable")` |
| `kHIDUsage_PD_Temperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Temperature")` |
| `kHIDUsage_PD_Test` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Test")` |
| `kHIDUsage_PD_Tested` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Tested")` |
| `kHIDUsage_PD_ToggleControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_ToggleControl")` |
| `kHIDUsage_PD_UPS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_UPS")` |
| `kHIDUsage_PD_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Undefined")` |
| `kHIDUsage_PD_Used` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Used")` |
| `kHIDUsage_PD_Voltage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_Voltage")` |
| `kHIDUsage_PD_VoltageOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_VoltageOutOfRange")` |
| `kHIDUsage_PD_iManufacturer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_iManufacturer")` |
| `kHIDUsage_PD_iName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_iName")` |
| `kHIDUsage_PD_iProduct` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_iProduct")` |
| `kHIDUsage_PD_iserialNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PD_iserialNumber")` |
| `kHIDUsage_PID_ActuatorOverrideSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ActuatorOverrideSwitch")` |
| `kHIDUsage_PID_ActuatorPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ActuatorPower")` |
| `kHIDUsage_PID_ActuatorsEnabled` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ActuatorsEnabled")` |
| `kHIDUsage_PID_AttackLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_AttackLevel")` |
| `kHIDUsage_PID_AttackTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_AttackTime")` |
| `kHIDUsage_PID_AxesEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_AxesEnable")` |
| `kHIDUsage_PID_BlockFreeReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockFreeReport")` |
| `kHIDUsage_PID_BlockHandle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockHandle")` |
| `kHIDUsage_PID_BlockLoadError` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockLoadError")` |
| `kHIDUsage_PID_BlockLoadFull` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockLoadFull")` |
| `kHIDUsage_PID_BlockLoadReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockLoadReport")` |
| `kHIDUsage_PID_BlockLoadStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockLoadStatus")` |
| `kHIDUsage_PID_BlockLoadSuccess` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockLoadSuccess")` |
| `kHIDUsage_PID_BlockType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_BlockType")` |
| `kHIDUsage_PID_CP_Offset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CP_Offset")` |
| `kHIDUsage_PID_CreateNewEffectReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CreateNewEffectReport")` |
| `kHIDUsage_PID_CustomForceData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CustomForceData")` |
| `kHIDUsage_PID_CustomForceDataOffset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CustomForceDataOffset")` |
| `kHIDUsage_PID_CustomForceDataReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CustomForceDataReport")` |
| `kHIDUsage_PID_CustomForceVendorDefinedData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_CustomForceVendorDefinedData")` |
| `kHIDUsage_PID_DC_DeviceContinue` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_DeviceContinue")` |
| `kHIDUsage_PID_DC_DevicePause` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_DevicePause")` |
| `kHIDUsage_PID_DC_DeviceReset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_DeviceReset")` |
| `kHIDUsage_PID_DC_DisableActuators` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_DisableActuators")` |
| `kHIDUsage_PID_DC_EnableActuators` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_EnableActuators")` |
| `kHIDUsage_PID_DC_StopAllEffects` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DC_StopAllEffects")` |
| `kHIDUsage_PID_DeadBand` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeadBand")` |
| `kHIDUsage_PID_DeviceControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeviceControl")` |
| `kHIDUsage_PID_DeviceControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeviceControlReport")` |
| `kHIDUsage_PID_DeviceGain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeviceGain")` |
| `kHIDUsage_PID_DeviceGainReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeviceGainReport")` |
| `kHIDUsage_PID_DeviceManagedPool` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DeviceManagedPool")` |
| `kHIDUsage_PID_DevicePaused` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DevicePaused")` |
| `kHIDUsage_PID_Direction` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Direction")` |
| `kHIDUsage_PID_DirectionEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DirectionEnable")` |
| `kHIDUsage_PID_DownloadForceSample` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_DownloadForceSample")` |
| `kHIDUsage_PID_Duration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Duration")` |
| `kHIDUsage_PID_ET_ConstantForce` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_ConstantForce")` |
| `kHIDUsage_PID_ET_CustomForceData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_CustomForceData")` |
| `kHIDUsage_PID_ET_Damper` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Damper")` |
| `kHIDUsage_PID_ET_Friction` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Friction")` |
| `kHIDUsage_PID_ET_Inertia` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Inertia")` |
| `kHIDUsage_PID_ET_Ramp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Ramp")` |
| `kHIDUsage_PID_ET_SawtoothDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_SawtoothDown")` |
| `kHIDUsage_PID_ET_SawtoothUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_SawtoothUp")` |
| `kHIDUsage_PID_ET_Sine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Sine")` |
| `kHIDUsage_PID_ET_Spring` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Spring")` |
| `kHIDUsage_PID_ET_Square` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Square")` |
| `kHIDUsage_PID_ET_Triangle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ET_Triangle")` |
| `kHIDUsage_PID_EffectBlockIndex` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_EffectBlockIndex")` |
| `kHIDUsage_PID_EffectOperation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_EffectOperation")` |
| `kHIDUsage_PID_EffectOperationReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_EffectOperationReport")` |
| `kHIDUsage_PID_EffectPlaying` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_EffectPlaying")` |
| `kHIDUsage_PID_EffectType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_EffectType")` |
| `kHIDUsage_PID_FadeLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_FadeLevel")` |
| `kHIDUsage_PID_FadeTime` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_FadeTime")` |
| `kHIDUsage_PID_Gain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Gain")` |
| `kHIDUsage_PID_IsochCustomForceEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_IsochCustomForceEnable")` |
| `kHIDUsage_PID_LoopCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_LoopCount")` |
| `kHIDUsage_PID_Magnitude` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Magnitude")` |
| `kHIDUsage_PID_MoveDestination` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_MoveDestination")` |
| `kHIDUsage_PID_MoveLength` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_MoveLength")` |
| `kHIDUsage_PID_MoveSource` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_MoveSource")` |
| `kHIDUsage_PID_NegativeCoefficient` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_NegativeCoefficient")` |
| `kHIDUsage_PID_NegativeSaturation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_NegativeSaturation")` |
| `kHIDUsage_PID_Normal` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Normal")` |
| `kHIDUsage_PID_Offset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Offset")` |
| `kHIDUsage_PID_OpEffectStart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_OpEffectStart")` |
| `kHIDUsage_PID_OpEffectStartSolo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_OpEffectStartSolo")` |
| `kHIDUsage_PID_OpEffectStop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_OpEffectStop")` |
| `kHIDUsage_PID_ParamBlockOffset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ParamBlockOffset")` |
| `kHIDUsage_PID_ParameterBlockSize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ParameterBlockSize")` |
| `kHIDUsage_PID_Period` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Period")` |
| `kHIDUsage_PID_Phase` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Phase")` |
| `kHIDUsage_PID_PhysicalInterfaceDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PhysicalInterfaceDevice")` |
| `kHIDUsage_PID_PoolAlignment` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PoolAlignment")` |
| `kHIDUsage_PID_PoolMoveReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PoolMoveReport")` |
| `kHIDUsage_PID_PoolReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PoolReport")` |
| `kHIDUsage_PID_PositiveCoefficient` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PositiveCoefficient")` |
| `kHIDUsage_PID_PositiveSaturation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_PositiveSaturation")` |
| `kHIDUsage_PID_RAM_PoolAvailable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_RAM_PoolAvailable")` |
| `kHIDUsage_PID_RAM_PoolSize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_RAM_PoolSize")` |
| `kHIDUsage_PID_ROM_EffectBlockCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ROM_EffectBlockCount")` |
| `kHIDUsage_PID_ROM_Flag` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ROM_Flag")` |
| `kHIDUsage_PID_ROM_PoolSize` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_ROM_PoolSize")` |
| `kHIDUsage_PID_RampEnd` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_RampEnd")` |
| `kHIDUsage_PID_RampStart` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_RampStart")` |
| `kHIDUsage_PID_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_Reserved")` |
| `kHIDUsage_PID_SafetySwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SafetySwitch")` |
| `kHIDUsage_PID_SampleCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SampleCount")` |
| `kHIDUsage_PID_SamplePeriod` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SamplePeriod")` |
| `kHIDUsage_PID_SetConditionReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetConditionReport")` |
| `kHIDUsage_PID_SetConstantForceReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetConstantForceReport")` |
| `kHIDUsage_PID_SetCustomForceReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetCustomForceReport")` |
| `kHIDUsage_PID_SetEffectReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetEffectReport")` |
| `kHIDUsage_PID_SetEnvelopeReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetEnvelopeReport")` |
| `kHIDUsage_PID_SetPeriodicReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetPeriodicReport")` |
| `kHIDUsage_PID_SetRampForceReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SetRampForceReport")` |
| `kHIDUsage_PID_SharedParameterBlocks` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SharedParameterBlocks")` |
| `kHIDUsage_PID_SimultaneousEffectsMax` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_SimultaneousEffectsMax")` |
| `kHIDUsage_PID_StartDelay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_StartDelay")` |
| `kHIDUsage_PID_StateReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_StateReport")` |
| `kHIDUsage_PID_TriggerButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_TriggerButton")` |
| `kHIDUsage_PID_TriggerRepeatInterval` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_TriggerRepeatInterval")` |
| `kHIDUsage_PID_TypeSpecificBlockHandle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_TypeSpecificBlockHandle")` |
| `kHIDUsage_PID_TypeSpecificBlockOffset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_PID_TypeSpecificBlockOffset")` |
| `kHIDUsage_Sim_Accelerator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Accelerator")` |
| `kHIDUsage_Sim_Aileron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Aileron")` |
| `kHIDUsage_Sim_AileronTrim` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_AileronTrim")` |
| `kHIDUsage_Sim_AirplaneSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_AirplaneSimulationDevice")` |
| `kHIDUsage_Sim_AntiTorqueControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_AntiTorqueControl")` |
| `kHIDUsage_Sim_AutomobileSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_AutomobileSimulationDevice")` |
| `kHIDUsage_Sim_AutopilotEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_AutopilotEnable")` |
| `kHIDUsage_Sim_Ballast` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Ballast")` |
| `kHIDUsage_Sim_BarrelElevation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_BarrelElevation")` |
| `kHIDUsage_Sim_BicycleCrank` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_BicycleCrank")` |
| `kHIDUsage_Sim_BicycleSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_BicycleSimulationDevice")` |
| `kHIDUsage_Sim_Brake` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Brake")` |
| `kHIDUsage_Sim_ChaffRelease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_ChaffRelease")` |
| `kHIDUsage_Sim_Clutch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Clutch")` |
| `kHIDUsage_Sim_CollectiveControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_CollectiveControl")` |
| `kHIDUsage_Sim_CyclicControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_CyclicControl")` |
| `kHIDUsage_Sim_CyclicTrim` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_CyclicTrim")` |
| `kHIDUsage_Sim_DiveBrake` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_DiveBrake")` |
| `kHIDUsage_Sim_DivePlane` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_DivePlane")` |
| `kHIDUsage_Sim_ElectronicCountermeasures` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_ElectronicCountermeasures")` |
| `kHIDUsage_Sim_Elevator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Elevator")` |
| `kHIDUsage_Sim_ElevatorTrim` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_ElevatorTrim")` |
| `kHIDUsage_Sim_FlareRelease` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlareRelease")` |
| `kHIDUsage_Sim_FlightCommunications` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlightCommunications")` |
| `kHIDUsage_Sim_FlightControlStick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlightControlStick")` |
| `kHIDUsage_Sim_FlightSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlightSimulationDevice")` |
| `kHIDUsage_Sim_FlightStick` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlightStick")` |
| `kHIDUsage_Sim_FlightYoke` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FlightYoke")` |
| `kHIDUsage_Sim_FrontBrake` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_FrontBrake")` |
| `kHIDUsage_Sim_HandleBars` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_HandleBars")` |
| `kHIDUsage_Sim_HelicopterSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_HelicopterSimulationDevice")` |
| `kHIDUsage_Sim_LandingGear` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_LandingGear")` |
| `kHIDUsage_Sim_MagicCarpetSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_MagicCarpetSimulationDevice")` |
| `kHIDUsage_Sim_MotorcycleSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_MotorcycleSimulationDevice")` |
| `kHIDUsage_Sim_RearBrake` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_RearBrake")` |
| `kHIDUsage_Sim_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Reserved")` |
| `kHIDUsage_Sim_Rudder` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Rudder")` |
| `kHIDUsage_Sim_SailingSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_SailingSimulationDevice")` |
| `kHIDUsage_Sim_Shifter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Shifter")` |
| `kHIDUsage_Sim_SpaceshipSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_SpaceshipSimulationDevice")` |
| `kHIDUsage_Sim_SportsSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_SportsSimulationDevice")` |
| `kHIDUsage_Sim_Steering` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Steering")` |
| `kHIDUsage_Sim_SubmarineSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_SubmarineSimulationDevice")` |
| `kHIDUsage_Sim_TankSimulationDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_TankSimulationDevice")` |
| `kHIDUsage_Sim_Throttle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Throttle")` |
| `kHIDUsage_Sim_ToeBrake` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_ToeBrake")` |
| `kHIDUsage_Sim_TrackControl` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_TrackControl")` |
| `kHIDUsage_Sim_Trigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Trigger")` |
| `kHIDUsage_Sim_TurretDirection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_TurretDirection")` |
| `kHIDUsage_Sim_Weapons` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_Weapons")` |
| `kHIDUsage_Sim_WeaponsArm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_WeaponsArm")` |
| `kHIDUsage_Sim_WingFlaps` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sim_WingFlaps")` |
| `kHIDUsage_Snsr_Biometric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Biometric")` |
| `kHIDUsage_Snsr_Biometric_HeartRate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Biometric_HeartRate")` |
| `kHIDUsage_Snsr_Biometric_HumanPresence` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Biometric_HumanPresence")` |
| `kHIDUsage_Snsr_Biometric_HumanProximity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Biometric_HumanProximity")` |
| `kHIDUsage_Snsr_Biometric_HumanTouch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Biometric_HumanTouch")` |
| `kHIDUsage_Snsr_Data_Biometric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric")` |
| `kHIDUsage_Snsr_Data_Biometric_HeartRate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric_HeartRate")` |
| `kHIDUsage_Snsr_Data_Biometric_HumanPresence` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric_HumanPresence")` |
| `kHIDUsage_Snsr_Data_Biometric_HumanProximityOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric_HumanProximityOutOfRange")` |
| `kHIDUsage_Snsr_Data_Biometric_HumanProximityRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric_HumanProximityRange")` |
| `kHIDUsage_Snsr_Data_Biometric_HumanTouchState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Biometric_HumanTouchState")` |
| `kHIDUsage_Snsr_Data_Custom` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom")` |
| `kHIDUsage_Snsr_Data_Custom_BooleanArray` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_BooleanArray")` |
| `kHIDUsage_Snsr_Data_Custom_Usage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Usage")` |
| `kHIDUsage_Snsr_Data_Custom_Value` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value")` |
| `kHIDUsage_Snsr_Data_Custom_Value1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value1")` |
| `kHIDUsage_Snsr_Data_Custom_Value2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value2")` |
| `kHIDUsage_Snsr_Data_Custom_Value3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value3")` |
| `kHIDUsage_Snsr_Data_Custom_Value4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value4")` |
| `kHIDUsage_Snsr_Data_Custom_Value5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value5")` |
| `kHIDUsage_Snsr_Data_Custom_Value6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Custom_Value6")` |
| `kHIDUsage_Snsr_Data_Electrical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical")` |
| `kHIDUsage_Snsr_Data_Electrical_Capacitance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Capacitance")` |
| `kHIDUsage_Snsr_Data_Electrical_Current` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Current")` |
| `kHIDUsage_Snsr_Data_Electrical_ElectricalPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_ElectricalPower")` |
| `kHIDUsage_Snsr_Data_Electrical_Frequency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Frequency")` |
| `kHIDUsage_Snsr_Data_Electrical_Inductance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Inductance")` |
| `kHIDUsage_Snsr_Data_Electrical_PercentOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_PercentOfRange")` |
| `kHIDUsage_Snsr_Data_Electrical_Period` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Period")` |
| `kHIDUsage_Snsr_Data_Electrical_Resistance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Resistance")` |
| `kHIDUsage_Snsr_Data_Electrical_Voltage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Electrical_Voltage")` |
| `kHIDUsage_Snsr_Data_Environmental` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental")` |
| `kHIDUsage_Snsr_Data_Environmental_AtmosphericPressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_AtmosphericPressure")` |
| `kHIDUsage_Snsr_Data_Environmental_RelativeHumidity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_RelativeHumidity")` |
| `kHIDUsage_Snsr_Data_Environmental_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_Reserved")` |
| `kHIDUsage_Snsr_Data_Environmental_Temperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_Temperature")` |
| `kHIDUsage_Snsr_Data_Environmental_WindDirection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_WindDirection")` |
| `kHIDUsage_Snsr_Data_Environmental_WindSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Environmental_WindSpeed")` |
| `kHIDUsage_Snsr_Data_Light` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light")` |
| `kHIDUsage_Snsr_Data_Light_Chromaticity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_Chromaticity")` |
| `kHIDUsage_Snsr_Data_Light_ChromaticityX` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_ChromaticityX")` |
| `kHIDUsage_Snsr_Data_Light_ChromaticityY` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_ChromaticityY")` |
| `kHIDUsage_Snsr_Data_Light_ColorTemperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_ColorTemperature")` |
| `kHIDUsage_Snsr_Data_Light_ConsumerIRSentenceReceive` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_ConsumerIRSentenceReceive")` |
| `kHIDUsage_Snsr_Data_Light_Illuminance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Light_Illuminance")` |
| `kHIDUsage_Snsr_Data_Location` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location")` |
| `kHIDUsage_Snsr_Data_Location_AddressLine1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AddressLine1")` |
| `kHIDUsage_Snsr_Data_Location_AddressLine2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AddressLine2")` |
| `kHIDUsage_Snsr_Data_Location_AltitudeAntennaSeaLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AltitudeAntennaSeaLevel")` |
| `kHIDUsage_Snsr_Data_Location_AltitudeEllipsoid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AltitudeEllipsoid")` |
| `kHIDUsage_Snsr_Data_Location_AltitudeEllipsoidError` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AltitudeEllipsoidError")` |
| `kHIDUsage_Snsr_Data_Location_AltitudeSeaLevel` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AltitudeSeaLevel")` |
| `kHIDUsage_Snsr_Data_Location_AltitudeSeaLevelError` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_AltitudeSeaLevelError")` |
| `kHIDUsage_Snsr_Data_Location_City` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_City")` |
| `kHIDUsage_Snsr_Data_Location_CountryOrRegion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_CountryOrRegion")` |
| `kHIDUsage_Snsr_Data_Location_DifferentialGPSDataAge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_DifferentialGPSDataAge")` |
| `kHIDUsage_Snsr_Data_Location_DifferentialReferenceStationID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_DifferentialReferenceStationID")` |
| `kHIDUsage_Snsr_Data_Location_ErrorRadius` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_ErrorRadius")` |
| `kHIDUsage_Snsr_Data_Location_FixQuality` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixQuality")` |
| `kHIDUsage_Snsr_Data_Location_FixQualityDGPS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixQualityDGPS")` |
| `kHIDUsage_Snsr_Data_Location_FixQualityGPS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixQualityGPS")` |
| `kHIDUsage_Snsr_Data_Location_FixQualityNoFix` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixQualityNoFix")` |
| `kHIDUsage_Snsr_Data_Location_FixType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixType")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeDGPSSPSMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeDGPSSPSMode")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeEstimated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeEstimated")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeFloatRTK` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeFloatRTK")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeGPSPPSMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeGPSPPSMode")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeGPSSPSMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeGPSSPSMode")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeManualInputMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeManualInputMode")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeNoFix` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeNoFix")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeRealTimeKinematic` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeRealTimeKinematic")` |
| `kHIDUsage_Snsr_Data_Location_FixTypeSimulatorMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_FixTypeSimulatorMode")` |
| `kHIDUsage_Snsr_Data_Location_GPSOperationMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSOperationMode")` |
| `kHIDUsage_Snsr_Data_Location_GPSOperationModeAutomatic` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSOperationModeAutomatic")` |
| `kHIDUsage_Snsr_Data_Location_GPSOperationModeManual` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSOperationModeManual")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionMode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionMode")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeAutonomous` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeAutonomous")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeDGPS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeDGPS")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeDataNotValid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeDataNotValid")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeEstimated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeEstimated")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeManualInput` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeManualInput")` |
| `kHIDUsage_Snsr_Data_Location_GPSSelectionModeSimulator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSSelectionModeSimulator")` |
| `kHIDUsage_Snsr_Data_Location_GPSStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSStatus")` |
| `kHIDUsage_Snsr_Data_Location_GPSStatusDataNotValid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSStatusDataNotValid")` |
| `kHIDUsage_Snsr_Data_Location_GPSStatusDataValid` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GPSStatusDataValid")` |
| `kHIDUsage_Snsr_Data_Location_GeoidalSeparation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_GeoidalSeparation")` |
| `kHIDUsage_Snsr_Data_Location_HorizontalDilutionOfPrecision` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_HorizontalDilutionOfPrecision")` |
| `kHIDUsage_Snsr_Data_Location_Latitude` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_Latitude")` |
| `kHIDUsage_Snsr_Data_Location_Longitude` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_Longitude")` |
| `kHIDUsage_Snsr_Data_Location_MagneticHeading` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_MagneticHeading")` |
| `kHIDUsage_Snsr_Data_Location_MagneticVariation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_MagneticVariation")` |
| `kHIDUsage_Snsr_Data_Location_NMEASentence` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_NMEASentence")` |
| `kHIDUsage_Snsr_Data_Location_PositionDilutionOfPrecision` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_PositionDilutionOfPrecision")` |
| `kHIDUsage_Snsr_Data_Location_PostalCode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_PostalCode")` |
| `kHIDUsage_Snsr_Data_Location_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_Reserved")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInView` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInView")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInViewAzimuth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInViewAzimuth")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInViewElevation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInViewElevation")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInViewIDs` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInViewIDs")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInViewPRNs` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInViewPRNs")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesInViewSNRatios` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesInViewSNRatios")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesUsedCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesUsedCount")` |
| `kHIDUsage_Snsr_Data_Location_SatellitesUsedPRNs` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_SatellitesUsedPRNs")` |
| `kHIDUsage_Snsr_Data_Location_Speed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_Speed")` |
| `kHIDUsage_Snsr_Data_Location_StateOrProvince` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_StateOrProvince")` |
| `kHIDUsage_Snsr_Data_Location_TrueHeading` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_TrueHeading")` |
| `kHIDUsage_Snsr_Data_Location_VerticalDilutionOfPrecision` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Location_VerticalDilutionOfPrecision")` |
| `kHIDUsage_Snsr_Data_Mechanical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical")` |
| `kHIDUsage_Snsr_Data_Mechanical_AbsolutePressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_AbsolutePressure")` |
| `kHIDUsage_Snsr_Data_Mechanical_BooleanSwitchArrayStates` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_BooleanSwitchArrayStates")` |
| `kHIDUsage_Snsr_Data_Mechanical_BooleanSwitchState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_BooleanSwitchState")` |
| `kHIDUsage_Snsr_Data_Mechanical_Force` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_Force")` |
| `kHIDUsage_Snsr_Data_Mechanical_GaugePressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_GaugePressure")` |
| `kHIDUsage_Snsr_Data_Mechanical_MultivalueSwitchValue` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_MultivalueSwitchValue")` |
| `kHIDUsage_Snsr_Data_Mechanical_Strain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_Strain")` |
| `kHIDUsage_Snsr_Data_Mechanical_Weight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Mechanical_Weight")` |
| `kHIDUsage_Snsr_Data_Motion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion")` |
| `kHIDUsage_Snsr_Data_Motion_Acceleration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_Acceleration")` |
| `kHIDUsage_Snsr_Data_Motion_AccelerationAxisX` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AccelerationAxisX")` |
| `kHIDUsage_Snsr_Data_Motion_AccelerationAxisY` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AccelerationAxisY")` |
| `kHIDUsage_Snsr_Data_Motion_AccelerationAxisZ` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AccelerationAxisZ")` |
| `kHIDUsage_Snsr_Data_Motion_AngularPosition` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularPosition")` |
| `kHIDUsage_Snsr_Data_Motion_AngularPositionXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularPositionXAxis")` |
| `kHIDUsage_Snsr_Data_Motion_AngularPositionYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularPositionYAxis")` |
| `kHIDUsage_Snsr_Data_Motion_AngularPositionZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularPositionZAxis")` |
| `kHIDUsage_Snsr_Data_Motion_AngularVelocity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularVelocity")` |
| `kHIDUsage_Snsr_Data_Motion_AngularVelocityXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularVelocityXAxis")` |
| `kHIDUsage_Snsr_Data_Motion_AngularVelocityYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularVelocityYAxis")` |
| `kHIDUsage_Snsr_Data_Motion_AngularVelocityZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_AngularVelocityZAxis")` |
| `kHIDUsage_Snsr_Data_Motion_Intensity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_Intensity")` |
| `kHIDUsage_Snsr_Data_Motion_Speed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_Speed")` |
| `kHIDUsage_Snsr_Data_Motion_State` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Motion_State")` |
| `kHIDUsage_Snsr_Data_Orientation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation")` |
| `kHIDUsage_Snsr_Data_Orientation_Distance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_Distance")` |
| `kHIDUsage_Snsr_Data_Orientation_DistanceOutOfRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_DistanceOutOfRange")` |
| `kHIDUsage_Snsr_Data_Orientation_DistanceXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_DistanceXAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_DistanceYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_DistanceYAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_DistanceZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_DistanceZAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_Heading` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_Heading")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingCompensatedMagneticNorth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingCompensatedMagneticNorth")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingCompensatedTrueNorth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingCompensatedTrueNorth")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingMagneticNorth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingMagneticNorth")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingTrueNorth` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingTrueNorth")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingXAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingYAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_HeadingZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_HeadingZAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_MagneticFlux` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_MagneticFlux")` |
| `kHIDUsage_Snsr_Data_Orientation_MagneticFluxXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_MagneticFluxXAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_MagneticFluxYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_MagneticFluxYAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_MagneticFluxZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_MagneticFluxZAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_Quaternion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_Quaternion")` |
| `kHIDUsage_Snsr_Data_Orientation_RotationMatrix` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_RotationMatrix")` |
| `kHIDUsage_Snsr_Data_Orientation_Tilt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_Tilt")` |
| `kHIDUsage_Snsr_Data_Orientation_TiltXAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_TiltXAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_TiltYAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_TiltYAxis")` |
| `kHIDUsage_Snsr_Data_Orientation_TiltZAxis` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Orientation_TiltZAxis")` |
| `kHIDUsage_Snsr_Data_Scanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Scanner")` |
| `kHIDUsage_Snsr_Data_Scanner_NFCSentenceReceive` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Scanner_NFCSentenceReceive")` |
| `kHIDUsage_Snsr_Data_Scanner_RFIDTag40Bit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Scanner_RFIDTag40Bit")` |
| `kHIDUsage_Snsr_Data_Time` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time")` |
| `kHIDUsage_Snsr_Data_Time_Day` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Day")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeek` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeek")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekFriday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekFriday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekMonday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekMonday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekSaturday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekSaturday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekSunday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekSunday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekThursday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekThursday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekTuesday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekTuesday")` |
| `kHIDUsage_Snsr_Data_Time_DayOfWeekWednesday` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_DayOfWeekWednesday")` |
| `kHIDUsage_Snsr_Data_Time_Hour` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Hour")` |
| `kHIDUsage_Snsr_Data_Time_JulianDayOfYear` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_JulianDayOfYear")` |
| `kHIDUsage_Snsr_Data_Time_Millisecond` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Millisecond")` |
| `kHIDUsage_Snsr_Data_Time_Minute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Minute")` |
| `kHIDUsage_Snsr_Data_Time_Month` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Month")` |
| `kHIDUsage_Snsr_Data_Time_Second` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Second")` |
| `kHIDUsage_Snsr_Data_Time_Timestamp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Timestamp")` |
| `kHIDUsage_Snsr_Data_Time_Year` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Data_Time_Year")` |
| `kHIDUsage_Snsr_Electrical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical")` |
| `kHIDUsage_Snsr_Electrical_Capacitance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Capacitance")` |
| `kHIDUsage_Snsr_Electrical_Current` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Current")` |
| `kHIDUsage_Snsr_Electrical_Frequency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Frequency")` |
| `kHIDUsage_Snsr_Electrical_Inductance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Inductance")` |
| `kHIDUsage_Snsr_Electrical_Period` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Period")` |
| `kHIDUsage_Snsr_Electrical_Potentiometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Potentiometer")` |
| `kHIDUsage_Snsr_Electrical_Power` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Power")` |
| `kHIDUsage_Snsr_Electrical_Resistance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Resistance")` |
| `kHIDUsage_Snsr_Electrical_Voltage` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Electrical_Voltage")` |
| `kHIDUsage_Snsr_Environmental` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental")` |
| `kHIDUsage_Snsr_Environmental_AtmosphericPressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental_AtmosphericPressure")` |
| `kHIDUsage_Snsr_Environmental_Humidity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental_Humidity")` |
| `kHIDUsage_Snsr_Environmental_Temperature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental_Temperature")` |
| `kHIDUsage_Snsr_Environmental_WindDirection` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental_WindDirection")` |
| `kHIDUsage_Snsr_Environmental_WindSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Environmental_WindSpeed")` |
| `kHIDUsage_Snsr_Event` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event")` |
| `kHIDUsage_Snsr_Event_SensorEvent` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent")` |
| `kHIDUsage_Snsr_Event_SensorEvent_ChangeSensitivity` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_ChangeSensitivity")` |
| `kHIDUsage_Snsr_Event_SensorEvent_ComplexTrigger` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_ComplexTrigger")` |
| `kHIDUsage_Snsr_Event_SensorEvent_DataUpdated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_DataUpdated")` |
| `kHIDUsage_Snsr_Event_SensorEvent_FrequencyExceeded` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_FrequencyExceeded")` |
| `kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossDown")` |
| `kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_HighThresholdCrossUp")` |
| `kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossDown")` |
| `kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_LowThresholdCrossUp")` |
| `kHIDUsage_Snsr_Event_SensorEvent_PeriodExceeded` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_PeriodExceeded")` |
| `kHIDUsage_Snsr_Event_SensorEvent_PollResponse` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_PollResponse")` |
| `kHIDUsage_Snsr_Event_SensorEvent_PropertyChanged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_PropertyChanged")` |
| `kHIDUsage_Snsr_Event_SensorEvent_RangeMaxReached` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_RangeMaxReached")` |
| `kHIDUsage_Snsr_Event_SensorEvent_RangeMinReached` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_RangeMinReached")` |
| `kHIDUsage_Snsr_Event_SensorEvent_StateChanged` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_StateChanged")` |
| `kHIDUsage_Snsr_Event_SensorEvent_Unknown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_Unknown")` |
| `kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossDown` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossDown")` |
| `kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossUp` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorEvent_ZeroThresholdCrossUp")` |
| `kHIDUsage_Snsr_Event_SensorState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState")` |
| `kHIDUsage_Snsr_Event_SensorState_AccessDenied` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_AccessDenied")` |
| `kHIDUsage_Snsr_Event_SensorState_Error` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_Error")` |
| `kHIDUsage_Snsr_Event_SensorState_Initializing` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_Initializing")` |
| `kHIDUsage_Snsr_Event_SensorState_NoData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_NoData")` |
| `kHIDUsage_Snsr_Event_SensorState_NotAvailable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_NotAvailable")` |
| `kHIDUsage_Snsr_Event_SensorState_Ready` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_Ready")` |
| `kHIDUsage_Snsr_Event_SensorState_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Event_SensorState_Undefined")` |
| `kHIDUsage_Snsr_Light` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Light")` |
| `kHIDUsage_Snsr_Light_AmbientLight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Light_AmbientLight")` |
| `kHIDUsage_Snsr_Light_ConsumerInfrared` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Light_ConsumerInfrared")` |
| `kHIDUsage_Snsr_Light_Illuminance` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Light_Illuminance")` |
| `kHIDUsage_Snsr_Location` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location")` |
| `kHIDUsage_Snsr_Location_Broadcast` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_Broadcast")` |
| `kHIDUsage_Snsr_Location_DeadReckoning` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_DeadReckoning")` |
| `kHIDUsage_Snsr_Location_GPS` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_GPS")` |
| `kHIDUsage_Snsr_Location_Lookup` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_Lookup")` |
| `kHIDUsage_Snsr_Location_Other` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_Other")` |
| `kHIDUsage_Snsr_Location_Static` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_Static")` |
| `kHIDUsage_Snsr_Location_Triangulation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Location_Triangulation")` |
| `kHIDUsage_Snsr_Mechanical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical")` |
| `kHIDUsage_Snsr_Mechanical_BooleanSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_BooleanSwitch")` |
| `kHIDUsage_Snsr_Mechanical_BooleanSwitchArray` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_BooleanSwitchArray")` |
| `kHIDUsage_Snsr_Mechanical_Force` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_Force")` |
| `kHIDUsage_Snsr_Mechanical_HallEffectSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_HallEffectSwitch")` |
| `kHIDUsage_Snsr_Mechanical_HapticVibrator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_HapticVibrator")` |
| `kHIDUsage_Snsr_Mechanical_MultivalueSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_MultivalueSwitch")` |
| `kHIDUsage_Snsr_Mechanical_Pressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_Pressure")` |
| `kHIDUsage_Snsr_Mechanical_Strain` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_Strain")` |
| `kHIDUsage_Snsr_Mechanical_Weight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Mechanical_Weight")` |
| `kHIDUsage_Snsr_Modifier_Accuracy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_Accuracy")` |
| `kHIDUsage_Snsr_Modifier_CalibrationMultiplier` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_CalibrationMultiplier")` |
| `kHIDUsage_Snsr_Modifier_CalibrationOffset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_CalibrationOffset")` |
| `kHIDUsage_Snsr_Modifier_ChangeSensitivityAbsolute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ChangeSensitivityAbsolute")` |
| `kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRange")` |
| `kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRelative` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ChangeSensitivityPercentRelative")` |
| `kHIDUsage_Snsr_Modifier_FrequencyMax` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_FrequencyMax")` |
| `kHIDUsage_Snsr_Modifier_Max` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_Max")` |
| `kHIDUsage_Snsr_Modifier_Min` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_Min")` |
| `kHIDUsage_Snsr_Modifier_None` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_None")` |
| `kHIDUsage_Snsr_Modifier_PeriodMax` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_PeriodMax")` |
| `kHIDUsage_Snsr_Modifier_ReportInterval` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ReportInterval")` |
| `kHIDUsage_Snsr_Modifier_Resolution` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_Resolution")` |
| `kHIDUsage_Snsr_Modifier_ThresholdHigh` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ThresholdHigh")` |
| `kHIDUsage_Snsr_Modifier_ThresholdLow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_ThresholdLow")` |
| `kHIDUsage_Snsr_Modifier_VendorDefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Modifier_VendorDefined")` |
| `kHIDUsage_Snsr_Motion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion")` |
| `kHIDUsage_Snsr_Motion_Accelerometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Accelerometer")` |
| `kHIDUsage_Snsr_Motion_Accelerometer1D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Accelerometer1D")` |
| `kHIDUsage_Snsr_Motion_Accelerometer2D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Accelerometer2D")` |
| `kHIDUsage_Snsr_Motion_Accelerometer3D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Accelerometer3D")` |
| `kHIDUsage_Snsr_Motion_GravityVector` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_GravityVector")` |
| `kHIDUsage_Snsr_Motion_Gyrometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Gyrometer")` |
| `kHIDUsage_Snsr_Motion_Gyrometer1D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Gyrometer1D")` |
| `kHIDUsage_Snsr_Motion_Gyrometer2D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Gyrometer2D")` |
| `kHIDUsage_Snsr_Motion_Gyrometer3D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Gyrometer3D")` |
| `kHIDUsage_Snsr_Motion_LinearAccelerometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_LinearAccelerometer")` |
| `kHIDUsage_Snsr_Motion_MotionDetector` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_MotionDetector")` |
| `kHIDUsage_Snsr_Motion_Speedometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Motion_Speedometer")` |
| `kHIDUsage_Snsr_Orientation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation")` |
| `kHIDUsage_Snsr_Orientation_Compass1D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Compass1D")` |
| `kHIDUsage_Snsr_Orientation_Compass2D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Compass2D")` |
| `kHIDUsage_Snsr_Orientation_Compass3D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Compass3D")` |
| `kHIDUsage_Snsr_Orientation_CompassD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_CompassD")` |
| `kHIDUsage_Snsr_Orientation_DeviceOrientation` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_DeviceOrientation")` |
| `kHIDUsage_Snsr_Orientation_Distance1D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Distance1D")` |
| `kHIDUsage_Snsr_Orientation_Distance2D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Distance2D")` |
| `kHIDUsage_Snsr_Orientation_Distance3D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Distance3D")` |
| `kHIDUsage_Snsr_Orientation_DistanceD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_DistanceD")` |
| `kHIDUsage_Snsr_Orientation_Inclinometer1D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Inclinometer1D")` |
| `kHIDUsage_Snsr_Orientation_Inclinometer2D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Inclinometer2D")` |
| `kHIDUsage_Snsr_Orientation_Inclinometer3D` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_Inclinometer3D")` |
| `kHIDUsage_Snsr_Orientation_InclinometerD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Orientation_InclinometerD")` |
| `kHIDUsage_Snsr_Other` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Other")` |
| `kHIDUsage_Snsr_Other_Custom` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Other_Custom")` |
| `kHIDUsage_Snsr_Other_Generic` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Other_Generic")` |
| `kHIDUsage_Snsr_Other_GenericEnumerator` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Other_GenericEnumerator")` |
| `kHIDUsage_Snsr_Property` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property")` |
| `kHIDUsage_Snsr_Property_Accuracy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Accuracy")` |
| `kHIDUsage_Snsr_Property_ChangeSensitivityAbsolute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ChangeSensitivityAbsolute")` |
| `kHIDUsage_Snsr_Property_ChangeSensitivityPercentRange` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ChangeSensitivityPercentRange")` |
| `kHIDUsage_Snsr_Property_ChangeSensitivityPercentRelative` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ChangeSensitivityPercentRelative")` |
| `kHIDUsage_Snsr_Property_ConnectionType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ConnectionType")` |
| `kHIDUsage_Snsr_Property_ConnectionType_Attached` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ConnectionType_Attached")` |
| `kHIDUsage_Snsr_Property_ConnectionType_External` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ConnectionType_External")` |
| `kHIDUsage_Snsr_Property_ConnectionType_Integrated` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ConnectionType_Integrated")` |
| `kHIDUsage_Snsr_Property_Description` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Description")` |
| `kHIDUsage_Snsr_Property_DevicePath` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_DevicePath")` |
| `kHIDUsage_Snsr_Property_Environmental` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Environmental")` |
| `kHIDUsage_Snsr_Property_Environmental_ReferencePressure` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Environmental_ReferencePressure")` |
| `kHIDUsage_Snsr_Property_FirmwareVersion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_FirmwareVersion")` |
| `kHIDUsage_Snsr_Property_FriendlyName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_FriendlyName")` |
| `kHIDUsage_Snsr_Property_HardwareRevision` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_HardwareRevision")` |
| `kHIDUsage_Snsr_Property_Light` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Light")` |
| `kHIDUsage_Snsr_Property_Light_ConsumerIRSentenceSend` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Light_ConsumerIRSentenceSend")` |
| `kHIDUsage_Snsr_Property_Location` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location")` |
| `kHIDUsage_Snsr_Property_Location_AccuracyDefault` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location_AccuracyDefault")` |
| `kHIDUsage_Snsr_Property_Location_AccuracyHigh` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location_AccuracyHigh")` |
| `kHIDUsage_Snsr_Property_Location_AccuracyLow` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location_AccuracyLow")` |
| `kHIDUsage_Snsr_Property_Location_AccuracyMedium` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location_AccuracyMedium")` |
| `kHIDUsage_Snsr_Property_Location_DesiredAccuracy` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Location_DesiredAccuracy")` |
| `kHIDUsage_Snsr_Property_Manufacturer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Manufacturer")` |
| `kHIDUsage_Snsr_Property_MaxFIFOEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_MaxFIFOEvents")` |
| `kHIDUsage_Snsr_Property_Maximum` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Maximum")` |
| `kHIDUsage_Snsr_Property_Mechanical` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Mechanical")` |
| `kHIDUsage_Snsr_Property_Mechanical_BackwardVibrationSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Mechanical_BackwardVibrationSpeed")` |
| `kHIDUsage_Snsr_Property_Mechanical_ForwardVibrationSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Mechanical_ForwardVibrationSpeed")` |
| `kHIDUsage_Snsr_Property_Mechanical_VibrationState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Mechanical_VibrationState")` |
| `kHIDUsage_Snsr_Property_Minimum` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Minimum")` |
| `kHIDUsage_Snsr_Property_MinimumReportInterval` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_MinimumReportInterval")` |
| `kHIDUsage_Snsr_Property_Model` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Model")` |
| `kHIDUsage_Snsr_Property_PersistentUniqueID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PersistentUniqueID")` |
| `kHIDUsage_Snsr_Property_PowerState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState")` |
| `kHIDUsage_Snsr_Property_PowerState_D0_FullPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_D0_FullPower")` |
| `kHIDUsage_Snsr_Property_PowerState_D1_LowPower` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_D1_LowPower")` |
| `kHIDUsage_Snsr_Property_PowerState_D2_Standby` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_D2_Standby")` |
| `kHIDUsage_Snsr_Property_PowerState_D3_Sleep` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_D3_Sleep")` |
| `kHIDUsage_Snsr_Property_PowerState_D4_PowerOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_D4_PowerOff")` |
| `kHIDUsage_Snsr_Property_PowerState_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_PowerState_Undefined")` |
| `kHIDUsage_Snsr_Property_ReleaseData` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReleaseData")` |
| `kHIDUsage_Snsr_Property_ReportInterval` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportInterval")` |
| `kHIDUsage_Snsr_Property_ReportLatency` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportLatency")` |
| `kHIDUsage_Snsr_Property_ReportingState` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState")` |
| `kHIDUsage_Snsr_Property_ReportingState_AllEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_AllEvents")` |
| `kHIDUsage_Snsr_Property_ReportingState_NoEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_NoEvents")` |
| `kHIDUsage_Snsr_Property_ReportingState_ThresholdEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_ThresholdEvents")` |
| `kHIDUsage_Snsr_Property_ReportingState_WakeAllEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_WakeAllEvents")` |
| `kHIDUsage_Snsr_Property_ReportingState_WakeNoEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_WakeNoEvents")` |
| `kHIDUsage_Snsr_Property_ReportingState_WakeThresholdEvents` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ReportingState_WakeThresholdEvents")` |
| `kHIDUsage_Snsr_Property_Resolution` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Resolution")` |
| `kHIDUsage_Snsr_Property_ResponseCurve` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_ResponseCurve")` |
| `kHIDUsage_Snsr_Property_SamplingRate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_SamplingRate")` |
| `kHIDUsage_Snsr_Property_Scanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Scanner")` |
| `kHIDUsage_Snsr_Property_Scanner_NFCSentenceSend` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Scanner_NFCSentenceSend")` |
| `kHIDUsage_Snsr_Property_SensorStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_SensorStatus")` |
| `kHIDUsage_Snsr_Property_SerialNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_SerialNumber")` |
| `kHIDUsage_Snsr_Property_Time` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time")` |
| `kHIDUsage_Snsr_Property_Time_ArmAlarm` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time_ArmAlarm")` |
| `kHIDUsage_Snsr_Property_Time_DaylightSavingsTimeObserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time_DaylightSavingsTimeObserved")` |
| `kHIDUsage_Snsr_Property_Time_TimeTrimAdjustment` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time_TimeTrimAdjustment")` |
| `kHIDUsage_Snsr_Property_Time_TimeZoneName` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time_TimeZoneName")` |
| `kHIDUsage_Snsr_Property_Time_TimeZoneOffsetFromUTC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Property_Time_TimeZoneOffsetFromUTC")` |
| `kHIDUsage_Snsr_Scanner` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Scanner")` |
| `kHIDUsage_Snsr_Scanner_Barcode` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Scanner_Barcode")` |
| `kHIDUsage_Snsr_Scanner_NFC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Scanner_NFC")` |
| `kHIDUsage_Snsr_Scanner_RFID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Scanner_RFID")` |
| `kHIDUsage_Snsr_Sensor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Sensor")` |
| `kHIDUsage_Snsr_Time` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Time")` |
| `kHIDUsage_Snsr_Time_AlarmTimer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Time_AlarmTimer")` |
| `kHIDUsage_Snsr_Time_RealTimeClock` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Time_RealTimeClock")` |
| `kHIDUsage_Snsr_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Snsr_Undefined")` |
| `kHIDUsage_Sprt_10Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_10Iron")` |
| `kHIDUsage_Sprt_11Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_11Iron")` |
| `kHIDUsage_Sprt_1Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_1Iron")` |
| `kHIDUsage_Sprt_1Wood` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_1Wood")` |
| `kHIDUsage_Sprt_2Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_2Iron")` |
| `kHIDUsage_Sprt_3Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_3Iron")` |
| `kHIDUsage_Sprt_3Wood` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_3Wood")` |
| `kHIDUsage_Sprt_4Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_4Iron")` |
| `kHIDUsage_Sprt_5Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_5Iron")` |
| `kHIDUsage_Sprt_5Wood` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_5Wood")` |
| `kHIDUsage_Sprt_6Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_6Iron")` |
| `kHIDUsage_Sprt_7Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_7Iron")` |
| `kHIDUsage_Sprt_7Wood` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_7Wood")` |
| `kHIDUsage_Sprt_8Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_8Iron")` |
| `kHIDUsage_Sprt_9Iron` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_9Iron")` |
| `kHIDUsage_Sprt_9Wood` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_9Wood")` |
| `kHIDUsage_Sprt_BaseballBat` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_BaseballBat")` |
| `kHIDUsage_Sprt_GolfClub` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_GolfClub")` |
| `kHIDUsage_Sprt_LoftWedge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_LoftWedge")` |
| `kHIDUsage_Sprt_Oar` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Oar")` |
| `kHIDUsage_Sprt_PowerWedge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_PowerWedge")` |
| `kHIDUsage_Sprt_Putter` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Putter")` |
| `kHIDUsage_Sprt_Rate` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Rate")` |
| `kHIDUsage_Sprt_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Reserved")` |
| `kHIDUsage_Sprt_RowingMachine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_RowingMachine")` |
| `kHIDUsage_Sprt_SandWedge` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_SandWedge")` |
| `kHIDUsage_Sprt_Slope` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Slope")` |
| `kHIDUsage_Sprt_StickFaceAngle` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickFaceAngle")` |
| `kHIDUsage_Sprt_StickFollowThrough` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickFollowThrough")` |
| `kHIDUsage_Sprt_StickHeelOrToe` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickHeelOrToe")` |
| `kHIDUsage_Sprt_StickHeight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickHeight")` |
| `kHIDUsage_Sprt_StickSpeed` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickSpeed")` |
| `kHIDUsage_Sprt_StickTempo` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickTempo")` |
| `kHIDUsage_Sprt_StickType` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_StickType")` |
| `kHIDUsage_Sprt_Treadmill` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Sprt_Treadmill")` |
| `kHIDUsage_TFon_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_TFon_Reserved")` |
| `kHIDUsage_Tfon_AlternateFunction` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_AlternateFunction")` |
| `kHIDUsage_Tfon_AnswerOnOrOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_AnswerOnOrOff")` |
| `kHIDUsage_Tfon_AnsweringMachine` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_AnsweringMachine")` |
| `kHIDUsage_Tfon_CallWaitingTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_CallWaitingTone")` |
| `kHIDUsage_Tfon_CallerID` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_CallerID")` |
| `kHIDUsage_Tfon_Conference` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Conference")` |
| `kHIDUsage_Tfon_ConfirmationTone1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ConfirmationTone1")` |
| `kHIDUsage_Tfon_ConfirmationTone2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ConfirmationTone2")` |
| `kHIDUsage_Tfon_DoNotDisturb` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_DoNotDisturb")` |
| `kHIDUsage_Tfon_Drop` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Drop")` |
| `kHIDUsage_Tfon_Feature` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Feature")` |
| `kHIDUsage_Tfon_Flash` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Flash")` |
| `kHIDUsage_Tfon_ForwardCalls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ForwardCalls")` |
| `kHIDUsage_Tfon_Handset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Handset")` |
| `kHIDUsage_Tfon_Headset` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Headset")` |
| `kHIDUsage_Tfon_Hold` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Hold")` |
| `kHIDUsage_Tfon_HookSwitch` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_HookSwitch")` |
| `kHIDUsage_Tfon_InsideDialTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_InsideDialTone")` |
| `kHIDUsage_Tfon_InsideRingTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_InsideRingTone")` |
| `kHIDUsage_Tfon_InsideRingback` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_InsideRingback")` |
| `kHIDUsage_Tfon_Line` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Line")` |
| `kHIDUsage_Tfon_LineBusyTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_LineBusyTone")` |
| `kHIDUsage_Tfon_Message` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Message")` |
| `kHIDUsage_Tfon_MessageControls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_MessageControls")` |
| `kHIDUsage_Tfon_OutsideDialTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_OutsideDialTone")` |
| `kHIDUsage_Tfon_OutsideRingTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_OutsideRingTone")` |
| `kHIDUsage_Tfon_OutsideRingback` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_OutsideRingback")` |
| `kHIDUsage_Tfon_Park` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Park")` |
| `kHIDUsage_Tfon_Phone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Phone")` |
| `kHIDUsage_Tfon_PhoneDirectory` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneDirectory")` |
| `kHIDUsage_Tfon_PhoneKey0` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey0")` |
| `kHIDUsage_Tfon_PhoneKey1` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey1")` |
| `kHIDUsage_Tfon_PhoneKey2` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey2")` |
| `kHIDUsage_Tfon_PhoneKey3` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey3")` |
| `kHIDUsage_Tfon_PhoneKey4` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey4")` |
| `kHIDUsage_Tfon_PhoneKey5` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey5")` |
| `kHIDUsage_Tfon_PhoneKey6` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey6")` |
| `kHIDUsage_Tfon_PhoneKey7` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey7")` |
| `kHIDUsage_Tfon_PhoneKey8` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey8")` |
| `kHIDUsage_Tfon_PhoneKey9` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKey9")` |
| `kHIDUsage_Tfon_PhoneKeyA` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyA")` |
| `kHIDUsage_Tfon_PhoneKeyB` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyB")` |
| `kHIDUsage_Tfon_PhoneKeyC` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyC")` |
| `kHIDUsage_Tfon_PhoneKeyD` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyD")` |
| `kHIDUsage_Tfon_PhoneKeyPound` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyPound")` |
| `kHIDUsage_Tfon_PhoneKeyStar` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneKeyStar")` |
| `kHIDUsage_Tfon_PhoneMute` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PhoneMute")` |
| `kHIDUsage_Tfon_PriorityRingTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PriorityRingTone")` |
| `kHIDUsage_Tfon_PriorityRingback` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_PriorityRingback")` |
| `kHIDUsage_Tfon_ProgrammableButton` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ProgrammableButton")` |
| `kHIDUsage_Tfon_RecallNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_RecallNumber")` |
| `kHIDUsage_Tfon_Redial` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Redial")` |
| `kHIDUsage_Tfon_ReorderTone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ReorderTone")` |
| `kHIDUsage_Tfon_Ring` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Ring")` |
| `kHIDUsage_Tfon_RingEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_RingEnable")` |
| `kHIDUsage_Tfon_ScreenCalls` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_ScreenCalls")` |
| `kHIDUsage_Tfon_SpeakerPhone` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_SpeakerPhone")` |
| `kHIDUsage_Tfon_SpeedDial` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_SpeedDial")` |
| `kHIDUsage_Tfon_StoreNumber` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_StoreNumber")` |
| `kHIDUsage_Tfon_TelephonyKeyPad` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_TelephonyKeyPad")` |
| `kHIDUsage_Tfon_TonesOff` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_TonesOff")` |
| `kHIDUsage_Tfon_Transfer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_Transfer")` |
| `kHIDUsage_Tfon_VoiceMail` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Tfon_VoiceMail")` |
| `kHIDUsage_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_Undefined")` |
| `kHIDUsage_VR_AnimatronicDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_AnimatronicDevice")` |
| `kHIDUsage_VR_Belt` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Belt")` |
| `kHIDUsage_VR_BodySuit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_BodySuit")` |
| `kHIDUsage_VR_DisplayEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_DisplayEnable")` |
| `kHIDUsage_VR_Flexor` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Flexor")` |
| `kHIDUsage_VR_Glove` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Glove")` |
| `kHIDUsage_VR_HandTracker` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_HandTracker")` |
| `kHIDUsage_VR_HeadMountedDisplay` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_HeadMountedDisplay")` |
| `kHIDUsage_VR_HeadTracker` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_HeadTracker")` |
| `kHIDUsage_VR_Oculometer` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Oculometer")` |
| `kHIDUsage_VR_Reserved` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Reserved")` |
| `kHIDUsage_VR_StereoEnable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_StereoEnable")` |
| `kHIDUsage_VR_Vest` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_VR_Vest")` |
| `kHIDUsage_WD_CalibrationCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_CalibrationCount")` |
| `kHIDUsage_WD_DataScaling` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_DataScaling")` |
| `kHIDUsage_WD_DataWeight` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_DataWeight")` |
| `kHIDUsage_WD_EnforcedZeroReturn` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_EnforcedZeroReturn")` |
| `kHIDUsage_WD_RezeroCount` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_RezeroCount")` |
| `kHIDUsage_WD_ScaleAtrributeReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleAtrributeReport")` |
| `kHIDUsage_WD_ScaleControlReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleControlReport")` |
| `kHIDUsage_WD_ScaleDataReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleDataReport")` |
| `kHIDUsage_WD_ScaleScaleClassGeneric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassGeneric")` |
| `kHIDUsage_WD_ScaleScaleClassIIIEnglish` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIIIEnglish")` |
| `kHIDUsage_WD_ScaleScaleClassIIILEnglish` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIIILEnglish")` |
| `kHIDUsage_WD_ScaleScaleClassIIILMetric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIIILMetric")` |
| `kHIDUsage_WD_ScaleScaleClassIIIMetric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIIIMetric")` |
| `kHIDUsage_WD_ScaleScaleClassIIMetric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIIMetric")` |
| `kHIDUsage_WD_ScaleScaleClassIMetric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIMetric")` |
| `kHIDUsage_WD_ScaleScaleClassIMetricCL` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIMetricCL")` |
| `kHIDUsage_WD_ScaleScaleClassIVEnglish` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIVEnglish")` |
| `kHIDUsage_WD_ScaleScaleClassIVMetric` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleClassIVMetric")` |
| `kHIDUsage_WD_ScaleScaleDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleScaleDevice")` |
| `kHIDUsage_WD_ScaleStatisticsReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatisticsReport")` |
| `kHIDUsage_WD_ScaleStatus` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatus")` |
| `kHIDUsage_WD_ScaleStatusFault` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusFault")` |
| `kHIDUsage_WD_ScaleStatusInMotion` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusInMotion")` |
| `kHIDUsage_WD_ScaleStatusOverWeightLimit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusOverWeightLimit")` |
| `kHIDUsage_WD_ScaleStatusReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusReport")` |
| `kHIDUsage_WD_ScaleStatusRequiresCalibration` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusRequiresCalibration")` |
| `kHIDUsage_WD_ScaleStatusRequiresRezeroing` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusRequiresRezeroing")` |
| `kHIDUsage_WD_ScaleStatusStableAtZero` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusStableAtZero")` |
| `kHIDUsage_WD_ScaleStatusUnderZero` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusUnderZero")` |
| `kHIDUsage_WD_ScaleStatusWeightStable` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleStatusWeightStable")` |
| `kHIDUsage_WD_ScaleWeightLimitReport` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ScaleWeightLimitReport")` |
| `kHIDUsage_WD_Undefined` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_Undefined")` |
| `kHIDUsage_WD_WeighingDevice` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeighingDevice")` |
| `kHIDUsage_WD_WeightUnit` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnit")` |
| `kHIDUsage_WD_WeightUnitAvoirTon` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitAvoirTon")` |
| `kHIDUsage_WD_WeightUnitCarats` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitCarats")` |
| `kHIDUsage_WD_WeightUnitGrains` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitGrains")` |
| `kHIDUsage_WD_WeightUnitGram` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitGram")` |
| `kHIDUsage_WD_WeightUnitKilogram` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitKilogram")` |
| `kHIDUsage_WD_WeightUnitMetricTon` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitMetricTon")` |
| `kHIDUsage_WD_WeightUnitMilligram` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitMilligram")` |
| `kHIDUsage_WD_WeightUnitOunce` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitOunce")` |
| `kHIDUsage_WD_WeightUnitPennyweights` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitPennyweights")` |
| `kHIDUsage_WD_WeightUnitPound` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitPound")` |
| `kHIDUsage_WD_WeightUnitTaels` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitTaels")` |
| `kHIDUsage_WD_WeightUnitTroyOunce` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_WeightUnitTroyOunce")` |
| `kHIDUsage_WD_ZeroScale` | enum constant | `IOHIDUsageTables.h` | `usage::ALL_USAGE_CONSTANTS` / `usage::constant("kHIDUsage_WD_ZeroScale")` |
| `IOHIDValueCreateWithBytes` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueCreateWithBytes` |
| `IOHIDValueCreateWithBytesNoCopy` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueCreateWithBytesNoCopy` |
| `IOHIDValueCreateWithIntegerValue` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueCreateWithIntegerValue` |
| `IOHIDValueGetBytePtr` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetBytePtr` |
| `IOHIDValueGetElement` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetElement` |
| `IOHIDValueGetIntegerValue` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetIntegerValue` |
| `IOHIDValueGetLength` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetLength` |
| `IOHIDValueGetScaledValue` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetScaledValue` |
| `IOHIDValueGetTimeStamp` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetTimeStamp` |
| `IOHIDValueGetTypeID` | function | `IOHIDValue.h` | `HidValue` / `ffi::IOHIDValueGetTypeID` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `IOHIDDeviceGetValueOptions` | enum | `IOHIDDevice.h` | The get-value option constants are present, but the enum type itself is not surfaced as a public Rust alias/newtype. |
| `IOHIDDeviceDeviceInterface` | struct | `IOHIDDevicePlugIn.h` | UUID discovery is exposed via `service_plugin::ServicePlugInUuid`, but the CFPlugIn vtable struct itself is not wrapped. |
| `IOHIDDeviceQueueInterface` | struct | `IOHIDDevicePlugIn.h` | UUID discovery is exposed via `service_plugin::ServicePlugInUuid`, but the CFPlugIn vtable struct itself is not wrapped. |
| `IOHIDDeviceTimeStampedDeviceInterface` | struct | `IOHIDDevicePlugIn.h` | UUID discovery is exposed via `service_plugin::ServicePlugInUuid`, but the CFPlugIn vtable struct itself is not wrapped. |
| `IOHIDDeviceTransactionInterface` | struct | `IOHIDDevicePlugIn.h` | UUID discovery is exposed via `service_plugin::ServicePlugInUuid`, but the CFPlugIn vtable struct itself is not wrapped. |
| `HIDReportCommandType` | enum | `IOHIDDeviceTypes.h` | Low-level report command/option enums from `IOHIDDeviceTypes.h` are not exposed. |
| `IOHIDCompletion` | struct | `IOHIDDeviceTypes.h` | Completion callback structs/types from `IOHIDDeviceTypes.h` are not exposed. |
| `IOHIDCompletionAction` | callback typedef | `IOHIDDeviceTypes.h` | Completion callback structs/types from `IOHIDDeviceTypes.h` are not exposed. |
| `IOHIDElementCommitDirection` | enum | `IOHIDDeviceTypes.h` | The element-commit direction enum used by lower-level report commit APIs is not wrapped. |
| `IOHIDElementCookie` | typedef | `IOHIDDeviceTypes.h` | `HidElement::cookie()` returns a `u32`, but there is no public `IOHIDElementCookie` alias. |
| `IOHIDElementFlags` | typedef | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `IOHIDValueOptions` | typedef | `IOHIDDeviceTypes.h` | Value-option bitmask definitions are not exposed as public Rust constants/types. |
| `kIOHIDDeviceDefaultAsyncRequestTimeout` | macro | `IOHIDDeviceTypes.h` | Low-level async timeout constants from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDDeviceMaxAsyncRequestTimeout` | macro | `IOHIDDeviceTypes.h` | Low-level async timeout constants from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDDeviceMinAsyncRequestTimeout` | macro | `IOHIDDeviceTypes.h` | Low-level async timeout constants from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDElementCommitDirectionIn` | enum constant | `IOHIDDeviceTypes.h` | The element-commit direction enum used by lower-level report commit APIs is not wrapped. |
| `kIOHIDElementCommitDirectionOut` | enum constant | `IOHIDDeviceTypes.h` | The element-commit direction enum used by lower-level report commit APIs is not wrapped. |
| `kIOHIDElementFlagsBufferedByteMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsConstantMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsNoPreferredMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsNonLinearMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsNullStateMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsRelativeMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsVariableMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsVolativeMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDElementFlagsWrapMask` | enum constant | `IOHIDDeviceTypes.h` | Safe APIs expose individual boolean queries, but the raw element-flags bitmask surface is not exported. |
| `kIOHIDReportCommandGetReport` | enum constant | `IOHIDDeviceTypes.h` | Low-level report command/option enums from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDReportCommandSetReport` | enum constant | `IOHIDDeviceTypes.h` | Low-level report command/option enums from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDReportOptionNotInterrupt` | enum constant | `IOHIDDeviceTypes.h` | Low-level report command/option enums from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDReportOptionVariableSize` | enum constant | `IOHIDDeviceTypes.h` | Low-level report command/option enums from `IOHIDDeviceTypes.h` are not exposed. |
| `kIOHIDValueOptionsFlagPrevious` | enum constant | `IOHIDDeviceTypes.h` | Value-option bitmask definitions are not exposed as public Rust constants/types. |
| `kIOHIDValueOptionsFlagRelativeSimple` | enum constant | `IOHIDDeviceTypes.h` | Value-option bitmask definitions are not exposed as public Rust constants/types. |
| `kIOHIDValueOptionsUpdateElementValues` | enum constant | `IOHIDDeviceTypes.h` | Value-option bitmask definitions are not exposed as public Rust constants/types. |
| `kHIDAccelParametricCurvesKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kHIDScrollAccelParametricCurvesKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDDropAccelPropertyEventsKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDScrollResolutionKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDScrollResolutionXKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDScrollResolutionYKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDScrollResolutionZKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `kIOHIDUseLinearScalingMouseAccelerationKey` | macro | `IOHIDEventServiceKeys.h` | Missing from `hid::event_system::ALL_EVENT_SYSTEM_KEYS`. |
| `IOHIDKeyboardEventOptions` | enum | `IOHIDEventServiceTypes.h` | The module exposes renamed numeric constants, but no public Rust type mirrors this SDK enum. |
| `IOHIDPointerEventOptions` | enum | `IOHIDEventServiceTypes.h` | The module exposes renamed numeric constants, but no public Rust type mirrors this SDK enum. |
| `IOHIDScrollEventOptions` | enum | `IOHIDEventServiceTypes.h` | The module exposes renamed numeric constants, but no public Rust type mirrors this SDK enum. |
| `IOHIDServiceSensorControlOptions` | enum | `IOHIDEventServiceTypes.h` | The module exposes renamed numeric constants, but no public Rust type mirrors this SDK enum. |
| `IOHIDAccelerationAlgorithmType` | typedef | `IOHIDKeys.h` | Related numeric constants exist in `hid::keys`, but the SDK typedef itself is not exposed as a public Rust type. |
| `IOHIDKeyboardPhysicalLayoutType` | typedef | `IOHIDKeys.h` | Related numeric constants exist in `hid::keys`, but the SDK typedef itself is not exposed as a public Rust type. |
| `IOHIDOptionsType` | typedef | `IOHIDKeys.h` | Raw option constants are available, but no public `IOHIDOptionsType` alias/newtype is exported. |
| `IOHIDStandardType` | typedef | `IOHIDKeys.h` | Related numeric constants exist in `hid::keys`, but the SDK typedef itself is not exposed as a public Rust type. |
| `kIOHIDElementVendorSpecificKey` | macro | `IOHIDKeys.h` | Missing from the generated `hid::keys` string-key catalog. |
| `kIOHIDSystemButtonPressedDuringDarkBoot` | macro | `IOHIDKeys.h` | Missing from the generated `hid::keys` string-key catalog. |
| `IOHIDManagerOptions` | enum | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kIOHIDManagerOptionDoNotLoadProperties` | enum constant | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kIOHIDManagerOptionDoNotSaveProperties` | enum constant | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kIOHIDManagerOptionIndependentDevices` | enum constant | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kIOHIDManagerOptionNone` | enum constant | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kIOHIDManagerOptionUsePersistentProperties` | enum constant | `IOHIDManager.h` | The safe manager API uses default/raw option bits only; `IOHIDManagerOptions` and its flags are not surfaced. |
| `kHIDUsage_GD_SystemMenu` | enum constant | `IOHIDUsageTables.h` | `hid::usage` includes the concrete `SystemMenu*` entries, but not the alias constant `kHIDUsage_GD_SystemMenu`. |

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
