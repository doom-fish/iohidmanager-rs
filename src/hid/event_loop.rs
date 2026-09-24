use core::ffi::c_void;
use core::ptr;
use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::{mpsc, Mutex, MutexGuard, OnceLock, PoisonError};
use std::thread::{self, ThreadId};

use apple_cf::raw::{
    kCFRunLoopDefaultMode, CFIndex, CFRunLoopAddSource, CFRunLoopGetCurrent, CFRunLoopRef,
    CFRunLoopRun, CFRunLoopSourceContext, CFRunLoopSourceCreate, CFRunLoopSourceRef,
    CFRunLoopSourceSignal, CFRunLoopWakeUp,
};
use doom_fish_utils::panic_safe::catch_user_panic;

use crate::error::HidError;

pub type Job = Box<dyn FnOnce() + Send>;

struct JobQueue {
    source: usize,
    jobs: Mutex<VecDeque<Job>>,
}

struct EventLoop {
    run_loop: usize,
    thread: ThreadId,
    jobs: JobQueue,
    teardowns: JobQueue,
}

static EVENT_LOOP: OnceLock<Option<EventLoop>> = OnceLock::new();

thread_local! {
    static CALLOUT_DEPTH: Cell<usize> = const { Cell::new(0) };
}

impl JobQueue {
    fn new(source: usize) -> Self {
        Self {
            source,
            jobs: Mutex::new(VecDeque::new()),
        }
    }

    fn lock(&self) -> MutexGuard<'_, VecDeque<Job>> {
        self.jobs.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn run_pending(&self) {
        while !in_callout() {
            let Some(job) = self.lock().pop_front() else {
                break;
            };
            catch_user_panic("iohidmanager event loop job", job);
        }
    }
}

impl EventLoop {
    fn signal(&self, queue: &JobQueue) {
        unsafe {
            CFRunLoopSourceSignal(queue.source as CFRunLoopSourceRef);
            CFRunLoopWakeUp(self.run_loop as CFRunLoopRef);
        }
    }

    fn push(&self, queue: &JobQueue, job: Job) {
        queue.lock().push_back(job);
        self.signal(queue);
    }
}

unsafe fn add_source(
    run_loop: CFRunLoopRef,
    order: CFIndex,
    perform: unsafe extern "C" fn(*mut c_void),
) -> Option<usize> {
    let mut context = CFRunLoopSourceContext {
        version: 0,
        info: ptr::null_mut(),
        retain: None,
        release: None,
        copyDescription: None,
        equal: None,
        hash: None,
        schedule: None,
        cancel: None,
        perform: Some(perform),
    };
    let source = unsafe { CFRunLoopSourceCreate(ptr::null(), order, &raw mut context) };
    if source.is_null() {
        return None;
    }
    unsafe { CFRunLoopAddSource(run_loop, source, kCFRunLoopDefaultMode) };
    Some(source as usize)
}

fn start() -> Option<EventLoop> {
    let (ready, started) = mpsc::channel::<(usize, usize, usize)>();
    let handle = thread::Builder::new()
        .name("iohidmanager-event-loop".to_owned())
        .spawn(move || {
            let run_loop = unsafe { CFRunLoopGetCurrent() };
            let sources = unsafe {
                add_source(run_loop, 0, perform_jobs).zip(add_source(
                    run_loop,
                    CFIndex::MAX,
                    perform_teardowns,
                ))
            };
            let Some((jobs, teardowns)) = sources else {
                return;
            };
            if ready.send((run_loop as usize, jobs, teardowns)).is_err() {
                return;
            }
            loop {
                unsafe { CFRunLoopRun() };
            }
        })
        .ok()?;
    let (run_loop, jobs, teardowns) = started.recv().ok()?;
    Some(EventLoop {
        run_loop,
        thread: handle.thread().id(),
        jobs: JobQueue::new(jobs),
        teardowns: JobQueue::new(teardowns),
    })
}

fn event_loop() -> Result<&'static EventLoop, HidError> {
    EVENT_LOOP
        .get_or_init(start)
        .as_ref()
        .ok_or(HidError::OperationFailed(
            "starting the iohidmanager event loop",
        ))
}

fn started_event_loop() -> Option<&'static EventLoop> {
    EVENT_LOOP.get().and_then(Option::as_ref)
}

unsafe extern "C" fn perform_jobs(_info: *mut c_void) {
    catch_user_panic("iohidmanager event loop", || {
        if let Some(event_loop) = started_event_loop() {
            event_loop.jobs.run_pending();
        }
    });
}

unsafe extern "C" fn perform_teardowns(_info: *mut c_void) {
    catch_user_panic("iohidmanager event loop teardown", || {
        if let Some(event_loop) = started_event_loop() {
            event_loop.teardowns.run_pending();
        }
    });
}

pub struct Callout(());

impl Callout {
    pub fn enter() -> Self {
        CALLOUT_DEPTH.with(|depth| depth.set(depth.get() + 1));
        Self(())
    }
}

impl Drop for Callout {
    fn drop(&mut self) {
        let depth = CALLOUT_DEPTH.with(|depth| {
            let remaining = depth.get().saturating_sub(1);
            depth.set(remaining);
            remaining
        });
        if depth > 0 || !is_event_loop_thread() {
            return;
        }
        if let Some(event_loop) = started_event_loop() {
            for queue in [&event_loop.jobs, &event_loop.teardowns] {
                if !queue.lock().is_empty() {
                    event_loop.signal(queue);
                }
            }
        }
    }
}

fn in_callout() -> bool {
    CALLOUT_DEPTH.with(Cell::get) > 0
}

fn is_event_loop_thread() -> bool {
    started_event_loop().is_some_and(|event_loop| event_loop.thread == thread::current().id())
}

pub fn run_sync<R, F>(job: F) -> Result<R, HidError>
where
    R: Send + 'static,
    F: FnOnce() -> R + Send + 'static,
{
    let event_loop = event_loop()?;
    if is_event_loop_thread() {
        return Ok(job());
    }
    let (done, result) = mpsc::sync_channel(1);
    event_loop.push(
        &event_loop.jobs,
        Box::new(move || {
            let _ = done.send(job());
        }),
    );
    result
        .recv()
        .map_err(|_| HidError::OperationFailed("iohidmanager event loop job"))
}

pub fn defer(job: Job) {
    if let Some(event_loop) = started_event_loop() {
        event_loop.push(&event_loop.teardowns, job);
    }
}

#[cfg(test)]
pub fn flush() {
    run_sync(|| ()).expect("event loop");
    let (done, finished) = mpsc::channel();
    defer(Box::new(move || {
        let _ = done.send(());
    }));
    finished.recv().expect("teardown queue");
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    use super::*;

    #[test]
    fn run_sync_executes_on_the_event_loop_thread() {
        let caller = thread::current().id();
        let (on_loop, loop_thread) =
            run_sync(|| (is_event_loop_thread(), thread::current().id())).expect("run_sync");
        assert!(on_loop);
        assert_ne!(loop_thread, caller);
        assert!(!is_event_loop_thread());
        assert!(started_event_loop().is_some());
    }

    #[test]
    fn nested_run_sync_runs_inline() {
        let value = run_sync(|| run_sync(|| 41).expect("inline") + 1).expect("outer");
        assert_eq!(value, 42);
    }

    #[test]
    fn deferred_jobs_wait_until_callouts_return() {
        let order = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&order);
        run_sync(move || {
            let callout = Callout::enter();
            let deferred = Arc::clone(&log);
            defer(Box::new(move || deferred.lock().unwrap().push("deferred")));
            if let Some(event_loop) = started_event_loop() {
                event_loop.teardowns.run_pending();
                event_loop.jobs.run_pending();
            }
            log.lock().unwrap().push("callout");
            drop(callout);
        })
        .expect("run_sync");
        flush();
        assert_eq!(*order.lock().unwrap(), vec!["callout", "deferred"]);
    }

    unsafe extern "C" fn record_source(info: *mut c_void) {
        let log = unsafe { &*info.cast::<Mutex<Vec<&'static str>>>() };
        log.lock().unwrap().push("source");
    }

    #[test]
    fn teardowns_run_after_sources_signalled_before_them() {
        let order: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
        let info = Arc::as_ptr(&order).cast_mut().cast::<c_void>() as usize;
        let log = Arc::clone(&order);
        let source = run_sync(move || unsafe {
            let mut context = CFRunLoopSourceContext {
                version: 0,
                info: info as *mut c_void,
                retain: None,
                release: None,
                copyDescription: None,
                equal: None,
                hash: None,
                schedule: None,
                cancel: None,
                perform: Some(record_source),
            };
            let source = CFRunLoopSourceCreate(ptr::null(), 0, &raw mut context);
            assert!(!source.is_null());
            CFRunLoopAddSource(CFRunLoopGetCurrent(), source, kCFRunLoopDefaultMode);
            defer(Box::new(move || log.lock().unwrap().push("teardown")));
            CFRunLoopSourceSignal(source);
            source as usize
        })
        .expect("run_sync");
        flush();
        run_sync(move || unsafe {
            apple_cf::raw::CFRunLoopSourceInvalidate(source as CFRunLoopSourceRef);
            apple_cf::raw::CFRelease(source as apple_cf::raw::CFTypeRef);
        })
        .expect("cleanup");
        assert_eq!(*order.lock().unwrap(), vec!["source", "teardown"]);
    }

    #[test]
    fn defer_never_waits_for_the_job() {
        run_sync(|| ()).expect("start");
        let (release, gate) = mpsc::channel::<()>();
        let hits = Arc::new(AtomicUsize::new(0));
        let counter = Arc::clone(&hits);
        defer(Box::new(move || {
            let _ = gate.recv();
            counter.fetch_add(1, Ordering::SeqCst);
        }));
        assert_eq!(hits.load(Ordering::SeqCst), 0);
        release.send(()).expect("release");
        flush();
        assert_eq!(hits.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_panicking_job_does_not_stop_the_loop() {
        assert!(run_sync(|| -> u8 { panic!("job panic") }).is_err());
        assert_eq!(run_sync(|| 7).expect("loop still runs"), 7);
    }

    #[test]
    fn jobs_from_many_threads_all_run() {
        let hits = Arc::new(AtomicUsize::new(0));
        let workers: Vec<_> = (0..8)
            .map(|_| {
                let hits = Arc::clone(&hits);
                thread::spawn(move || {
                    for _ in 0..50 {
                        let hits = Arc::clone(&hits);
                        run_sync(move || {
                            hits.fetch_add(1, Ordering::SeqCst);
                        })
                        .expect("run_sync");
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().expect("worker");
        }
        assert_eq!(hits.load(Ordering::SeqCst), 400);
    }
}
