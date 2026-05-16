//! Raw FFI declarations for the `IOHIDManager` + `IOHIDDevice` subset we use.
//!
//! Pure C — no Swift bridge.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, missing_docs)]

use core::ffi::{c_char, c_void};

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFStringRef = *const c_void;
pub type CFNumberRef = *const c_void;
pub type CFSetRef = *const c_void;
pub type CFDictionaryRef = *const c_void;
pub type CFMutableDictionaryRef = *mut c_void;
pub type CFIndex = isize;

pub type IOHIDManagerRef = *mut c_void;
pub type IOHIDDeviceRef = *mut c_void;
pub type IOHIDElementRef = *mut c_void;
pub type CFArrayRef = *const c_void;
pub type IOReturn = i32;
pub type IOOptionBits = u32;

pub const kIOReturnSuccess: IOReturn = 0;

pub const kCFNumberSInt32Type: i64 = 3;
pub const kCFStringEncodingUTF8: u32 = 0x0800_0100;

pub const kIOHIDOptionsTypeNone: IOOptionBits = 0;
pub const kIOHIDOptionsTypeSeizeDevice: IOOptionBits = 1;

extern "C" {
    pub static kCFAllocatorDefault: CFAllocatorRef;
    pub static kCFTypeDictionaryKeyCallBacks: c_void;
    pub static kCFTypeDictionaryValueCallBacks: c_void;

    // CoreFoundation
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFGetTypeID(cf: CFTypeRef) -> usize;

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
    pub fn CFStringGetTypeID() -> usize;

    pub fn CFNumberCreate(
        alloc: CFAllocatorRef,
        the_type: i64,
        value_ptr: *const c_void,
    ) -> CFNumberRef;
    pub fn CFNumberGetValue(num: CFNumberRef, the_type: i64, value_ptr: *mut c_void) -> bool;
    pub fn CFNumberGetTypeID() -> usize;

    pub fn CFDictionaryCreateMutable(
        alloc: CFAllocatorRef,
        capacity: CFIndex,
        key_cb: *const c_void,
        value_cb: *const c_void,
    ) -> CFMutableDictionaryRef;
    pub fn CFDictionarySetValue(d: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    pub fn CFDictionaryGetValue(d: CFDictionaryRef, key: *const c_void) -> *const c_void;

    pub fn CFSetGetCount(set: CFSetRef) -> CFIndex;
    pub fn CFSetGetValues(set: CFSetRef, values: *mut *const c_void);

    // IOHIDManager
    pub fn IOHIDManagerCreate(
        allocator: CFAllocatorRef,
        options: IOOptionBits,
    ) -> IOHIDManagerRef;
    pub fn IOHIDManagerOpen(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerClose(manager: IOHIDManagerRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDManagerSetDeviceMatching(manager: IOHIDManagerRef, matching: CFDictionaryRef);
    pub fn IOHIDManagerCopyDevices(manager: IOHIDManagerRef) -> CFSetRef;

    // IOHIDDevice
    pub fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: CFStringRef) -> CFTypeRef;
    pub fn IOHIDDeviceSetProperty(
        device: IOHIDDeviceRef,
        key: CFStringRef,
        value: CFTypeRef,
    ) -> bool;
    pub fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: CFDictionaryRef,
        options: IOOptionBits,
    ) -> CFArrayRef;

    pub fn IOHIDElementGetType(element: IOHIDElementRef) -> i32;
    pub fn IOHIDElementGetUsage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetUsagePage(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetCookie(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementGetLogicalMin(element: IOHIDElementRef) -> i64;
    pub fn IOHIDElementGetLogicalMax(element: IOHIDElementRef) -> i64;
    pub fn IOHIDElementGetReportSize(element: IOHIDElementRef) -> u32;
    pub fn IOHIDElementIsRelative(element: IOHIDElementRef) -> bool;

    pub fn IOHIDDeviceOpen(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDDeviceClose(device: IOHIDDeviceRef, options: IOOptionBits) -> IOReturn;
    pub fn IOHIDDeviceRegisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        report_length: CFIndex,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceUnregisterInputReportCallback(
        device: IOHIDDeviceRef,
        report: *mut u8,
        report_length: CFIndex,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    );
    pub fn IOHIDDeviceScheduleWithRunLoop(
        device: IOHIDDeviceRef,
        run_loop: *mut c_void,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDDeviceUnscheduleFromRunLoop(
        device: IOHIDDeviceRef,
        run_loop: *mut c_void,
        run_loop_mode: CFStringRef,
    );

    // IOHIDManager run-loop scheduling + input-report callbacks
    pub fn IOHIDManagerScheduleWithRunLoop(
        manager: IOHIDManagerRef,
        run_loop: *mut c_void,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDManagerUnscheduleFromRunLoop(
        manager: IOHIDManagerRef,
        run_loop: *mut c_void,
        run_loop_mode: CFStringRef,
    );
    pub fn IOHIDManagerRegisterInputReportCallback(
        manager: IOHIDManagerRef,
        callback: IOHIDReportCallback,
        context: *mut c_void,
    );

    // Run-loop
    pub fn CFRunLoopGetCurrent() -> *mut c_void;
    pub static kCFRunLoopDefaultMode: CFStringRef;

    pub fn CFArrayGetCount(array: CFArrayRef) -> isize;
    pub fn CFArrayGetValueAtIndex(array: CFArrayRef, index: isize) -> *const c_void;
}

pub type IOHIDReportType = u32;
pub type IOHIDReportCallback = unsafe extern "C" fn(
    context: *mut c_void,
    result: IOReturn,
    sender: *mut c_void,
    report_type: IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: CFIndex,
);

// HID property keys — defined as CFStrings in IOHIDKeys.h. The values
// below are the canonical UTF-8 string literals.
pub const kIOHIDVendorIDKey: &str = "VendorID";
pub const kIOHIDProductIDKey: &str = "ProductID";
pub const kIOHIDProductKey: &str = "Product";
pub const kIOHIDManufacturerKey: &str = "Manufacturer";
pub const kIOHIDSerialNumberKey: &str = "SerialNumber";
pub const kIOHIDTransportKey: &str = "Transport";
pub const kIOHIDPrimaryUsagePageKey: &str = "PrimaryUsagePage";
pub const kIOHIDPrimaryUsageKey: &str = "PrimaryUsage";
pub const kIOHIDLocationIDKey: &str = "LocationID";
pub const kIOHIDDeviceUsagePageKey: &str = "DeviceUsagePage";
pub const kIOHIDDeviceUsageKey: &str = "DeviceUsage";

// HID Usage Page constants (subset)
pub const kHIDPage_GenericDesktop: u32 = 0x01;
pub const kHIDPage_KeyboardOrKeypad: u32 = 0x07;

// Generic Desktop Usage IDs
pub const kHIDUsage_GD_Mouse: u32 = 0x02;
pub const kHIDUsage_GD_Joystick: u32 = 0x04;
pub const kHIDUsage_GD_GamePad: u32 = 0x05;
pub const kHIDUsage_GD_Keyboard: u32 = 0x06;
pub const kHIDUsage_GD_Keypad: u32 = 0x07;
pub const kHIDUsage_GD_MultiAxisController: u32 = 0x08;
