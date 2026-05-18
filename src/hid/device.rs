use core::ffi::c_void;
use core::ptr;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

#[allow(clippy::type_complexity)]
struct DeviceRemovalContext {
    callback: *mut Box<dyn Fn() + Send + Sync + 'static>,
}

unsafe extern "C" fn device_removal_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    if context.is_null() || result != ffi::kIOReturnSuccess {
        return;
    }
    let callback = unsafe { &*(*context.cast::<DeviceRemovalContext>()).callback };
    callback();
}

fn build_element_array(elements: &[HidElement]) -> Result<ffi::CFArrayRef, HidError> {
    let capacity = ffi::CFIndex::try_from(elements.len())
        .map_err(|_| HidError::InvalidArgument("array size does not fit CFIndex".to_owned()))?;
    let array = unsafe {
        ffi::CFArrayCreateMutable(
            ffi::kCFAllocatorDefault,
            capacity,
            &raw const ffi::kCFTypeArrayCallBacks,
        )
    };
    if array.is_null() {
        return Err(HidError::OperationFailed("CFArrayCreateMutable"));
    }
    for element in elements {
        unsafe {
            ffi::CFArrayAppendValue(array, element.raw.cast());
        }
    }
    Ok(array.cast())
}

fn build_element_value_dictionary(
    values: &[(HidElement, HidValue)],
) -> Result<ffi::CFDictionaryRef, HidError> {
    let capacity = ffi::CFIndex::try_from(values.len()).map_err(|_| {
        HidError::InvalidArgument("dictionary size does not fit CFIndex".to_owned())
    })?;
    let dict = unsafe {
        ffi::CFDictionaryCreateMutable(
            ffi::kCFAllocatorDefault,
            capacity,
            &raw const ffi::kCFTypeDictionaryKeyCallBacks,
            &raw const ffi::kCFTypeDictionaryValueCallBacks,
        )
    };
    if dict.is_null() {
        return Err(HidError::OperationFailed("CFDictionaryCreateMutable"));
    }
    for (element, value) in values {
        unsafe {
            ffi::CFDictionarySetValue(dict, element.raw.cast(), value.raw.cast());
        }
    }
    Ok(dict.cast())
}

#[allow(clippy::filter_map_bool_then)]
fn read_element_value_dictionary(dict: ffi::CFDictionaryRef) -> Vec<(HidElement, HidValue)> {
    if dict.is_null() {
        return Vec::new();
    }
    let count = usize::try_from(unsafe { ffi::CFDictionaryGetCount(dict) }).unwrap_or(0);
    let mut keys = vec![ptr::null(); count];
    let mut values = vec![ptr::null(); count];
    unsafe {
        ffi::CFDictionaryGetKeysAndValues(dict, keys.as_mut_ptr(), values.as_mut_ptr());
    }
    keys.into_iter()
        .zip(values)
        .filter_map(|(key, value)| {
            (!key.is_null() && !value.is_null()).then(|| {
                clone_value_ref(value.cast()).map(|value| (HidElement { raw: key.cast() }, value))
            })
        })
        .flatten()
        .collect()
}

/// Owns a registration from `IOHIDDeviceRegisterRemovalCallback`.
pub struct DeviceRemovalSubscription {
    device: ffi::IOHIDDeviceRef,
    run_loop: ffi::CFRunLoopRef,
    context: *mut DeviceRemovalContext,
}

unsafe impl Send for DeviceRemovalSubscription {}

impl Drop for DeviceRemovalSubscription {
    fn drop(&mut self) {
        if self.device.is_null() || self.context.is_null() {
            return;
        }
        unsafe {
            ffi::IOHIDDeviceRegisterRemovalCallback(self.device, None, ptr::null_mut());
            close_and_unschedule_device(self.device, self.run_loop);
            ffi::CFRelease(self.device);
            let context = Box::from_raw(self.context);
            let _ = Box::from_raw(context.callback);
        }
        self.device = ptr::null();
        self.context = ptr::null_mut();
    }
}

#[allow(clippy::missing_errors_doc)]
impl HidDevice {
    /// Wraps `IOHIDDeviceCreate`.
    pub fn create(service: ffi::io_service_t) -> Result<Self, HidError> {
        let raw = unsafe { ffi::IOHIDDeviceCreate(ffi::kCFAllocatorDefault, service) };
        if raw.is_null() {
            Err(HidError::OperationFailed("IOHIDDeviceCreate"))
        } else {
            Ok(Self { raw })
        }
    }

    /// Wraps `IOHIDDeviceOpen`.
    pub fn open_with_options(&self, options: ffi::IOHIDOptionsType) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDDeviceOpen(self.raw, options) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceOpen", status))
        }
    }

    /// Wraps `IOHIDDeviceClose`.
    pub fn close_with_options(&self, options: ffi::IOHIDOptionsType) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDDeviceClose(self.raw, options) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceClose", status))
        }
    }

    /// Wraps `IOHIDDeviceActivate`.
    pub fn activate(&self) {
        unsafe { ffi::IOHIDDeviceActivate(self.raw) };
    }

    /// Wraps `IOHIDDeviceCancel`.
    pub fn cancel(&self) {
        unsafe { ffi::IOHIDDeviceCancel(self.raw) };
    }

    /// Wraps `IOHIDDeviceRegisterRemovalCallback`.
    pub fn on_removal<F>(&self, callback: F) -> Result<DeviceRemovalSubscription, HidError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let run_loop = open_and_schedule_device(self.raw)?;
        let callback: Box<dyn Fn() + Send + Sync + 'static> = Box::new(callback);
        let callback_ptr = Box::into_raw(Box::new(callback));
        let context_ptr = Box::into_raw(Box::new(DeviceRemovalContext {
            callback: callback_ptr,
        }));
        unsafe {
            ffi::IOHIDDeviceRegisterRemovalCallback(
                self.raw,
                Some(device_removal_trampoline),
                context_ptr.cast(),
            );
            ffi::CFRetain(self.raw);
        }
        Ok(DeviceRemovalSubscription {
            device: self.raw,
            run_loop,
            context: context_ptr,
        })
    }

    /// Wraps `IOHIDDeviceSetValueMultiple`.
    pub fn set_value_multiple(&self, values: &[(HidElement, HidValue)]) -> Result<(), HidError> {
        let dict = build_element_value_dictionary(values)?;
        let status = unsafe { ffi::IOHIDDeviceSetValueMultiple(self.raw, dict) };
        unsafe { ffi::CFRelease(dict.cast()) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn("IOHIDDeviceSetValueMultiple", status))
        }
    }

    /// Wraps `IOHIDDeviceSetValueMultipleWithCallback`.
    pub fn set_value_multiple_with_timeout(
        &self,
        values: &[(HidElement, HidValue)],
        timeout_ms: f64,
    ) -> Result<(), HidError> {
        let dict = build_element_value_dictionary(values)?;
        let status = unsafe {
            ffi::IOHIDDeviceSetValueMultipleWithCallback(
                self.raw,
                dict,
                timeout_ms,
                None,
                ptr::null_mut(),
            )
        };
        unsafe { ffi::CFRelease(dict.cast()) };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn(
                "IOHIDDeviceSetValueMultipleWithCallback",
                status,
            ))
        }
    }

    /// Wraps `IOHIDDeviceCopyValueMultiple`.
    pub fn copy_value_multiple(
        &self,
        elements: &[HidElement],
    ) -> Result<Vec<(HidElement, HidValue)>, HidError> {
        if elements.is_empty() {
            return Ok(Vec::new());
        }
        let array = build_element_array(elements)?;
        let mut multiple = ptr::null();
        let status = unsafe { ffi::IOHIDDeviceCopyValueMultiple(self.raw, array, &mut multiple) };
        unsafe { ffi::CFRelease(array.cast()) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceCopyValueMultiple", status));
        }
        let pairs = read_element_value_dictionary(multiple);
        if !multiple.is_null() {
            unsafe { ffi::CFRelease(multiple.cast()) };
        }
        Ok(pairs)
    }

    /// Wraps `IOHIDDeviceCopyValueMultipleWithCallback`.
    pub fn copy_value_multiple_with_timeout(
        &self,
        elements: &[HidElement],
        timeout_ms: f64,
    ) -> Result<Vec<(HidElement, HidValue)>, HidError> {
        if elements.is_empty() {
            return Ok(Vec::new());
        }
        let array = build_element_array(elements)?;
        let mut multiple = ptr::null();
        let status = unsafe {
            ffi::IOHIDDeviceCopyValueMultipleWithCallback(
                self.raw,
                array,
                &mut multiple,
                timeout_ms,
                None,
                ptr::null_mut(),
            )
        };
        unsafe { ffi::CFRelease(array.cast()) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn(
                "IOHIDDeviceCopyValueMultipleWithCallback",
                status,
            ));
        }
        let pairs = read_element_value_dictionary(multiple);
        if !multiple.is_null() {
            unsafe { ffi::CFRelease(multiple.cast()) };
        }
        Ok(pairs)
    }

    /// Wraps `IOHIDDeviceSetValueWithCallback`.
    pub fn set_value_with_timeout(
        &self,
        element: &HidElement,
        value: &HidValue,
        timeout_ms: f64,
    ) -> Result<(), HidError> {
        let status = unsafe {
            ffi::IOHIDDeviceSetValueWithCallback(
                self.raw,
                element.raw,
                value.raw,
                timeout_ms,
                None,
                ptr::null_mut(),
            )
        };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn(
                "IOHIDDeviceSetValueWithCallback",
                status,
            ))
        }
    }

    /// Wraps `IOHIDDeviceGetValueWithCallback`.
    pub fn get_value_with_timeout(
        &self,
        element: &HidElement,
        timeout_ms: f64,
    ) -> Result<HidValue, HidError> {
        let mut value = ptr::null();
        let status = unsafe {
            ffi::IOHIDDeviceGetValueWithCallback(
                self.raw,
                element.raw,
                &mut value,
                timeout_ms,
                None,
                ptr::null_mut(),
            )
        };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn(
                "IOHIDDeviceGetValueWithCallback",
                status,
            ));
        }
        clone_value_ref(value).ok_or(HidError::OperationFailed("IOHIDDeviceGetValueWithCallback"))
    }

    /// Wraps `IOHIDDeviceSetReportWithCallback`.
    pub fn set_report_with_timeout(
        &self,
        report_type: HidReportType,
        report_id: u32,
        report: &[u8],
        timeout_ms: f64,
    ) -> Result<(), HidError> {
        let report_id = ffi::CFIndex::from(report_id);
        let report_length = ffi::CFIndex::try_from(report.len()).map_err(|_| {
            HidError::InvalidArgument("report length does not fit CFIndex".to_owned())
        })?;
        let status = unsafe {
            ffi::IOHIDDeviceSetReportWithCallback(
                self.raw,
                report_type.as_raw(),
                report_id,
                report.as_ptr(),
                report_length,
                timeout_ms,
                None,
                ptr::null_mut(),
            )
        };
        if status == ffi::kIOReturnSuccess {
            Ok(())
        } else {
            Err(HidError::IoReturn(
                "IOHIDDeviceSetReportWithCallback",
                status,
            ))
        }
    }
}
