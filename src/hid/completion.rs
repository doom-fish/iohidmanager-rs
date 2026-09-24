use core::ffi::{c_char, c_int, c_void};
use core::ptr;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};
use std::time::{Duration, Instant};

use apple_cf::raw::{kCFRunLoopRunFinished, CFRunLoopRunInMode};
use doom_fish_utils::callback_context::CallbackContext;
use doom_fish_utils::panic_safe::catch_user_panic;

use super::subscription::{ForeignRef, Retained};
use super::{make_cfstring, DEVICE_MAX_ASYNC_REQUEST_TIMEOUT, DEVICE_MIN_ASYNC_REQUEST_TIMEOUT};
use crate::error::HidError;
use crate::ffi_impl as ffi;

const FIRST_DARWIN_RELEASE_DELIVERING_COMPLETIONS: u32 = 22;

extern "C" {
    fn sysctlbyname(
        name: *const c_char,
        oldp: *mut c_void,
        oldlenp: *mut usize,
        newp: *mut c_void,
        newlen: usize,
    ) -> c_int;
}

fn darwin_release_major(release: &[u8]) -> Option<u32> {
    let digits = release.split(|byte| !byte.is_ascii_digit()).next()?;
    core::str::from_utf8(digits).ok()?.parse().ok()
}

fn running_darwin_major() -> Option<u32> {
    static MAJOR: OnceLock<Option<u32>> = OnceLock::new();
    *MAJOR.get_or_init(|| {
        let mut release = [0_u8; 64];
        let mut length = release.len();
        let status = unsafe {
            sysctlbyname(
                c"kern.osrelease".as_ptr(),
                release.as_mut_ptr().cast(),
                &raw mut length,
                ptr::null_mut(),
                0,
            )
        };
        if status != 0 {
            return None;
        }
        darwin_release_major(release.get(..length)?)
    })
}

fn delivers_completions(darwin_major: Option<u32>) -> bool {
    darwin_major.is_some_and(|major| major >= FIRST_DARWIN_RELEASE_DELIVERING_COMPLETIONS)
}

pub fn request_timeout_ms(timeout: Duration) -> Result<f64, HidError> {
    let millis = u32::try_from(timeout.as_millis())
        .ok()
        .filter(|millis| {
            (DEVICE_MIN_ASYNC_REQUEST_TIMEOUT..=DEVICE_MAX_ASYNC_REQUEST_TIMEOUT)
                .contains(&u64::from(*millis))
        })
        .ok_or_else(|| {
            HidError::InvalidArgument(format!(
                "timeout must be between {DEVICE_MIN_ASYNC_REQUEST_TIMEOUT} ms and \
                 {DEVICE_MAX_ASYNC_REQUEST_TIMEOUT} ms"
            ))
        })?;
    Ok(f64::from(millis))
}

#[derive(Default)]
pub struct Completion(Mutex<Option<ffi::IOReturn>>);

impl Completion {
    fn lock(&self) -> MutexGuard<'_, Option<ffi::IOReturn>> {
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn finish(&self, status: ffi::IOReturn) {
        self.lock().get_or_insert(status);
    }

    pub fn status(&self) -> Option<ffi::IOReturn> {
        *self.lock()
    }
}

pub unsafe extern "C" fn report_completed(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
    _report_type: ffi::IOHIDReportType,
    _report_id: u32,
    _report: *mut u8,
    _report_length: ffi::CFIndex,
) {
    catch_user_panic("report_completed", || unsafe {
        CallbackContext::<Completion>::with(context, "report_completed", |completion| {
            completion.finish(result);
        });
    });
}

pub unsafe extern "C" fn commit_completed(
    context: *mut c_void,
    result: ffi::IOReturn,
    _sender: *mut c_void,
) {
    catch_user_panic("commit_completed", || unsafe {
        CallbackContext::<Completion>::with(context, "commit_completed", |completion| {
            completion.finish(result);
        });
    });
}

pub fn private_mode() -> Result<ffi::CFStringRef, HidError> {
    static MODE: OnceLock<usize> = OnceLock::new();
    let mode = *MODE
        .get_or_init(|| make_cfstring("iohidmanager.completion").map_or(0, |mode| mode as usize));
    if mode == 0 {
        Err(HidError::OperationFailed("CFStringCreateWithCString"))
    } else {
        Ok(mode as ffi::CFStringRef)
    }
}

pub struct PrivateDevice {
    device: Retained,
    opened: bool,
    scheduled: Option<(ffi::CFRunLoopRef, ffi::CFStringRef)>,
}

impl PrivateDevice {
    pub fn create(service: ffi::io_service_t) -> Result<Self, HidError> {
        let device = Retained::from_create(unsafe {
            ffi::IOHIDDeviceCreate(ffi::kCFAllocatorDefault, service)
        })
        .ok_or(HidError::OperationFailed("IOHIDDeviceCreate"))?;
        Ok(Self {
            device,
            opened: false,
            scheduled: None,
        })
    }

    fn open(&mut self) -> Result<(), HidError> {
        let status = unsafe { ffi::IOHIDDeviceOpen(self.raw(), ffi::kIOHIDOptionsTypeNone) };
        if status != ffi::kIOReturnSuccess {
            return Err(HidError::DeviceOpenFailed(status));
        }
        self.opened = true;
        Ok(())
    }

    pub fn schedule(&mut self, mode: ffi::CFStringRef) {
        self.unschedule();
        let run_loop = unsafe { ffi::CFRunLoopGetCurrent() };
        unsafe { ffi::IOHIDDeviceScheduleWithRunLoop(self.raw(), run_loop, mode) };
        self.scheduled = Some((run_loop, mode));
    }

    pub fn unschedule(&mut self) {
        if let Some((run_loop, mode)) = self.scheduled.take() {
            unsafe { ffi::IOHIDDeviceUnscheduleFromRunLoop(self.raw(), run_loop, mode) };
        }
    }

    pub const fn raw(&self) -> ffi::IOHIDDeviceRef {
        self.device.get()
    }
}

impl Drop for PrivateDevice {
    fn drop(&mut self) {
        self.unschedule();
        if self.opened {
            unsafe {
                let _ = ffi::IOHIDDeviceClose(self.raw(), ffi::kIOHIDOptionsTypeNone);
            }
        }
    }
}

fn run_until_complete(
    completion: &Completion,
    deadline: Instant,
    mode: ffi::CFStringRef,
) -> Option<ffi::IOReturn> {
    loop {
        if let Some(status) = completion.status() {
            return Some(status);
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return None;
        }
        let result = unsafe { CFRunLoopRunInMode(mode, remaining.as_secs_f64(), 1) };
        if u32::try_from(result).ok() == Some(kCFRunLoopRunFinished) {
            return completion.status();
        }
    }
}

const fn completion_result(
    operation: &'static str,
    status: Option<ffi::IOReturn>,
) -> Result<(), HidError> {
    match status {
        Some(ffi::kIOReturnSuccess) => Ok(()),
        None | Some(ffi::kIOReturnTimeout) => Err(HidError::Timeout(operation)),
        Some(status) => Err(HidError::IoReturn(operation, status)),
    }
}

pub fn complete_on_private_device<P>(
    operation: &'static str,
    service: ffi::io_service_t,
    timeout: Duration,
    issue: impl FnOnce(ffi::IOHIDDeviceRef, *mut c_void) -> Result<P, HidError>,
) -> Result<(), HidError> {
    if !delivers_completions(running_darwin_major()) {
        return Err(HidError::Unsupported(operation));
    }
    let mode = private_mode()?;
    let deadline = Instant::now()
        .checked_add(timeout)
        .ok_or_else(|| HidError::InvalidArgument("timeout is too long".to_owned()))?;
    let completion = CallbackContext::new(Completion::default());
    let foreign = ForeignRef::new(&completion);
    let mut device = PrivateDevice::create(service)?;
    device.open()?;
    device.schedule(mode);
    let pending = issue(device.raw(), foreign.as_ptr())?;
    let status = run_until_complete(completion.get(), deadline, mode);
    device.unschedule();
    drop(device);
    drop(pending);
    drop(foreign);
    completion_result(operation, status)
}

#[cfg(test)]
mod tests {
    use std::thread;

    use apple_cf::raw::{
        CFRelease, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRef, CFRunLoopSourceContext,
        CFRunLoopSourceCreate, CFRunLoopSourceInvalidate, CFRunLoopSourceRef,
        CFRunLoopSourceSignal, CFRunLoopWakeUp,
    };

    use super::*;

    #[allow(clippy::cast_possible_wrap)]
    const NOT_OPEN: ffi::IOReturn = 0xe000_02cd_u32 as ffi::IOReturn;

    unsafe extern "C" fn complete_successfully(info: *mut c_void) {
        unsafe {
            report_completed(
                info,
                ffi::kIOReturnSuccess,
                ptr::null_mut(),
                0,
                0,
                ptr::null_mut(),
                0,
            );
        }
    }

    struct PrivateModeSource(CFRunLoopSourceRef);

    impl PrivateModeSource {
        fn add(info: *mut c_void) -> Self {
            let mut context = CFRunLoopSourceContext {
                version: 0,
                info,
                retain: None,
                release: None,
                copyDescription: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(complete_successfully),
            };
            let source = unsafe { CFRunLoopSourceCreate(ptr::null(), 0, &raw mut context) };
            assert!(!source.is_null());
            let mode = private_mode().expect("private mode");
            unsafe { CFRunLoopAddSource(CFRunLoopGetCurrent(), source, mode) };
            Self(source)
        }
    }

    impl Drop for PrivateModeSource {
        fn drop(&mut self) {
            unsafe {
                CFRunLoopSourceInvalidate(self.0);
                CFRelease(self.0.cast());
            }
        }
    }

    #[test]
    fn darwin_releases_parse_to_their_major_version() {
        assert_eq!(darwin_release_major(b"22.1.0"), Some(22));
        assert_eq!(darwin_release_major(b"27.0.0\0"), Some(27));
        assert_eq!(darwin_release_major(b"9.8.0"), Some(9));
        assert_eq!(darwin_release_major(b""), None);
        assert_eq!(darwin_release_major(b"beta"), None);
    }

    #[test]
    fn completions_need_darwin_22_or_later() {
        assert!(!delivers_completions(None));
        assert!(!delivers_completions(Some(19)));
        assert!(!delivers_completions(Some(21)));
        assert!(delivers_completions(Some(22)));
        assert!(delivers_completions(Some(27)));
        assert!(running_darwin_major().is_some());
    }

    #[test]
    fn request_timeouts_must_fit_the_iokit_range() {
        for rejected in [
            Duration::ZERO,
            Duration::from_millis(49),
            Duration::from_millis(1_200_001),
            Duration::MAX,
        ] {
            assert!(matches!(
                request_timeout_ms(rejected),
                Err(HidError::InvalidArgument(_))
            ));
        }
        let accepted = |timeout, expected: f64| {
            request_timeout_ms(timeout).is_ok_and(|millis| (millis - expected).abs() < f64::EPSILON)
        };
        assert!(accepted(Duration::from_millis(50), 50.0));
        assert!(accepted(Duration::from_micros(1_500_900), 1500.0));
        assert!(accepted(Duration::from_secs(1200), 1_200_000.0));
    }

    #[test]
    fn completion_statuses_map_to_results() {
        assert_eq!(completion_result("op", Some(ffi::kIOReturnSuccess)), Ok(()));
        assert_eq!(completion_result("op", None), Err(HidError::Timeout("op")));
        assert_eq!(
            completion_result("op", Some(ffi::kIOReturnTimeout)),
            Err(HidError::Timeout("op"))
        );
        assert_eq!(
            completion_result("op", Some(NOT_OPEN)),
            Err(HidError::IoReturn("op", NOT_OPEN))
        );
    }

    #[test]
    fn a_completion_keeps_its_first_status() {
        let completion = Completion::default();
        assert_eq!(completion.status(), None);
        completion.finish(NOT_OPEN);
        completion.finish(ffi::kIOReturnSuccess);
        assert_eq!(completion.status(), Some(NOT_OPEN));
    }

    #[test]
    fn a_completion_in_the_private_mode_ends_the_wait() {
        let completion = CallbackContext::new(Completion::default());
        let foreign = ForeignRef::new(&completion);
        let source = PrivateModeSource::add(foreign.as_ptr());
        let run_loop = unsafe { CFRunLoopGetCurrent() } as usize;
        let source_address = source.0 as usize;
        let signaller = thread::spawn(move || {
            thread::sleep(Duration::from_millis(20));
            unsafe {
                CFRunLoopSourceSignal(source_address as CFRunLoopSourceRef);
                CFRunLoopWakeUp(run_loop as CFRunLoopRef);
            }
        });
        let started = Instant::now();
        let mode = private_mode().expect("private mode");
        let status = run_until_complete(completion.get(), started + Duration::from_secs(10), mode);
        signaller.join().expect("signaller");
        assert_eq!(status, Some(ffi::kIOReturnSuccess));
        assert!(started.elapsed() < Duration::from_secs(5));
        drop(source);
    }

    #[test]
    fn the_wait_ends_at_the_deadline_without_a_completion() {
        let completion = CallbackContext::new(Completion::default());
        let foreign = ForeignRef::new(&completion);
        let source = PrivateModeSource::add(foreign.as_ptr());
        let started = Instant::now();
        let mode = private_mode().expect("private mode");
        let status =
            run_until_complete(completion.get(), started + Duration::from_millis(60), mode);
        let elapsed = started.elapsed();
        assert_eq!(status, None);
        assert!(elapsed >= Duration::from_millis(60));
        assert!(elapsed < Duration::from_secs(5));
        drop(source);
    }

    #[test]
    fn an_empty_private_mode_ends_the_wait_at_once() {
        let completion = Completion::default();
        let started = Instant::now();
        let mode = private_mode().expect("private mode");
        assert_eq!(
            run_until_complete(&completion, started + Duration::from_secs(30), mode),
            None
        );
        assert!(started.elapsed() < Duration::from_secs(5));
    }
}
