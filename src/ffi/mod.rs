//! Raw FFI declarations for the current public `IOKit` HID headers.
//!
//! Pure C — no Swift bridge.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    missing_docs
)]

use core::ffi::{c_char, c_void};

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFNumberRef = *const c_void;
pub type CFDataRef = *const c_void;
pub type CFArrayRef = *const c_void;
pub type CFMutableArrayRef = *mut c_void;
pub type CFSetRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;
pub type CFRunLoopRef = *mut c_void;
pub type CFIndex = isize;
pub type CFTimeInterval = f64;
pub type CFTypeID = usize;

pub type dispatch_queue_t = *mut c_void;
pub type dispatch_block_t = *mut c_void;
pub type io_service_t = u32;

pub type IOHIDManagerRef = *mut c_void;
pub type IOHIDDeviceRef = *mut c_void;
pub type IOHIDElementRef = *mut c_void;
pub type IOHIDValueRef = *mut c_void;
pub type IOReturn = i32;
pub type IOOptionBits = u32;
pub type IOHIDReportType = u32;
pub type IOHIDValueScaleType = u32;

pub const kIOReturnSuccess: IOReturn = 0;

pub const kCFNumberSInt32Type: i64 = 3;
pub const kCFNumberSInt64Type: i64 = 4;
pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

pub const kIOHIDOptionsTypeNone: IOOptionBits = 0;
pub const kIOHIDOptionsTypeSeizeDevice: IOOptionBits = 1;

pub const kIOHIDReportTypeInput: IOHIDReportType = 0;
pub const kIOHIDReportTypeOutput: IOHIDReportType = 1;
pub const kIOHIDReportTypeFeature: IOHIDReportType = 2;
pub const kIOHIDReportTypeCount: IOHIDReportType = 3;

pub const kIOHIDValueScaleTypeCalibrated: IOHIDValueScaleType = 0;
pub const kIOHIDValueScaleTypePhysical: IOHIDValueScaleType = 1;
pub const kIOHIDValueScaleTypeExponent: IOHIDValueScaleType = 2;

pub const kIOHIDDeviceGetValueWithUpdate: u32 = 0x0002_0000;
pub const kIOHIDDeviceGetValueWithoutUpdate: u32 = 0x0004_0000;

pub const kHIDPage_GenericDesktop: u32 = 0x01;
pub const kHIDPage_KeyboardOrKeypad: u32 = 0x07;

pub const kHIDUsage_GD_Mouse: u32 = 0x02;
pub const kHIDUsage_GD_Joystick: u32 = 0x04;
pub const kHIDUsage_GD_GamePad: u32 = 0x05;
pub const kHIDUsage_GD_Keyboard: u32 = 0x06;
pub const kHIDUsage_GD_Keypad: u32 = 0x07;
pub const kHIDUsage_GD_MultiAxisController: u32 = 0x08;

pub type IOHIDCallback =
    unsafe extern "C" fn(context: *mut c_void, result: IOReturn, sender: *mut c_void);
pub type IOHIDReportCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    report_type: IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: CFIndex,
);
pub type IOHIDReportWithTimeStampCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    report_type: IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: CFIndex,
    time_stamp: u64,
);
pub type IOHIDValueCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    value: IOHIDValueRef,
);
pub type IOHIDValueMultipleCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    multiple: CFDictionaryRef,
);
pub type IOHIDDeviceCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    device: IOHIDDeviceRef,
);

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFTypeArrayCallBacks: c_void;
    pub static kCFTypeDictionaryKeyCallBacks: c_void;
    pub static kCFTypeDictionaryValueCallBacks: c_void;
    pub static kCFRunLoopDefaultMode: CFStringRef;

    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;

    pub fn CFStringCreateWithCString(
        alloc: CFAllocatorRef,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;
    pub fn CFStringGetLength(s: CFStringRef) -> CFIndex;
    pub fn CFStringGetCString(
        s: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: u32,
    ) -> bool;
    pub fn CFStringGetTypeID() -> CFTypeID;

    pub fn CFNumberCreate(
        alloc: CFAllocatorRef,
        the_type: i64,
        value_ptr: *const c_void,
    ) -> CFNumberRef;
    pub fn CFNumberGetValue(num: CFNumberRef, the_type: i64, value_ptr: *mut c_void) -> bool;
    pub fn CFNumberGetTypeID() -> CFTypeID;

    pub fn CFDataCreate(alloc: CFAllocatorRef, bytes: *const u8, length: CFIndex) -> CFDataRef;
    pub fn CFDataGetTypeID() -> CFTypeID;
    pub fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    pub fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;

    pub fn CFArrayCreateMutable(
        alloc: CFAllocatorRef,
        capacity: CFIndex,
        call_backs: *const c_void,
    ) -> CFMutableArrayRef;
    pub fn CFArrayAppendValue(array: CFMutableArrayRef, value: *const c_void);
    pub fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    pub fn CFArrayGetValueAtIndex(array: CFArrayRef, index: CFIndex) -> *const c_void;

    pub fn CFDictionaryCreateMutable(
        alloc: CFAllocatorRef,
        capacity: CFIndex,
        key_cb: *const c_void,
        value_cb: *const c_void,
    ) -> CFMutableDictionaryRef;
    pub fn CFDictionarySetValue(
        d: CFMutableDictionaryRef,
        key: *const c_void,
        value: *const c_void,
    );
    pub fn CFDictionaryGetValue(d: CFDictionaryRef, key: *const c_void) -> *const c_void;

    pub fn CFSetGetCount(set: CFSetRef) -> CFIndex;
    pub fn CFSetGetValues(set: CFSetRef, values: *mut *const c_void);

    pub fn CFRunLoopGetCurrent() -> CFRunLoopRef;

    pub fn IOHIDManagerGetTypeID() -> CFTypeID;
    pub fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: IOOptionBits) -> IOHIDManagerRef;
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerGetProperty(manager: IOHIDManagerRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDManagerSetProperty(
        manager: IOHIDManagerRef,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDManagerUnscheduleFromRunLoop(
        manager: IOHIDManagerRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDManagerSetDispatchQueue(manager: IOHIDManagerRef, queue: dispatch_queue_t);
    pub fn IOHIDManagerSetCancelHandler(manager: IOHIDManagerRef, handler: dispatch_block_t);
    pub fn IOHIDManagerActivate(manager: IOHIDManagerRef);
    pub fn IOHIDManagerCancel(manager: IOHIDManagerRef);
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerSetDeviceMatchingMultiple(manager: IOHIDManagerRef, multiple: CFArrayRef);
    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;
    pub fn IOHIDManagerRegisterDeviceMatchingCallback(
        manager: IOHIDManagerRef,
        callback: Option<IOHIDDeviceCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDManagerRegisterDeviceRemovalCallback(
        manager: IOHIDManagerRef,
        callback: Option<IOHIDDeviceCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDManagerRegisterInputReportCallback(
        manager: IOHIDManagerRef,
        callback: Option<IOHIDReportCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDManagerRegisterInputReportWithTimeStampCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportWithTimeStampCallback,
        context: *mut c_void,
    );
    pub fn IOHIDManagerRegisterInputValueCallback(
        manager: IOHIDManagerRef,
        callback: Option<IOHIDValueCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDManagerSetInputValueMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerSetInputValueMatchingMultiple(
        manager: IOHIDManagerRef,
        multiple: CFArrayRef,
    );
    pub fn IOHIDManagerSaveToPropertyDomain(
        manager: IOHIDManagerRef,
        application_id: CFStringRef,
        user_name: CFStringRef,
        host_name: CFStringRef,
        options: IOOptionBits,
    );

    pub fn IOHIDDeviceGetTypeID() -> CFTypeID;
    pub fn IOHIDDeviceCreate(allocator: CFAllocatorRef, service: io_service_t) -> IOHIDDeviceRef;
    pub fn IOHIDDeviceGetService(device: IOHIDDeviceRef) -> io_service_t;
    pub fn IOHIDDeviceOpen(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDDeviceConformsTo(device: IOHIDDeviceRef, usage_page: u32, usage: u32) -> bool;
    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDDeviceSetProperty(
        device: IOHIDDeviceRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> bool;
    pub fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: CFDictionaryRef,
        options: IOOptionBits,
    ) -> CFArrayRef;
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDDeviceUnscheduleFromRunLoop(
        device: IOHIDDeviceRef,
        run_loop: CFRunLoopRef,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDDeviceSetDispatchQueue(device: IOHIDDeviceRef, queue: dispatch_queue_t);
    pub fn IOHIDDeviceSetCancelHandler(device: IOHIDDeviceRef, handler: dispatch_block_t);
    pub fn IOHIDDeviceActivate(device: IOHIDDeviceRef);
    pub fn IOHIDDeviceCancel(device: IOHIDDeviceRef);
    pub fn IOHIDDeviceRegisterRemovalCallback(
        device: IOHIDDeviceRef,
        callback: Option<IOHIDCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceRegisterInputValueCallback(
        device: IOHIDDeviceRef,
        callback: Option<IOHIDValueCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceRegisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        report_length: CFIndex,
        callback: Option<IOHIDReportCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceRegisterInputReportWithTimeStampCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        report_length: CFIndex,
        callback: Option<IOHIDReportWithTimeStampCallback>,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceSetInputValueMatching(device: IOHIDDeviceRef, matching: CFDictionaryRef);
    pub fn IOHIDDeviceSetInputValueMatchingMultiple(device: IOHIDDeviceRef, multiple: CFArrayRef);
    pub fn IOHIDDeviceSetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
    ) -> IOReturn;
    pub fn IOHIDDeviceSetValueMultiple(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
    ) -> IOReturn;
    pub fn IOHIDDeviceSetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: Option<IOHIDValueCallback>,
        context: *mut c_void,
    ) -> IOReturn;
    pub fn IOHIDDeviceSetValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        multiple: CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: Option<IOHIDValueMultipleCallback>,
        context: *mut c_void,
    ) -> IOReturn;
    pub fn IOHIDDeviceGetValue(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: *mut IOHIDValueRef,
    ) -> IOReturn;
    pub fn IOHIDDeviceGetValueWithOptions(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: *mut IOHIDValueRef,
        options: u32,
    ) -> IOReturn;
    pub fn IOHIDDeviceCopyValueMultiple(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        multiple: *mut CFDictionaryRef,
    ) -> IOReturn;
    pub fn IOHIDDeviceGetValueWithCallback(
        device: IOHIDDeviceRef,
        element: IOHIDElementRef,
        value: *mut IOHIDValueRef,
        timeout: CFTimeInterval,
        callback: Option<IOHIDValueCallback>,
        context: *mut c_void,
    ) -> IOReturn;
    pub fn IOHIDDeviceCopyValueMultipleWithCallback(
        device: IOHIDDeviceRef,
        elements: CFArrayRef,
        multiple: *mut CFDictionaryRef,
        timeout: CFTimeInterval,
        callback: Option<IOHIDValueMultipleCallback>,
        context: *mut c_void,
    ) -> IOReturn;
    pub fn IOHIDDeviceSetReport(
        device: IOHIDDeviceRef,
        report_type: IOHIDReportType,
        report_id: CFIndex,
        report: *const u8,
        report_length: CFIndex,
    ) -> IOReturn;
    pub fn IOHIDDeviceSetReportWithCallback(
        device: IOHIDDeviceRef,
        report_type: IOHIDReportType,
        report_id: CFIndex,
        report: *const u8,
        report_length: CFIndex,
        timeout: CFTimeInterval,
        callback: Option<IOHIDReportCallback>,
        context: *mut c_void,
    ) -> IOReturn;
    pub fn IOHIDDeviceGetReport(
        device: IOHIDDeviceRef,
        report_type: IOHIDReportType,
        report_id: CFIndex,
        report: *mut u8,
        report_length: *mut CFIndex,
    ) -> IOReturn;
    pub fn IOHIDDeviceGetReportWithCallback(
        device: IOHIDDeviceRef,
        report_type: IOHIDReportType,
        report_id: CFIndex,
        report: *mut u8,
        report_length: *mut CFIndex,
        timeout: CFTimeInterval,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    ) -> IOReturn;

    pub fn IOHIDElementGetTypeID() -> CFTypeID;
    pub fn IOHIDElementCreateWithDictionary(
        allocator: CFAllocatorRef,
        dictionary: CFDictionaryRef,
    ) -> IOHIDElementRef;
    pub fn IOHIDElementGetDevice(element: IOHIDElementRef) -> IOHIDDeviceRef;
    pub fn IOHIDElementGetParent(element: IOHIDElementRef) -> IOHIDElementRef;
    pub fn IOHIDElementGetChildren(element: IOHIDElementRef) -> CFArrayRef;
    pub fn IOHIDElementAttach(element: IOHIDElementRef, to_attach: IOHIDElementRef);
    pub fn IOHIDElementDetach(element: IOHIDElementRef, to_detach: IOHIDElementRef);
    pub fn IOHIDElementCopyAttached(element: IOHIDElementRef) -> CFArrayRef;
    pub fn IOHIDElementGetCookie(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetType(element: IOHIDElementRef) -> i32;
    pub fn IOHIDElementGetCollectionType(element: IOHIDElementRef) -> i32;
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementIsVirtual(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementIsRelative(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementIsWrapping(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementIsArray(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementIsNonLinear(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementHasPreferredState(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementHasNullState(element: IOHIDElementRef) -> bool;
    pub fn IOHIDElementGetName(element: IOHIDElementRef) -> CFStringRef;
    pub fn IOHIDElementGetReportID(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetReportSize(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetReportCount(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUnit(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUnitExponent(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetLogicalMin(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetLogicalMax(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetPhysicalMin(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetPhysicalMax(element: IOHIDElementRef) -> CFIndex;
    pub fn IOHIDElementGetProperty(element: IOHIDElementRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDElementSetProperty(
        element: IOHIDElementRef,
        key: CFStringRef,
        property: CFTypeRef,
    ) -> bool;

    pub fn IOHIDValueGetTypeID() -> CFTypeID;
    pub fn IOHIDValueCreateWithIntegerValue(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        time_stamp: u64,
        value: CFIndex,
    ) -> IOHIDValueRef;
    pub fn IOHIDValueCreateWithBytes(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        time_stamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;
    pub fn IOHIDValueCreateWithBytesNoCopy(
        allocator: CFAllocatorRef,
        element: IOHIDElementRef,
        time_stamp: u64,
        bytes: *const u8,
        length: CFIndex,
    ) -> IOHIDValueRef;
    pub fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    pub fn IOHIDValueGetTimeStamp(value: IOHIDValueRef) -> u64;
    pub fn IOHIDValueGetLength(value: IOHIDValueRef) -> CFIndex;
    pub fn IOHIDValueGetBytePtr(value: IOHIDValueRef) -> *const u8;
    pub fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> CFIndex;
    pub fn IOHIDValueGetScaledValue(value: IOHIDValueRef, scale_type: IOHIDValueScaleType) -> f64;
}

pub const kIOHIDVendorIDKey: &str = "VendorID";
pub const kIOHIDProductIDKey: &str = "ProductID";
pub const kIOHIDProductKey: &str = "Product";
pub const kIOHIDManufacturerKey: &str = "Manufacturer";
pub const kIOHIDSerialNumberKey: &str = "SerialNumber";
pub const kIOHIDTransportKey: &str = "Transport";
pub const kIOHIDLocationIDKey: &str = "LocationID";
pub const kIOHIDDeviceUsagePairsKey: &str = "DeviceUsagePairs";
pub const kIOHIDDeviceUsagePageKey: &str = "DeviceUsagePage";
pub const kIOHIDDeviceUsageKey: &str = "DeviceUsage";
pub const kIOHIDPrimaryUsagePageKey: &str = "PrimaryUsagePage";
pub const kIOHIDPrimaryUsageKey: &str = "PrimaryUsage";
pub const kIOHIDReportDescriptorKey: &str = "ReportDescriptor";
pub const kIOHIDElementUsageKey: &str = "Usage";
pub const kIOHIDElementUsagePageKey: &str = "UsagePage";
pub const kIOHIDElementReportIDKey: &str = "ReportID";
pub const kIOHIDElementCookieMinKey: &str = "ElementCookieMin";
pub const kIOHIDElementCookieMaxKey: &str = "ElementCookieMax";
pub const kIOHIDElementUsageMinKey: &str = "UsageMin";
pub const kIOHIDElementUsageMaxKey: &str = "UsageMax";
