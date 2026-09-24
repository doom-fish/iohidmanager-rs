use core::ffi::c_void;
use core::ptr;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use super::subscription::next_id;
#[allow(clippy::wildcard_imports)]
use super::*;
use crate::ffi_impl as ffi;

type QueueHandler = Box<dyn Fn() + Send + Sync + 'static>;

unsafe extern "C" fn queue_value_trampoline(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    catch_user_panic("queue_value_trampoline", || {
        if result != ffi::kIOReturnSuccess {
            return;
        }
        let _callout = Callout::enter();
        unsafe {
            CallbackContext::<QueueHandler>::with(context, "queue_value_trampoline", |callback| {
                callback();
            });
        }
    });
}

#[derive(Debug, Default)]
struct QueueSchedule {
    claimed: Option<u64>,
    registered: Option<u64>,
    scheduled: bool,
}

fn lock(schedule: &Mutex<QueueSchedule>) -> MutexGuard<'_, QueueSchedule> {
    schedule.lock().unwrap_or_else(PoisonError::into_inner)
}

fn release_claim(schedule: &Mutex<QueueSchedule>, id: u64) {
    let mut schedule = lock(schedule);
    if schedule.claimed == Some(id) {
        schedule.claimed = None;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// Wraps the option bits consumed by `IOHIDQueueCreate`.
pub struct HidQueueOptions(u32);

impl HidQueueOptions {
    /// Mirrors `kIOHIDQueueOptionsTypeNone`.
    pub const NONE: Self = Self(ffi::kIOHIDQueueOptionsTypeNone);
    /// Mirrors `kIOHIDQueueOptionsTypeEnqueueAll`.
    pub const ENQUEUE_ALL: Self = Self(ffi::kIOHIDQueueOptionsTypeEnqueueAll);

    #[must_use]
    /// Returns the raw option bits passed to `IOHIDQueueCreate`.
    pub const fn bits(self) -> u32 {
        self.0
    }
}

/// Owns an `IOHIDQueueRef` returned by `IOHIDQueueCreate` and the device it reads from.
pub struct HidQueue {
    raw: ffi::IOHIDQueueRef,
    device: HidDevice,
    schedule: Arc<Mutex<QueueSchedule>>,
}

impl Drop for HidQueue {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            unsafe { ffi::CFRelease(self.raw.cast_const()) };
            self.raw = ptr::null_mut();
        }
    }
}

/// Owns a registration from `IOHIDQueueRegisterValueAvailableCallback`.
pub struct QueueValueAvailableSubscription {
    id: u64,
    schedule: Arc<Mutex<QueueSchedule>>,
    _registration: Registration<QueueHandler>,
}

impl Drop for QueueValueAvailableSubscription {
    fn drop(&mut self) {
        release_claim(&self.schedule, self.id);
    }
}

fn register_queue(
    queue: Retained,
    schedule: Arc<Mutex<QueueSchedule>>,
    id: u64,
    foreign: ForeignRef,
) -> Teardown {
    let raw = queue.get().cast_mut();
    {
        let mut state = lock(&schedule);
        unsafe {
            ffi::IOHIDQueueRegisterValueAvailableCallback(
                raw,
                Some(queue_value_trampoline),
                foreign.as_ptr(),
            );
        }
        state.registered = Some(id);
        if !state.scheduled {
            unsafe {
                ffi::IOHIDQueueScheduleWithRunLoop(
                    raw,
                    ffi::CFRunLoopGetCurrent(),
                    ffi::kCFRunLoopDefaultMode,
                );
            }
            state.scheduled = true;
        }
    }
    Box::new(move || {
        let raw = queue.get().cast_mut();
        let mut state = lock(&schedule);
        if state.registered == Some(id) {
            unsafe {
                ffi::IOHIDQueueRegisterValueAvailableCallback(
                    raw,
                    Some(queue_value_trampoline),
                    ptr::null_mut(),
                );
            }
            state.registered = None;
        }
        if state.scheduled && state.registered.is_none() && state.claimed.is_none() {
            unsafe {
                ffi::IOHIDQueueUnscheduleFromRunLoop(
                    raw,
                    ffi::CFRunLoopGetCurrent(),
                    ffi::kCFRunLoopDefaultMode,
                );
            }
            state.scheduled = false;
        }
        drop(state);
        drop(queue);
        drop(foreign);
    })
}

#[allow(clippy::missing_errors_doc)]
impl HidQueue {
    #[must_use]
    /// Wraps `IOHIDQueueGetTypeID`.
    pub fn type_id() -> ffi::CFTypeID {
        unsafe { ffi::IOHIDQueueGetTypeID() }
    }

    /// Wraps `IOHIDQueueCreate` with `kIOHIDQueueOptionsTypeNone`.
    pub fn new(device: &HidDevice, depth: usize) -> Result<Self, HidError> {
        Self::with_options(device, depth, HidQueueOptions::NONE)
    }

    /// Wraps `IOHIDQueueCreate`.
    pub fn with_options(
        device: &HidDevice,
        depth: usize,
        options: HidQueueOptions,
    ) -> Result<Self, HidError> {
        let depth = ffi::CFIndex::try_from(depth)
            .map_err(|_| HidError::InvalidArgument("depth does not fit CFIndex".to_owned()))?;
        let raw = unsafe {
            ffi::IOHIDQueueCreate(ffi::kCFAllocatorDefault, device.raw, depth, options.bits())
        };
        if raw.is_null() {
            Err(HidError::OperationFailed("IOHIDQueueCreate"))
        } else {
            Ok(Self {
                raw,
                device: device.clone(),
                schedule: Arc::default(),
            })
        }
    }

    #[must_use]
    /// Returns the device the queue was created for (`IOHIDQueueGetDevice`).
    pub fn device(&self) -> Option<HidDevice> {
        Some(self.device.clone())
    }

    #[must_use]
    /// Wraps `IOHIDQueueGetDepth`.
    pub fn depth(&self) -> usize {
        usize::try_from(unsafe { ffi::IOHIDQueueGetDepth(self.raw) }).unwrap_or(0)
    }

    /// Wraps `IOHIDQueueSetDepth`.
    pub fn set_depth(&self, depth: usize) -> Result<(), HidError> {
        let depth = ffi::CFIndex::try_from(depth)
            .map_err(|_| HidError::InvalidArgument("depth does not fit CFIndex".to_owned()))?;
        unsafe { ffi::IOHIDQueueSetDepth(self.raw, depth) };
        Ok(())
    }

    /// Wraps `IOHIDQueueAddElement`.
    pub fn add_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueAddElement(self.raw, element.raw) };
    }

    /// Wraps `IOHIDQueueRemoveElement`.
    pub fn remove_element(&self, element: &HidElement) {
        unsafe { ffi::IOHIDQueueRemoveElement(self.raw, element.raw) };
    }

    #[must_use]
    /// Wraps `IOHIDQueueContainsElement`.
    pub fn contains_element(&self, element: &HidElement) -> bool {
        unsafe { ffi::IOHIDQueueContainsElement(self.raw, element.raw) }
    }

    /// Wraps `IOHIDQueueStart`.
    pub fn start(&self) {
        unsafe { ffi::IOHIDQueueStart(self.raw) };
    }

    /// Wraps `IOHIDQueueStop`.
    pub fn stop(&self) {
        unsafe { ffi::IOHIDQueueStop(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDQueueActivate`.
    pub unsafe fn activate(&self) {
        unsafe { ffi::IOHIDQueueActivate(self.raw) };
    }

    #[allow(clippy::missing_safety_doc)]
    /// Wraps `IOHIDQueueCancel`.
    pub unsafe fn cancel(&self) {
        unsafe { ffi::IOHIDQueueCancel(self.raw) };
    }

    #[must_use]
    /// Wraps `IOHIDQueueCopyNextValue`.
    pub fn copy_next_value(&self) -> Option<HidValue> {
        HidValue::from_raw_owned(
            unsafe { ffi::IOHIDQueueCopyNextValue(self.raw) },
            Some(&self.device),
        )
    }

    /// Wraps `IOHIDQueueRegisterValueAvailableCallback`.
    pub fn on_value_available<F>(
        &self,
        callback: F,
    ) -> Result<QueueValueAvailableSubscription, HidError>
    where
        F: Fn() + Send + Sync + 'static,
    {
        let id = next_id();
        {
            let mut schedule = lock(&self.schedule);
            if schedule.claimed.is_some() {
                return Err(HidError::InvalidArgument(
                    "the queue already has a value-available subscription".to_owned(),
                ));
            }
            schedule.claimed = Some(id);
        }
        let handler: QueueHandler = Box::new(callback);
        let context = CallbackContext::new(handler);
        let foreign = ForeignRef::new(&context);
        let queue = Retained::retain(self.raw.cast_const());
        let schedule = Arc::clone(&self.schedule);
        match event_loop::run_sync(move || register_queue(queue, schedule, id, foreign)) {
            Ok(teardown) => Ok(QueueValueAvailableSubscription {
                id,
                schedule: Arc::clone(&self.schedule),
                _registration: Registration::new(context, teardown),
            }),
            Err(error) => {
                release_claim(&self.schedule, id);
                Err(error)
            }
        }
    }

    #[must_use]
    /// Mirrors `IOHIDQueueRef`.
    pub const fn as_ptr(&self) -> ffi::IOHIDQueueRef {
        self.raw
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn queue_for_first_device() -> Option<HidQueue> {
        let manager = HidManager::new().expect("manager");
        manager.set_device_matching(None).expect("matching");
        manager
            .live_devices()
            .into_iter()
            .find_map(|device| HidQueue::new(&device, 8).ok())
    }

    fn state(queue: &HidQueue) -> (Option<u64>, Option<u64>, bool) {
        let schedule = lock(&queue.schedule);
        (schedule.claimed, schedule.registered, schedule.scheduled)
    }

    #[test]
    fn a_queue_takes_one_value_subscription_at_a_time() {
        let Some(queue) = queue_for_first_device() else {
            return;
        };
        let first = queue.on_value_available(|| {}).expect("first");
        assert!(matches!(
            queue.on_value_available(|| {}),
            Err(HidError::InvalidArgument(_))
        ));
        assert_eq!(state(&queue), (Some(first.id), Some(first.id), true));
        drop(first);
        event_loop::flush();
        assert_eq!(state(&queue), (None, None, false));
    }

    #[test]
    fn a_new_subscription_survives_the_previous_teardown_in_either_order() {
        let Some(queue) = queue_for_first_device() else {
            return;
        };
        let first = queue.on_value_available(|| {}).expect("first");
        drop(first);
        let second = queue.on_value_available(|| {}).expect("second");
        event_loop::flush();
        assert_eq!(state(&queue), (Some(second.id), Some(second.id), true));

        drop(second);
        event_loop::flush();
        let third = queue.on_value_available(|| {}).expect("third");
        event_loop::flush();
        assert_eq!(state(&queue), (Some(third.id), Some(third.id), true));

        drop(third);
        event_loop::flush();
        assert_eq!(state(&queue), (None, None, false));
    }

    #[test]
    fn a_subscription_outlives_its_queue_handle() {
        let Some(queue) = queue_for_first_device() else {
            return;
        };
        let subscription = queue.on_value_available(|| {}).expect("subscribe");
        let schedule = Arc::clone(&queue.schedule);
        drop(queue);
        drop(subscription);
        event_loop::flush();
        let schedule = lock(&schedule);
        assert_eq!(
            (schedule.claimed, schedule.registered, schedule.scheduled),
            (None, None, false)
        );
    }
}
