use core::ptr;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use apple_cf::raw::CFGetRetainCount;

use super::*;
use crate::ffi_impl as ffi;

struct DropFlag(Arc<AtomicBool>);

impl Drop for DropFlag {
    fn drop(&mut self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

fn manager_matching_all() -> HidManager {
    let manager = HidManager::new().expect("manager");
    manager.set_device_matching(None).expect("matching");
    manager
}

fn wait_for(condition: impl Fn() -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if condition() {
            return true;
        }
        thread::sleep(Duration::from_millis(5));
    }
    condition()
}

fn without_keyboards(devices: Vec<HidDevice>) -> Vec<HidDevice> {
    devices
        .into_iter()
        .filter(|device| {
            !device.conforms_to(HidUsage::Keyboard)
                && !device.conforms_to(HidUsage::Custom(
                    ffi::kHIDPage_GenericDesktop,
                    ffi::kHIDUsage_GD_Keypad,
                ))
        })
        .collect()
}

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
        unsafe {
            ffi::IOHIDElementGetReportSize(self.0) == 0
                && ffi::IOHIDElementGetReportCount(self.0) == 0
        }
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
    assert_eq!(
        attached[0].device().map(|d| d.as_ptr()),
        Some(device.as_ptr())
    );
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

    assert!(matches!(
        first.attach(&second),
        Err(HidError::InvalidArgument(_))
    ));
    assert!(matches!(
        first.attach(&orphan),
        Err(HidError::InvalidArgument(_))
    ));
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

    let raw =
        unsafe { ffi::IOHIDValueCreateWithIntegerValue(ffi::kCFAllocatorDefault, element.0, 7, 0) };
    assert!(!raw.is_null());
    assert_eq!(element.retain_count(), base + 1);

    let value = HidValue::from_raw_owned(raw, Some(&device)).expect("value");
    assert_eq!(unsafe { CFGetRetainCount(raw) }, 1);
    assert_eq!(value.timestamp(), 7);

    let value_element = value.element();
    assert_eq!(value_element.as_ptr(), element.0);
    assert_eq!(
        value_element.device().map(|d| d.as_ptr()),
        Some(device.as_ptr())
    );
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
fn report_buffers_follow_the_device_maximum() {
    assert_eq!(report_buffer_len(Some(8)), 8);
    assert_eq!(report_buffer_len(Some(0)), DEFAULT_REPORT_BUFFER_LEN);
    assert_eq!(report_buffer_len(None), DEFAULT_REPORT_BUFFER_LEN);
    assert_eq!(report_buffer_len(Some(u32::MAX)), MAX_REPORT_BUFFER_LEN);
}

#[test]
fn manager_subscriptions_require_device_matching() {
    let manager = HidManager::new().expect("manager");
    assert!(matches!(
        manager.on_device_matching(|_| {}),
        Err(HidError::InvalidArgument(_))
    ));
    assert!(matches!(
        manager.on_input_value(|_, _| {}),
        Err(HidError::InvalidArgument(_))
    ));
    manager.set_device_matching(None).expect("matching");
    drop(manager.on_device_removal(|_| {}).expect("removal"));
    event_loop::flush();
}

#[test]
fn subscriptions_on_one_manager_do_not_replace_each_other() {
    let manager = manager_matching_all();
    if manager.devices().is_empty() {
        return;
    }
    let counters: Vec<Arc<AtomicUsize>> = (0..3).map(|_| Arc::new(AtomicUsize::new(0))).collect();
    let subscribe = |counter: &Arc<AtomicUsize>| {
        let counter = Arc::clone(counter);
        manager
            .on_device_matching(move |_| {
                counter.fetch_add(1, Ordering::SeqCst);
            })
            .expect("subscribe")
    };
    let first = subscribe(&counters[0]);
    let second = subscribe(&counters[1]);
    assert!(wait_for(|| {
        counters[0].load(Ordering::SeqCst) > 0 && counters[1].load(Ordering::SeqCst) > 0
    }));
    drop(first);
    let third = subscribe(&counters[2]);
    assert!(wait_for(|| counters[2].load(Ordering::SeqCst) > 0));
    drop((second, third));
    event_loop::flush();
}

#[test]
fn a_subscription_dropped_inside_its_callback_stops_at_once() {
    let manager = manager_matching_all();
    if manager.devices().is_empty() {
        return;
    }
    let calls = Arc::new(AtomicUsize::new(0));
    let released = Arc::new(AtomicBool::new(false));
    let slot: Arc<Mutex<Option<ManagerDeviceSubscription>>> = Arc::new(Mutex::new(None));
    let (stored, ready) = mpsc::channel::<()>();
    let ready = Mutex::new(ready);
    let flag = DropFlag(Arc::clone(&released));
    let counter = Arc::clone(&calls);
    let owner = Arc::clone(&slot);
    let subscription = manager
        .on_device_matching(move |_| {
            assert!(!flag.0.load(Ordering::SeqCst));
            counter.fetch_add(1, Ordering::SeqCst);
            let _ = ready.lock().unwrap().recv_timeout(Duration::from_secs(10));
            drop(owner.lock().unwrap().take());
        })
        .expect("subscribe");
    *slot.lock().unwrap() = Some(subscription);
    stored.send(()).expect("stored");
    assert!(wait_for(|| slot.lock().unwrap().is_none()));
    event_loop::flush();
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    assert!(released.load(Ordering::SeqCst));
}

#[test]
fn dropping_on_another_thread_releases_the_callback() {
    let manager = manager_matching_all();
    let released = Arc::new(AtomicBool::new(false));
    let flag = DropFlag(Arc::clone(&released));
    let subscription = manager
        .on_device_removal(move |_| assert!(!flag.0.load(Ordering::SeqCst)))
        .expect("subscribe");
    thread::spawn(move || drop(subscription))
        .join()
        .expect("drop thread");
    event_loop::flush();
    assert!(released.load(Ordering::SeqCst));
}

#[test]
fn subscribing_and_dropping_from_many_threads_is_sound() {
    let workers: Vec<_> = (0..4)
        .map(|_| {
            thread::spawn(|| {
                let manager = manager_matching_all();
                for _ in 0..10 {
                    let hits = Arc::new(AtomicUsize::new(0));
                    let counter = Arc::clone(&hits);
                    let subscription = manager
                        .on_device_matching(move |device| {
                            let _ = device.info();
                            counter.fetch_add(1, Ordering::SeqCst);
                        })
                        .expect("subscribe");
                    drop(subscription);
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().expect("worker");
    }
    event_loop::flush();
}

#[test]
fn dropping_a_device_subscription_leaves_the_callers_device_open() {
    let manager = manager_matching_all();
    for device in manager.live_devices() {
        let Some(element) = device.elements().into_iter().find(|element| {
            matches!(
                element.element_kind(),
                HidElementType::InputMisc | HidElementType::InputButton | HidElementType::InputAxis
            )
        }) else {
            continue;
        };
        if device.get_value(&element).is_err() {
            continue;
        }
        let first = device.on_removal(|| {}).expect("first removal");
        let second = device.on_removal(|| {}).expect("second removal");
        drop(first);
        event_loop::flush();
        assert!(device.get_value(&element).is_ok());
        drop(second);
        event_loop::flush();
        assert!(device.get_value(&element).is_ok());
        return;
    }
}

#[test]
fn value_and_report_subscriptions_use_their_own_device() {
    let manager = manager_matching_all();
    let Some(device) = without_keyboards(manager.live_devices()).into_iter().next() else {
        return;
    };
    let accept = |subscribed: Result<(), HidError>| match subscribed {
        Ok(()) | Err(HidError::DeviceOpenFailed(_)) => {}
        Err(error) => panic!("subscribe failed: {error}"),
    };
    accept(device.on_input_value(&[], |_| {}).map(drop));
    accept(
        device
            .on_input_value(&[ElementMatch::new().with_usage_page(0xff00)], |_| {})
            .map(drop),
    );
    accept(device.on_input_report(|_| {}).map(drop));
    accept(device.on_input_report_with_timestamp(|_| {}).map(drop));
    event_loop::flush();
}

#[allow(clippy::cast_possible_wrap)]
const NOT_OPEN: ffi::IOReturn = 0xe000_02cd_u32 as ffi::IOReturn;

fn first_device_without_keyboards() -> Option<HidDevice> {
    without_keyboards(manager_matching_all().live_devices())
        .into_iter()
        .next()
}

fn private_mode_is_empty(mode: ffi::CFStringRef) -> bool {
    let result = unsafe { apple_cf::raw::CFRunLoopRunInMode(mode, 0.01, 1) };
    u32::try_from(result).ok() == Some(apple_cf::raw::kCFRunLoopRunFinished)
}

#[test]
fn a_device_id_opens_the_same_service_again() {
    let Some(device) = manager_matching_all().live_devices().into_iter().next() else {
        return;
    };
    let id = device.id().expect("id");
    let reopened = HidDevice::from_id(id).expect("reopened");
    assert_ne!(reopened.as_ptr(), device.as_ptr());
    assert_eq!(reopened.id().expect("reopened id"), id);
    assert_eq!(reopened.info(), device.info());
    assert!(matches!(
        HidDevice::from_id(HidDeviceId(u64::MAX)),
        Err(HidError::OperationFailed("IOServiceGetMatchingService"))
    ));
}

#[test]
fn value_info_copies_the_element_identity_and_payload() {
    let Some(element) = manager_matching_all()
        .live_devices()
        .into_iter()
        .find_map(|device| {
            device
                .elements()
                .into_iter()
                .find(|element| element.report_size_bits() == 8 && element.report_count() == 1)
        })
    else {
        return;
    };
    let value = HidValue::from_bytes(&element, 42, &[7]).expect("value");
    assert_eq!(
        value.info(),
        HidValueInfo {
            cookie: element.cookie(),
            usage_page: element.usage_page(),
            usage: element.usage(),
            timestamp: 42,
            integer_value: value.integer_value(),
            bytes: vec![7],
        }
    );
}

#[test]
fn transaction_directions_keep_unknown_values() {
    for (raw, direction) in [
        (0, HidTransactionDirection::Input),
        (1, HidTransactionDirection::Output),
        (7, HidTransactionDirection::Unknown(7)),
    ] {
        assert_eq!(HidTransactionDirection::from_raw(raw), direction);
        assert_eq!(direction.as_raw(), raw);
    }
}

#[test]
fn strings_with_multibyte_characters_round_trip() {
    for text in ["", "plain", "Größe ÷ 2", "手柄 🎮🕹️"] {
        let string = make_cfstring(text).expect("cfstring");
        let copied = cfstring_to_string(string);
        unsafe { ffi::CFRelease(string.cast()) };
        assert_eq!(copied.as_deref(), Some(text));
    }
}

#[test]
fn setters_reject_keys_with_nul_before_calling_iokit() {
    let raw = DeviceLessElement::new();
    let element = HidElement::from_raw_retained(raw.0, None).expect("element");
    let manager = HidManager::new().expect("manager");
    for result in [
        element.set_property_u32("bad\0key", 1),
        element.set_property_string("bad\0key", "value"),
        element.set_property_data("bad\0key", &[1]),
        manager.set_property_u32("bad\0key", 1),
        manager.set_property_string("bad\0key", "value"),
        manager.set_property_data("bad\0key", &[1]),
    ] {
        assert!(matches!(result, Err(HidError::InvalidArgument(_))));
    }
}

#[test]
fn a_private_device_is_scheduled_only_in_the_private_mode_of_this_thread() {
    let Some(device) = first_device_without_keyboards() else {
        return;
    };
    let mode = completion::private_mode().expect("private mode");
    let service = device.registered_service().expect("service");
    let mut private = completion::PrivateDevice::create(service).expect("private device");
    assert_ne!(private.raw(), device.as_ptr());
    assert!(private_mode_is_empty(mode));
    private.schedule(mode);
    assert!(!private_mode_is_empty(mode));
    private.unschedule();
    assert!(private_mode_is_empty(mode));
}

#[test]
fn a_report_on_an_unopened_private_device_is_rejected_before_it_is_sent() {
    let Some(device) = first_device_without_keyboards() else {
        return;
    };
    let mode = completion::private_mode().expect("private mode");
    let service = device.registered_service().expect("service");
    let mut private = completion::PrivateDevice::create(service).expect("private device");
    private.schedule(mode);
    let context = CallbackContext::new(completion::Completion::default());
    let foreign = ForeignRef::new(&context);
    let sent = unsafe {
        device::send_report_with_callback(
            private.raw(),
            HidReportType::Output,
            0,
            vec![0],
            1000.0,
            foreign.as_ptr(),
        )
    };
    private.unschedule();
    drop(private);
    assert_eq!(
        sent,
        Err(HidError::IoReturn(
            "IOHIDDeviceSetReportWithCallback",
            NOT_OPEN
        ))
    );
    assert_eq!(context.get().status(), None);
}

#[test]
fn commit_with_timeout_rejects_input_transactions_and_bad_timeouts_up_front() {
    let Some(device) = first_device_without_keyboards() else {
        return;
    };
    let input = HidTransaction::new(&device, HidTransactionDirection::Input).expect("input");
    assert!(matches!(
        input.commit_with_timeout(Duration::from_secs(1)),
        Err(HidError::InvalidArgument(_))
    ));
    let output = HidTransaction::new(&device, HidTransactionDirection::Output).expect("output");
    for timeout in [Duration::from_millis(10), Duration::from_secs(3600)] {
        assert!(matches!(
            output.commit_with_timeout(timeout),
            Err(HidError::InvalidArgument(_))
        ));
    }
}

#[test]
fn an_empty_output_commit_fails_on_the_private_device_without_waiting() {
    let Some(device) = first_device_without_keyboards() else {
        return;
    };
    let transaction =
        HidTransaction::new(&device, HidTransactionDirection::Output).expect("transaction");
    let started = Instant::now();
    match transaction.commit_with_timeout(Duration::from_secs(2)) {
        Err(HidError::IoReturn("IOHIDTransactionCommitWithCallback", _)) => {
            assert!(started.elapsed() < Duration::from_secs(2));
        }
        Err(HidError::DeviceOpenFailed(_) | HidError::Unsupported(_)) => {}
        other => panic!("unexpected commit result: {other:?}"),
    }
    let mode = completion::private_mode().expect("private mode");
    assert!(private_mode_is_empty(mode));
}

#[test]
fn the_commit_mirror_copies_elements_and_values_onto_the_private_device() {
    let found = without_keyboards(manager_matching_all().live_devices())
        .into_iter()
        .find_map(|device| {
            let element = device.elements().into_iter().find(|element| {
                matches!(
                    element.element_kind(),
                    HidElementType::Output | HidElementType::Feature
                ) && element.report_size_bits() > 0
            })?;
            Some((device, element))
        });
    let Some((device, element)) = found else {
        return;
    };
    let transaction =
        HidTransaction::new(&device, HidTransactionDirection::Output).expect("transaction");
    transaction.add_element(&element);
    let value = HidValue::from_integer(&element, 5, 1).expect("value");
    transaction.set_value(&element, &value, 0);

    let service = device.registered_service().expect("service");
    let private = completion::PrivateDevice::create(service).expect("private device");
    let mirror = transaction.mirror_onto(private.raw()).expect("mirror");
    let targets = elements_from_array(
        unsafe {
            ffi::IOHIDDeviceCopyMatchingElements(
                private.raw(),
                ptr::null(),
                ffi::kIOHIDOptionsTypeNone,
            )
        },
        true,
        None,
    );
    let target = targets
        .iter()
        .find(|target| target.cookie() == element.cookie())
        .expect("target element");
    assert_ne!(target.as_ptr(), element.as_ptr());
    assert_eq!(
        unsafe { ffi::IOHIDElementGetDevice(target.as_ptr()) },
        private.raw()
    );
    let mirrored = mirror.get().cast_mut();
    assert!(unsafe { ffi::IOHIDTransactionContainsElement(mirrored, target.as_ptr()) });
    let copy = unsafe { ffi::IOHIDTransactionGetValue(mirrored, target.as_ptr(), 0) };
    assert!(!copy.is_null());
    assert_eq!(unsafe { ffi::IOHIDValueGetElement(copy) }, target.as_ptr());
    assert_eq!(unsafe { ffi::IOHIDValueGetIntegerValue(copy) }, 1);
    assert_eq!(unsafe { ffi::IOHIDValueGetTimeStamp(copy) }, 5);
    drop(mirror);
    drop(targets);
    drop(private);
}
