//! API-surface coverage harness for `iohidmanager`.
//!
//! `IOKit` HID lives in `IOKit.framework/Headers/hid/`. Pure C headers,
//! mirrors the apple-cf / cgevents / imageio C-function-regex pattern.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read_header(name: &str) -> String {
    let p = sdk_root().join(format!(
        "System/Library/Frameworks/IOKit.framework/Headers/hid/{name}.h"
    ));
    std::fs::read_to_string(&p).unwrap_or_else(|e| panic!("read {}: {e}", p.display()))
}

fn extract_c_functions(prefix: &str, source: &str) -> BTreeSet<String> {
    let pattern = format!(r"\b({prefix}[A-Za-z0-9_]+)\s*\(");
    let re = regex_lite::Regex::new(&pattern).unwrap();
    re.captures_iter(source).map(|c| c[1].to_string()).collect()
}

fn extract_rust_externs() -> BTreeSet<String> {
    let p = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/ffi/mod.rs");
    let s = std::fs::read_to_string(&p).unwrap();
    let re = regex_lite::Regex::new(r"pub\s+fn\s+([A-Za-z0-9_]+)\s*\(").unwrap();
    re.captures_iter(&s).map(|c| c[1].to_string()).collect()
}

fn report(name: &str, apple: &BTreeSet<String>, ours: &BTreeSet<String>, omitted: &BTreeSet<String>) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|s| !omitted.contains(*s))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

#[test]
fn iohid_manager_coverage() {
    let header = read_header("IOHIDManager");
    let apple = extract_c_functions("IOHIDManager", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        "IOHIDManagerGetTypeID",
        // Element + value matching — v0.2 (we only filter by usage now).
        "IOHIDManagerSetDeviceMatchingMultiple",
        "IOHIDManagerSetInputValueMatching",
        "IOHIDManagerSetInputValueMatchingMultiple",
        "IOHIDManagerCopyDescriptors",
        "IOHIDManagerRegisterDeviceMatchingCallback",
        "IOHIDManagerRegisterDeviceRemovalCallback",
        "IOHIDManagerRegisterInputReportCallback",
        "IOHIDManagerRegisterInputReportWithTimeStampCallback",
        "IOHIDManagerRegisterInputValueCallback",
        // Run-loop + dispatch-queue scheduling — v0.2.
        "IOHIDManagerScheduleWithRunLoop",
        "IOHIDManagerUnscheduleFromRunLoop",
        "IOHIDManagerSetDispatchQueue",
        "IOHIDManagerSetCancelHandler",
        "IOHIDManagerActivate",
        "IOHIDManagerCancel",
        // Element / device value reads + sets — v0.2.
        "IOHIDManagerCopyEvent",
        "IOHIDManagerCopyEventWithOptions",
        "IOHIDManagerCopyMatchingElements",
        "IOHIDManagerSaveToPropertyDomain",
        "IOHIDManagerSetProperty",
        "IOHIDManagerGetProperty",
    ]);
    report("IOHIDManager", &apple, &ours, &omitted);
}

#[test]
fn iohid_device_coverage() {
    let header = read_header("IOHIDDevice");
    let apple = extract_c_functions("IOHIDDevice", &header);
    let ours = extract_rust_externs();
    let omitted = omitted_set([
        "IOHIDDeviceGetTypeID",
        // Open / close on individual devices — v0.2.
        "IOHIDDeviceOpen",
        "IOHIDDeviceClose",
        // Run-loop + dispatch-queue + activate/cancel — v0.2.
        "IOHIDDeviceScheduleWithRunLoop",
        "IOHIDDeviceUnscheduleFromRunLoop",
        "IOHIDDeviceSetDispatchQueue",
        "IOHIDDeviceSetCancelHandler",
        "IOHIDDeviceActivate",
        "IOHIDDeviceCancel",
        // Per-device callbacks + element queries — v0.2.
        "IOHIDDeviceRegisterRemovalCallback",
        "IOHIDDeviceRegisterInputValueCallback",
        "IOHIDDeviceRegisterInputReportCallback",
        "IOHIDDeviceRegisterInputReportWithTimeStampCallback",
        "IOHIDDeviceCopyMatchingElements",
        "IOHIDDeviceSetInputValueMatching",
        "IOHIDDeviceSetInputValueMatchingMultiple",
        "IOHIDDeviceCopyDescriptor",
        "IOHIDDeviceCopyEvent",
        "IOHIDDeviceCopyEventWithOptions",
        // Get/Set value + report — v0.2.
        "IOHIDDeviceGetValue",
        "IOHIDDeviceGetValueWithOptions",
        "IOHIDDeviceGetValueWithCallback",
        "IOHIDDeviceCopyValueMultiple",
        "IOHIDDeviceCopyValueMultipleWithCallback",
        "IOHIDDeviceSetValue",
        "IOHIDDeviceSetValueMultiple",
        "IOHIDDeviceSetValueWithCallback",
        "IOHIDDeviceSetValueMultipleWithCallback",
        "IOHIDDeviceGetReport",
        "IOHIDDeviceGetReportWithCallback",
        "IOHIDDeviceSetReport",
        "IOHIDDeviceSetReportWithCallback",
        // Misc accessors not in the v0.1 surface.
        "IOHIDDeviceGetService",
        "IOHIDDeviceConformsTo",
        "IOHIDDeviceCreate",
        "IOHIDDeviceSetProperty",
    ]);
    report("IOHIDDevice", &apple, &ours, &omitted);
}
