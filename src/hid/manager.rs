use core::ffi::c_void;
use core::ptr;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

type ManagerDeviceHandler = Box<dyn Fn(HidDevice) + Send + Sync + 'static>;
type ManagerReportHandler = Box<dyn Fn(HidDevice, HidInputReport) + Send + Sync + 'static>;
type ManagerValueHandler = Box<dyn Fn(HidDevice, &HidValue) + Send + Sync + 'static>;

unsafe extern "C" fn manager_device_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    device: ffi::IOHIDDeviceRef,
) {
    catch_user_panic("manager_device_trampoline", || {
        if device.is_null() || result != ffi::kIOReturnSuccess {
            return;
        }
        let _callout = Callout::enter();
        unsafe {
            CallbackContext::<ManagerDeviceHandler>::with(
                context,
                "manager_device_trampoline",
                |callback| callback(HidDevice::from_raw_retained(device)),
            );
        }
    });
}

unsafe fn deliver_report(
    context: *mut c_void,
    site: &str,
    sender: *mut c_void,
    report: HidInputReport,
) {
    let _callout = Callout::enter();
    unsafe {
        CallbackContext::<ManagerReportHandler>::with(context, site, |callback| {
            callback(HidDevice::from_raw_retained(sender.cast()), report);
        });
    }
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
    catch_user_panic("manager_report_trampoline", || {
        if sender.is_null() || report.is_null() || result != ffi::kIOReturnSuccess {
            return;
        }
        let length = usize::try_from(report_length).unwrap_or(0);
        let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
        let report = HidInputReport {
            report_type: HidReportType::from_raw(report_type),
            report_id,
            bytes,
            timestamp: 0,
        };
        unsafe { deliver_report(context, "manager_report_trampoline", sender, report) };
    });
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
    catch_user_panic("manager_timestamped_report_trampoline", || {
        if sender.is_null() || report.is_null() || result != ffi::kIOReturnSuccess {
            return;
        }
        let length = usize::try_from(report_length).unwrap_or(0);
        let bytes = unsafe { core::slice::from_raw_parts(report.cast_const(), length) }.to_vec();
        let report = HidInputReport {
            report_type: HidReportType::from_raw(report_type),
            report_id,
            bytes,
            timestamp,
        };
        unsafe {
            deliver_report(
                context,
                "manager_timestamped_report_trampoline",
                sender,
                report,
            );
        }
    });
}

unsafe extern "C" fn manager_value_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    sender: *mut c_void,
    value: ffi::IOHIDValueRef,
) {
    catch_user_panic("manager_value_trampoline", || {
        if sender.is_null() || value.is_null() || result != ffi::kIOReturnSuccess {
            return;
        }
        let _callout = Callout::enter();
        unsafe {
            CallbackContext::<ManagerValueHandler>::with(
                context,
                "manager_value_trampoline",
                |callback| {
                    let device = HidDevice::from_raw_retained(sender.cast());
                    if let Some(value) = clone_value_ref(value, Some(&device)) {
                        callback(device, &value);
                    }
                },
            );
        }
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagerCallback {
    Matching,
    Removal,
    Report,
    TimestampedReport,
    Value,
}

impl ManagerCallback {
    const fn opens_devices(self) -> bool {
        matches!(self, Self::Report | Self::TimestampedReport | Self::Value)
    }

    unsafe fn register(self, manager: ffi::IOHIDManagerRef, context: *mut c_void) {
        match self {
            Self::Matching => ffi::IOHIDManagerRegisterDeviceMatchingCallback(
                manager,
                Some(manager_device_trampoline),
                context,
            ),
            Self::Removal => ffi::IOHIDManagerRegisterDeviceRemovalCallback(
                manager,
                Some(manager_device_trampoline),
                context,
            ),
            Self::Report => ffi::IOHIDManagerRegisterInputReportCallback(
                manager,
                Some(manager_report_trampoline),
                context,
            ),
            Self::TimestampedReport => ffi::IOHIDManagerRegisterInputReportWithTimeStampCallback(
                manager,
                Some(manager_timestamped_report_trampoline),
                context,
            ),
            Self::Value => ffi::IOHIDManagerRegisterInputValueCallback(
                manager,
                Some(manager_value_trampoline),
                context,
            ),
        }
    }

    unsafe fn unregister(self, manager: ffi::IOHIDManagerRef) {
        match self {
            Self::Matching => {
                ffi::IOHIDManagerRegisterDeviceMatchingCallback(manager, None, ptr::null_mut());
            }
            Self::Removal => {
                ffi::IOHIDManagerRegisterDeviceRemovalCallback(manager, None, ptr::null_mut());
            }
            Self::Report => {
                ffi::IOHIDManagerRegisterInputReportCallback(manager, None, ptr::null_mut());
            }
            Self::TimestampedReport => {
                ffi::IOHIDManagerRegisterInputReportWithTimeStampCallback(
                    manager,
                    None,
                    ptr::null_mut(),
                );
            }
            Self::Value => {
                ffi::IOHIDManagerRegisterInputValueCallback(manager, None, ptr::null_mut());
            }
        }
    }
}

fn open_private_manager(
    devices: Option<Retained>,
    inputs: Option<Retained>,
    callback: ManagerCallback,
    foreign: ForeignRef,
) -> Result<Teardown, HidError> {
    let manager = Retained::from_create(unsafe {
        ffi::IOHIDManagerCreate(ffi::kCFAllocatorDefault, ffi::kIOHIDManagerOptionNone)
    })
    .ok_or(HidError::ManagerCreateFailed)?;
    unsafe {
        ffi::IOHIDManagerSetDeviceMatchingMultiple(manager.get(), array_ptr(devices.as_ref()));
        if inputs.is_some() {
            ffi::IOHIDManagerSetInputValueMatchingMultiple(
                manager.get(),
                array_ptr(inputs.as_ref()),
            );
        }
        callback.register(manager.get(), foreign.as_ptr());
    }
    drop(devices);
    drop(inputs);
    if callback.opens_devices() {
        let status = unsafe { ffi::IOHIDManagerOpen(manager.get(), ffi::kIOHIDOptionsTypeNone) };
        if status != ffi::kIOReturnSuccess {
            unsafe {
                let _ = ffi::IOHIDManagerClose(manager.get(), ffi::kIOHIDOptionsTypeNone);
                callback.unregister(manager.get());
            }
            return Err(HidError::ManagerOpenFailed(status));
        }
    }
    unsafe {
        ffi::IOHIDManagerScheduleWithRunLoop(
            manager.get(),
            ffi::CFRunLoopGetCurrent(),
            ffi::kCFRunLoopDefaultMode,
        );
    }
    Ok(Box::new(move || {
        unsafe {
            let _ = ffi::IOHIDManagerClose(manager.get(), ffi::kIOHIDOptionsTypeNone);
            callback.unregister(manager.get());
        }
        drop(manager);
        drop(foreign);
    }))
}

/// Owns a registration from `IOHIDManagerRegisterDeviceMatchingCallback` or `IOHIDManagerRegisterDeviceRemovalCallback`.
pub struct ManagerDeviceSubscription {
    _registration: Registration<ManagerDeviceHandler>,
}

/// Owns a registration from `IOHIDManagerRegisterInputReportCallback` or `IOHIDManagerRegisterInputReportWithTimeStampCallback`.
pub struct ManagerReportSubscription {
    _registration: Registration<ManagerReportHandler>,
}

/// Owns a registration from `IOHIDManagerRegisterInputValueCallback`.
pub struct ManagerValueSubscription {
    _registration: Registration<ManagerValueHandler>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Wraps the `IOHIDManagerOptions` bitfield used by `IOHIDManagerCreate`.
pub struct HidManagerOptions(u32);

impl HidManagerOptions {
    /// Mirrors `kIOHIDManagerOptionNone`.
    pub const NONE: Self = Self(ffi::kIOHIDManagerOptionNone);
    /// Mirrors `kIOHIDManagerOptionUsePersistentProperties`.
    pub const USE_PERSISTENT_PROPERTIES: Self =
        Self(ffi::kIOHIDManagerOptionUsePersistentProperties);
    /// Mirrors `kIOHIDManagerOptionDoNotLoadProperties`.
    pub const DO_NOT_LOAD_PROPERTIES: Self = Self(ffi::kIOHIDManagerOptionDoNotLoadProperties);
    /// Mirrors `kIOHIDManagerOptionDoNotSaveProperties`.
    pub const DO_NOT_SAVE_PROPERTIES: Self = Self(ffi::kIOHIDManagerOptionDoNotSaveProperties);
    /// Mirrors `kIOHIDManagerOptionIndependentDevices`.
    pub const INDEPENDENT_DEVICES: Self = Self(ffi::kIOHIDManagerOptionIndependentDevices);

    #[must_use]
    /// Mirrors `IOHIDManagerOptions`.
    pub const fn bits(self) -> ffi::IOHIDManagerOptions {
        self.0
    }

    #[must_use]
    /// Mirrors `IOHIDManagerOptions`.
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

#[allow(clippy::missing_errors_doc)]
impl HidManager {
    /// Wraps `IOHIDManagerCreate` and `IOHIDManagerOpen`.
    pub fn with_options(options: HidManagerOptions) -> Result<Self, HidError> {
        let raw = unsafe { ffi::IOHIDManagerCreate(ffi::kCFAllocatorDefault, options.bits()) };
        if raw.is_null() {
            return Err(HidError::ManagerCreateFailed);
        }
        let status = unsafe { ffi::IOHIDManagerOpen(raw, ffi::kIOHIDOptionsTypeNone) };
        if status != ffi::kIOReturnSuccess {
            unsafe { ffi::CFRelease(raw) };
            return Err(HidError::ManagerOpenFailed(status));
        }
        Ok(Self {
            raw,
            device_matching: RefCell::new(None),
            input_value_matching: RefCell::new(Vec::new()),
        })
    }

    /// Wraps `IOHIDManagerSaveToPropertyDomain` with `HidManagerOptions`.
    pub fn save_to_property_domain_with_options(
        &self,
        application_id: &str,
        user_name: &str,
        host_name: &str,
        options: HidManagerOptions,
    ) -> Result<(), HidError> {
        self.save_to_property_domain(application_id, user_name, host_name, options.bits())
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDManagerActivate`.
    pub unsafe fn activate(&self) {
        unsafe { ffi::IOHIDManagerActivate(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDManagerCancel`.
    pub unsafe fn cancel(&self) {
        unsafe { ffi::IOHIDManagerCancel(self.raw) };
    }

    /// Wraps `IOHIDManagerSetInputValueMatching`.
    pub fn set_input_value_matching(
        &self,
        matching: Option<&ElementMatch>,
    ) -> Result<(), HidError> {
        self.set_input_value_matching_multiple(
            matching.map(core::slice::from_ref).unwrap_or_default(),
        )
    }

    /// Wraps `IOHIDManagerSetInputValueMatchingMultiple`.
    pub fn set_input_value_matching_multiple(
        &self,
        matches: &[ElementMatch],
    ) -> Result<(), HidError> {
        let array = matching_array(matches, ElementMatch::to_cf_dictionary)?;
        unsafe {
            ffi::IOHIDManagerSetInputValueMatchingMultiple(self.raw, array_ptr(array.as_ref()));
        }
        *self.input_value_matching.borrow_mut() = matches.to_vec();
        Ok(())
    }

    fn subscribe<T: Send + Sync + 'static>(
        &self,
        handler: T,
        callback: ManagerCallback,
    ) -> Result<Registration<T>, HidError> {
        let devices = self.device_matching.borrow().clone().ok_or_else(|| {
            HidError::InvalidArgument("set the device matching before subscribing".to_owned())
        })?;
        let devices = matching_array(&devices, DeviceMatch::to_cf_dictionary)?;
        let inputs = matching_array(
            &self.input_value_matching.borrow(),
            ElementMatch::to_cf_dictionary,
        )?;
        let context = CallbackContext::new(handler);
        let foreign = ForeignRef::new(&context);
        let teardown = event_loop::run_sync(move || {
            open_private_manager(devices, inputs, callback, foreign)
        })??;
        Ok(Registration::new(context, teardown))
    }

    /// Wraps `IOHIDManagerRegisterDeviceMatchingCallback`.
    pub fn on_device_matching<F>(&self, callback: F) -> Result<ManagerDeviceSubscription, HidError>
    where
        F: Fn(HidDevice) + Send + Sync + 'static,
    {
        let handler: ManagerDeviceHandler = Box::new(callback);
        self.subscribe(handler, ManagerCallback::Matching)
            .map(|registration| ManagerDeviceSubscription {
                _registration: registration,
            })
    }

    /// Wraps `IOHIDManagerRegisterDeviceRemovalCallback`.
    pub fn on_device_removal<F>(&self, callback: F) -> Result<ManagerDeviceSubscription, HidError>
    where
        F: Fn(HidDevice) + Send + Sync + 'static,
    {
        let handler: ManagerDeviceHandler = Box::new(callback);
        self.subscribe(handler, ManagerCallback::Removal)
            .map(|registration| ManagerDeviceSubscription {
                _registration: registration,
            })
    }

    /// Wraps `IOHIDManagerRegisterInputReportCallback`.
    pub fn on_input_report<F>(&self, callback: F) -> Result<ManagerReportSubscription, HidError>
    where
        F: Fn(HidDevice, HidInputReport) + Send + Sync + 'static,
    {
        let handler: ManagerReportHandler = Box::new(callback);
        self.subscribe(handler, ManagerCallback::Report)
            .map(|registration| ManagerReportSubscription {
                _registration: registration,
            })
    }

    /// Wraps `IOHIDManagerRegisterInputReportWithTimeStampCallback`.
    pub fn on_input_report_with_timestamp<F>(
        &self,
        callback: F,
    ) -> Result<ManagerReportSubscription, HidError>
    where
        F: Fn(HidDevice, HidInputReport) + Send + Sync + 'static,
    {
        let handler: ManagerReportHandler = Box::new(callback);
        self.subscribe(handler, ManagerCallback::TimestampedReport)
            .map(|registration| ManagerReportSubscription {
                _registration: registration,
            })
    }

    /// Wraps `IOHIDManagerRegisterInputValueCallback`.
    pub fn on_input_value<F>(&self, callback: F) -> Result<ManagerValueSubscription, HidError>
    where
        F: Fn(HidDevice, &HidValue) + Send + Sync + 'static,
    {
        let handler: ManagerValueHandler = Box::new(callback);
        self.subscribe(handler, ManagerCallback::Value)
            .map(|registration| ManagerValueSubscription {
                _registration: registration,
            })
    }
}
