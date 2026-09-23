use core::ptr;
use std::time::Duration;

use apple_cf::raw::CFGetRetainCount;

use super::*;
use crate::ffi_impl as ffi;

trait AmbiguousIfSync<A> {
    fn assert_not_sync() {}
}
impl<T: ?Sized> AmbiguousIfSync<()> for T {}
impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}

fn assert_send<T: Send>() {}

struct DeviceLessElement(ffi::IOHIDElementRef);

impl DeviceLessElement {
    fn new() -> Self {
        let dict = unsafe {
            ffi::CFDictionaryCreateMutable(
                ffi::kCFAllocatorDefault,
                0,
                &raw const ffi::kCFTypeDictionaryKeyCallBacks,
                &raw const ffi::kCFTypeDictionaryValueCallBacks,
            )
        };
        assert!(!dict.is_null());
        let raw =
            unsafe { ffi::IOHIDElementCreateWithDictionary(ffi::kCFAllocatorDefault, dict.cast()) };
        unsafe { ffi::CFRelease(dict.cast()) };
        assert!(!raw.is_null());
        Self(raw)
    }

    fn retain_count(&self) -> ffi::CFIndex {
        unsafe { CFGetRetainCount(self.0) }
    }

    fn has_empty_layout(&self) -> bool {
        unsafe { ffi::IOHIDElementGetReportSize(self.0) == 0 && ffi::IOHIDElementGetReportCount(self.0) == 0 }
    }
}

impl Drop for DeviceLessElement {
    fn drop(&mut self) {
        unsafe { ffi::CFRelease(self.0) };
    }
}

fn stand_in_device() -> HidDevice {
    let raw = make_cfstring("stand-in device").expect("cfstring");
    HidDevice { raw: raw.cast() }
}

#[test]
fn handles_are_send_but_not_sync() {
    assert_send::<HidManager>();
    assert_send::<HidDevice>();
    assert_send::<HidElement>();
    assert_send::<HidQueue>();
    assert_send::<HidTransaction>();
    <HidManager as AmbiguousIfSync<_>>::assert_not_sync();
    <HidDevice as AmbiguousIfSync<_>>::assert_not_sync();
    <HidElement as AmbiguousIfSync<_>>::assert_not_sync();
    <HidQueue as AmbiguousIfSync<_>>::assert_not_sync();
    <HidTransaction as AmbiguousIfSync<_>>::assert_not_sync();
}

#[test]
fn element_handles_hold_their_own_reference() {
    let raw = DeviceLessElement::new();
    let base = raw.retain_count();

    let element = HidElement::from_raw_retained(raw.0, None).expect("element");
    assert_eq!(raw.retain_count(), base + 1);

    let copy = element.clone();
    assert_eq!(copy, element);
    assert_eq!(raw.retain_count(), base + 2);

    drop(copy);
    assert_eq!(raw.retain_count(), base + 1);
    drop(element);
    assert_eq!(raw.retain_count(), base);

    assert!(HidElement::from_raw_retained(ptr::null(), None).is_none());
}

#[test]
fn elements_outlive_the_array_they_came_from() {
    let raw = DeviceLessElement::new();
    let base = raw.retain_count();

    let array = unsafe {
        ffi::CFArrayCreateMutable(
            ffi::kCFAllocatorDefault,
            1,
            &raw const ffi::kCFTypeArrayCallBacks,
        )
    };
    assert!(!array.is_null());
    unsafe { ffi::CFArrayAppendValue(array, raw.0) };
    assert_eq!(raw.retain_count(), base + 1);

    let elements = elements_from_array(array.cast_const(), true, None);
    assert_eq!(elements.len(), 1);
    assert_eq!(elements[0].as_ptr(), raw.0);
    assert_eq!(raw.retain_count(), base + 1);
    assert!(elements[0].parent().is_none());
    assert!(elements[0].children().is_empty());

    drop(elements);
    assert_eq!(raw.retain_count(), base);
}

#[test]
fn elements_keep_their_device_and_share_it_with_relatives() {
    let raw = DeviceLessElement::new();
    let device = stand_in_device();
    let element = HidElement::from_raw_retained(raw.0, Some(&device)).expect("element");
    assert_eq!(element.device().map(|d| d.as_ptr()), Some(device.as_ptr()));

    let sibling = DeviceLessElement::new();
    let sibling = HidElement::from_raw_retained(sibling.0, Some(&device)).expect("sibling");
    element.attach(&sibling).expect("same device");
    let attached = element.attached();
    assert_eq!(attached, vec![sibling.clone()]);
    assert_eq!(attached[0].device().map(|d| d.as_ptr()), Some(device.as_ptr()));
    element.detach(&sibling);
    assert!(element.attached().is_empty());
}

#[test]
fn attach_rejects_elements_of_another_device() {
    let first = DeviceLessElement::new();
    let second = DeviceLessElement::new();
    let first_device = stand_in_device();
    let second_device = stand_in_device();
    let first = HidElement::from_raw_retained(first.0, Some(&first_device)).expect("first");
    let second = HidElement::from_raw_retained(second.0, Some(&second_device)).expect("second");
    let orphan = DeviceLessElement::new();
    let orphan = HidElement::from_raw_retained(orphan.0, None).expect("orphan");

    assert!(matches!(first.attach(&second), Err(HidError::InvalidArgument(_))));
    assert!(matches!(first.attach(&orphan), Err(HidError::InvalidArgument(_))));
    assert!(first.attached().is_empty());
}

#[test]
fn owned_values_take_the_create_reference() {
    let element = DeviceLessElement::new();
    if !element.has_empty_layout() {
        return;
    }
    let base = element.retain_count();
    let device = stand_in_device();

    let raw = unsafe {
        ffi::IOHIDValueCreateWithIntegerValue(ffi::kCFAllocatorDefault, element.0, 7, 0)
    };
    assert!(!raw.is_null());
    assert_eq!(element.retain_count(), base + 1);

    let value = HidValue::from_raw_owned(raw, Some(&device)).expect("value");
    assert_eq!(unsafe { CFGetRetainCount(raw) }, 1);
    assert_eq!(value.timestamp(), 7);

    let value_element = value.element();
    assert_eq!(value_element.as_ptr(), element.0);
    assert_eq!(value_element.device().map(|d| d.as_ptr()), Some(device.as_ptr()));
    drop(value_element);

    let copy = value.clone();
    assert_eq!(unsafe { CFGetRetainCount(raw) }, 2);
    drop(copy);
    drop(value);
    assert_eq!(element.retain_count(), base);
}

#[test]
fn owned_values_reject_null() {
    assert!(HidValue::from_raw_owned(ptr::null(), None).is_none());
}

#[test]
fn queue_timeouts_are_seconds_for_iokit() {
    assert!((queue::queue_timeout_seconds(Duration::from_millis(1500)) - 1.5).abs() < f64::EPSILON);
    assert!(queue::queue_timeout_seconds(Duration::ZERO).abs() < f64::EPSILON);
    assert!(
        (queue::queue_timeout_seconds(Duration::MAX) - f64::from(u32::MAX) / 1000.0).abs()
            < f64::EPSILON
    );
}
