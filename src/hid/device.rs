use core::ffi::c_void;
use core::ptr;
use std::time::Duration;

#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

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

fn read_element_value_dictionary(
    dict: ffi::CFDictionaryRef,
    device: &HidDevice,
) -> Vec<(HidElement, HidValue)> {
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
            let element = HidElement::from_raw_retained(key, Some(device))?;
            let value = clone_value_ref(value, Some(device))?;
            Some((element, value))
        })
        .collect()
}

/// Owns a registration from `IOHIDDeviceRegisterRemovalCallback`.
pub struct DeviceRemovalSubscription {
    _registration: Registration<RemovalHandler>,
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

    pub fn from_id(id: HidDeviceId) -> Result<Self, HidError> {
        let matching = unsafe { ffi::IORegistryEntryIDMatching(id.0) };
        if matching.is_null() {
            return Err(HidError::OperationFailed("IORegistryEntryIDMatching"));
        }
        let service = unsafe {
            ffi::IOServiceGetMatchingService(ffi::kIOMainPortDefault, matching.cast_const())
        };
        if service == 0 {
            return Err(HidError::OperationFailed("IOServiceGetMatchingService"));
        }
        let device = Self::create(service);
        unsafe { ffi::IOObjectRelease(service) };
        device
    }

    pub fn id(&self) -> Result<HidDeviceId, HidError> {
        let service = self.registered_service()?;
        let mut entry_id = 0_u64;
        let status = unsafe { ffi::IORegistryEntryGetRegistryEntryID(service, &raw mut entry_id) };
        if status == ffi::kIOReturnSuccess {
            Ok(HidDeviceId(entry_id))
        } else {
            Err(HidError::IoReturn(
                "IORegistryEntryGetRegistryEntryID",
                status,
            ))
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

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDDeviceActivate`.
    pub unsafe fn activate(&self) {
        unsafe { ffi::IOHIDDeviceActivate(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDDeviceCancel`.
    pub unsafe fn cancel(&self) {
        unsafe { ffi::IOHIDDeviceCancel(self.raw) };
    }

    /// Wraps `IOHIDDeviceRegisterRemovalCallback`.
    pub fn on_removal<F>(&self, callback: F) -> Result<DeviceRemovalSubscription, HidError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let handler: RemovalHandler = Box::new(callback);
        subscribe_device(self, handler, DeviceCallback::Removal).map(|registration| {
            DeviceRemovalSubscription {
                _registration: registration,
            }
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
        let status =
            unsafe { ffi::IOHIDDeviceCopyValueMultiple(self.raw, array, &raw mut multiple) };
        unsafe { ffi::CFRelease(array.cast()) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::IoReturn("IOHIDDeviceCopyValueMultiple", status));
        }
        let pairs = read_element_value_dictionary(multiple, self);
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
                &raw mut multiple,
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
        let pairs = read_element_value_dictionary(multiple, self);
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
                &raw mut value,
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
        clone_value_ref(value, Some(self))
            .ok_or(HidError::OperationFailed("IOHIDDeviceGetValueWithCallback"))
    }

    /// Wraps `IOHIDDeviceSetReportWithCallback`.
    pub fn set_report_with_timeout(
        &self,
        report_type: HidReportType,
        report_id: u32,
        report: &[u8],
        timeout: Duration,
    ) -> Result<(), HidError> {
        let timeout_ms = completion::request_timeout_ms(timeout)?;
        let service = self.registered_service()?;
        let report = report.to_vec();
        completion::complete_on_private_device(
            SET_REPORT_WITH_CALLBACK,
            service,
            timeout,
            |device, context| unsafe {
                send_report_with_callback(
                    device,
                    report_type,
                    report_id,
                    report,
                    timeout_ms,
                    context,
                )
            },
        )
    }
}

const SET_REPORT_WITH_CALLBACK: &str = "IOHIDDeviceSetReportWithCallback";

pub(super) unsafe fn send_report_with_callback(
    device: ffi::IOHIDDeviceRef,
    report_type: HidReportType,
    report_id: u32,
    report: Vec<u8>,
    timeout_ms: f64,
    context: *mut c_void,
) -> Result<Vec<u8>, HidError> {
    let report_length = ffi::CFIndex::try_from(report.len())
        .map_err(|_| HidError::InvalidArgument("report length does not fit CFIndex".to_owned()))?;
    let status = unsafe {
        ffi::IOHIDDeviceSetReportWithCallback(
            device,
            report_type.as_raw(),
            ffi::CFIndex::from(report_id),
            report.as_ptr(),
            report_length,
            timeout_ms,
            Some(completion::report_completed),
            context,
        )
    };
    if status == ffi::kIOReturnSuccess {
        Ok(report)
    } else {
        Err(HidError::IoReturn(SET_REPORT_WITH_CALLBACK, status))
    }
}
