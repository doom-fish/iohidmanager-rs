use core::ffi::c_void;
use core::ptr;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagerDeviceCallbackKind {
    Matching,
    Removal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagerReportCallbackKind {
    Untimestamped,
    Timestamped,
}

#[allow(clippy::type_complexity)]
struct ManagerDeviceContext {
    callback: *mut Box<dyn Fn(HidDevice) + Send + Sync + 'static>,
}

#[allow(clippy::type_complexity)]
struct ManagerReportContext {
    callback: *mut Box<dyn Fn(HidDevice, HidInputReport) + Send + Sync + 'static>,
}

#[allow(clippy::type_complexity)]
struct ManagerValueContext {
    callback: *mut Box<dyn Fn(HidDevice, &HidValue) + Send + Sync + 'static>,
}

unsafe extern "C" fn manager_device_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    device: ffi::IOHIDDeviceRef,
) {
    if context.is_null() || device.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    unsafe { ffi::CFRetain(device.cast_const()) };
    let callback = unsafe { &*(*context.cast::<ManagerDeviceContext>()).callback };
    callback(HidDevice { raw: device });
}

unsafe extern "C" fn manager_report_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void,
    report_type: ffi::IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: ffi::CFIndex,
) {
    if context.is_null() || sender.is_null() || report.is_null() || result != ffi::kIOReturnSuccess
    {
        return;
    }
    let Some(device) = retained_device_from_sender(sender) else {
        return;
    };
    let length = usize::try_from(report_length).unwrap_or(0);
    let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
    let callback = unsafe { &*(*context.cast::<ManagerReportContext>()).callback };
    callback(
        device,
        HidInputReport {
            report_type: HidReportType::from_raw(report_type),
            report_id,
            bytes,
            timestamp: 0,
        },
    );
}

unsafe extern "C" fn manager_timestamped_report_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void,
    report_type: ffi::IOHIDReportType,
    report_id: u32,
    report: *mut u8,
    report_length: ffi::CFIndex,
    timestamp: u64,
) {
    if context.is_null() || sender.is_null() || report.is_null() || result != ffi::kIOReturnSuccess
    {
        return;
    }
    let Some(device) = retained_device_from_sender(sender) else {
        return;
    };
    let length = usize::try_from(report_length).unwrap_or(0);
    let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
    let callback = unsafe { &*(*context.cast::<ManagerReportContext>()).callback };
    callback(
        device,
        HidInputReport {
            report_type: HidReportType::from_raw(report_type),
            report_id,
            bytes,
            timestamp,
        },
    );
}

unsafe extern "C" fn manager_value_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void,
    value: ffi::IOHIDValueRef,
) {
    if context.is_null() || sender.is_null() || value.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let Some(device) = retained_device_from_sender(sender) else {
        return;
    };
    let Some(value) = clone_value_ref(value) else {
        return;
    };
    let callback = unsafe { &*(*context.cast::<ManagerValueContext>()).callback };
    callback(device, &value);
}

fn retained_device_from_sender(sender: *mut c_void) -> Option<HidDevice> {
    let device = sender.cast::<c_void>();
    if device.is_null() {
        return None;
    }
    unsafe { ffi::CFRetain(device.cast_const()) };
    Some(HidDevice { raw: device.cast() })
}

fn schedule_manager(manager: ffi::IOHIDManagerRef) -> ffi::CFRunLoopRef {
    let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
    unsafe {
        ffi::IOHIDManagerScheduleWithRunLoop(manager, run_loop, ffi::kCFRunLoopDefaultMode);
    }
    run_loop
}

unsafe fn unschedule_manager(manager: ffi::IOHIDManagerRef, run_loop: ffi::CFRunLoopRef) {
    ffi::IOHIDManagerUnscheduleFromRunLoop(manager, run_loop, ffi::kCFRunLoopDefaultMode);
}

pub struct ManagerDeviceSubscription {
    manager: ffi::IOHIDManagerRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut ManagerDeviceContext,
    kind: ManagerDeviceCallbackKind,
}

unsafe impl Send for ManagerDeviceSubscription {}

impl Drop for ManagerDeviceSubscription {
    fn drop(&mut self) {
        if self.manager.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            match self.kind {
                ManagerDeviceCallbackKind::Matching => {
                    ffi::IOHIDManagerRegisterDeviceMatchingCallback(
                        self.manager,
                        None,
                        ptr::null_mut(),
                    );
                }
                ManagerDeviceCallbackKind::Removal => {
                    ffi::IOHIDManagerRegisterDeviceRemovalCallback(
                        self.manager,
                        None,
                        ptr::null_mut(),
                    );
                }
            }
            unschedule_manager(self.manager, self.run_loop);
            ffi::CFRelease(self.manager.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.manager = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

pub struct ManagerReportSubscription {
    manager: ffi::IOHIDManagerRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut ManagerReportContext,
    kind: ManagerReportCallbackKind,
}

unsafe impl Send for ManagerReportSubscription {}

impl Drop for ManagerReportSubscription {
    fn drop(&mut self) {
        if self.manager.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            match self.kind {
                ManagerReportCallbackKind::Untimestamped => {
                    ffi::IOHIDManagerRegisterInputReportCallback(
                        self.manager,
                        None,
                        ptr::null_mut(),
                    );
                }
                ManagerReportCallbackKind::Timestamped => {
                    ffi::IOHIDManagerRegisterInputReportWithTimeStampCallback(
                        self.manager,
                        None,
                        ptr::null_mut(),
                    );
                }
            }
            unschedule_manager(self.manager, self.run_loop);
            ffi::CFRelease(self.manager.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.manager = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

pub struct ManagerValueSubscription {
    manager: ffi::IOHIDManagerRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut ManagerValueContext,
}

unsafe impl Send for ManagerValueSubscription {}

impl Drop for ManagerValueSubscription {
    fn drop(&mut self) {
        if self.manager.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            ffi::IOHIDManagerRegisterInputValueCallback(self.manager, None, ptr::null_mut());
            unschedule_manager(self.manager, self.run_loop);
            ffi::CFRelease(self.manager.cast_const());
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.manager = ptr::null_mut();
        self.context = ptr::null_mut();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HidManagerOptions(u32);

impl HidManagerOptions {
    pub const NONE: Self = Self(ffi::kIOHIDManagerOptionNone);
    pub const USE_PERSISTENT_PROPERTIES: Self =
        Self(ffi::kIOHIDManagerOptionUsePersistentProperties);
    pub const DO_NOT_LOAD_PROPERTIES: Self = Self(ffi::kIOHIDManagerOptionDoNotLoadProperties);
    pub const DO_NOT_SAVE_PROPERTIES: Self = Self(ffi::kIOHIDManagerOptionDoNotSaveProperties);
    pub const INDEPENDENT_DEVICES: Self = Self(ffi::kIOHIDManagerOptionIndependentDevices);

    #[must_use]
    pub const fn bits(self) -> ffi::IOHIDManagerOptions {
        self.0
    }

    #[must_use]
    pub const fn from_bits(bits: ffi::IOHIDManagerOptions) -> Self {
        Self(bits)
    }
}

impl core::ops::BitOr for HidManagerOptions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for HidManagerOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[allow(clippy::missing_errors_doc, clippy::type_complexity)]
impl HidManager {
    pub fn with_options(options: HidManagerOptions) -> Result<Self, HidError> {
        let bits = options.bits();
        let raw = unsafe { ffi::IOHIDManagerCreate(ffi::kCFAllocatorDefault, bits) };
        if raw.is_null() {
            return Err(HidError::ManagerCreateFailed);
        }
        let status = unsafe { ffi::IOHIDManagerOpen(raw, bits) };
        if status != ffi::kIOReturnSuccess {
            unsafe { ffi::CFRelease(raw.cast_const()) };
            return Err(HidError::ManagerOpenFailed(status));
        }
        Ok(Self { raw })
    }

    pub fn save_to_property_domain_with_options(
        &self,
        application_id: &str,
        user_name: &str,
        host_name: &str,
        options: HidManagerOptions,
    ) -> Result<(), HidError> {
        self.save_to_property_domain(application_id, user_name, host_name, options.bits())
    }
    pub fn activate(&self) {
        unsafe { ffi::IOHIDManagerActivate(self.raw) };
    }

    pub fn cancel(&self) {
        unsafe { ffi::IOHIDManagerCancel(self.raw) };
    }

    pub fn set_input_value_matching(
        &self,
        matching: Option<&ElementMatch>,
    ) -> Result<(), HidError> {
        match matching {
            None => unsafe {
                ffi::IOHIDManagerSetInputValueMatching(self.raw, ptr::null());
                Ok(())
            },
            Some(matching) => {
                let dict = matching.to_cf_dictionary()?;
                unsafe {
                    ffi::IOHIDManagerSetInputValueMatching(self.raw, dict);
                    ffi::CFRelease(dict.cast());
                }
                Ok(())
            }
        }
    }

    pub fn set_input_value_matching_multiple(
        &self,
        matches: &[ElementMatch],
    ) -> Result<(), HidError> {
        if matches.is_empty() {
            unsafe {
                ffi::IOHIDManagerSetInputValueMatchingMultiple(self.raw, ptr::null());
            }
            return Ok(());
        }
        let array = build_cf_dictionary_array(matches, ElementMatch::to_cf_dictionary)?;
        unsafe {
            ffi::IOHIDManagerSetInputValueMatchingMultiple(self.raw, array);
            ffi::CFRelease(array.cast());
        }
        Ok(())
    }

    pub fn on_device_matching<F>(&self, callback: F) -> Result<ManagerDeviceSubscription, HidError>
    where
        F: Fn(HidDevice) + Send + Sync + 'static,
    {
        let run_loop = schedule_manager(self.raw);
        let callback: Box<dyn Fn(HidDevice) + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(ManagerDeviceContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDManagerRegisterDeviceMatchingCallback(
                self.raw,
                Some(manager_device_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ManagerDeviceSubscription {
            manager: self.raw,
            run_loop,
            context: context_ptr,
            kind: ManagerDeviceCallbackKind::Matching,
        })
    }

    pub fn on_device_removal<F>(&self, callback: F) -> Result<ManagerDeviceSubscription, HidError>
    where
        F: Fn(HidDevice) + Send + Sync + 'static,
    {
        let run_loop = schedule_manager(self.raw);
        let callback: Box<dyn Fn(HidDevice) + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(ManagerDeviceContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDManagerRegisterDeviceRemovalCallback(
                self.raw,
                Some(manager_device_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ManagerDeviceSubscription {
            manager: self.raw,
            run_loop,
            context: context_ptr,
            kind: ManagerDeviceCallbackKind::Removal,
        })
    }

    pub fn on_input_report<F>(&self, callback: F) -> Result<ManagerReportSubscription, HidError>
    where
        F: Fn(HidDevice, HidInputReport) + Send + Sync + 'static,
    {
        let run_loop = schedule_manager(self.raw);
        let callback: Box<dyn Fn(HidDevice, HidInputReport) + Send + Sync + 'static> =
            Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(ManagerReportContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDManagerRegisterInputReportCallback(
                self.raw,
                Some(manager_report_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ManagerReportSubscription {
            manager: self.raw,
            run_loop,
            context: context_ptr,
            kind: ManagerReportCallbackKind::Untimestamped,
        })
    }

    pub fn on_input_report_with_timestamp<F>(
        &self,
        callback: F,
    ) -> Result<ManagerReportSubscription, HidError>
    where
        F: Fn(HidDevice, HidInputReport) + Send + Sync + 'static,
    {
        let run_loop = schedule_manager(self.raw);
        let callback: Box<dyn Fn(HidDevice, HidInputReport) + Send + Sync + 'static> =
            Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(ManagerReportContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDManagerRegisterInputReportWithTimeStampCallback(
                self.raw,
                Some(manager_timestamped_report_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ManagerReportSubscription {
            manager: self.raw,
            run_loop,
            context: context_ptr,
            kind: ManagerReportCallbackKind::Timestamped,
        })
    }

    pub fn on_input_value<F>(&self, callback: F) -> Result<ManagerValueSubscription, HidError>
    where
        F: Fn(HidDevice, &HidValue) + Send + Sync + 'static,
    {
        let run_loop = schedule_manager(self.raw);
        let callback: Box<dyn Fn(HidDevice, &HidValue) + Send + Sync + 'static> =
            Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(ManagerValueContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDManagerRegisterInputValueCallback(
                self.raw,
                Some(manager_value_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw.cast_const());
        }
        Ok(ManagerValueSubscription {
            manager: self.raw,
            run_loop,
            context: context_ptr,
        })
    }
}
