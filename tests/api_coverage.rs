//! API-surface coverage harness for `iohidmanager`.
//!
//! `IOKit` HID lives in `IOKit.framework/Headers/hid/`. Pure C headers,
//! mirroring the apple-cf / cgevents / imageio C-function-regex pattern.

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

fn report(name: &str, apple: &BTreeSet<String>, ours: &BTreeSet<String>) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple.difference(ours).collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for missing_fn in &missing {
            println!("  - {missing_fn}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

#[test]
fn iohid_manager_coverage() {
    let header = read_header("IOHIDManager");
    report(
        "IOHIDManager",
        &extract_c_functions("IOHIDManager", &header),
        &extract_rust_externs(),
    );
}

#[test]
fn iohid_device_coverage() {
    let header = read_header("IOHIDDevice");
    report(
        "IOHIDDevice",
        &extract_c_functions("IOHIDDevice", &header),
        &extract_rust_externs(),
    );
}

#[test]
fn iohid_element_coverage() {
    let header = read_header("IOHIDElement");
    report(
        "IOHIDElement",
        &extract_c_functions("IOHIDElement", &header),
        &extract_rust_externs(),
    );
}

#[test]
fn iohid_value_coverage() {
    let header = read_header("IOHIDValue");
    report(
        "IOHIDValue",
        &extract_c_functions("IOHIDValue", &header),
        &extract_rust_externs(),
    );
}

#[test]
fn iohid_transaction_coverage() {
    let header = read_header("IOHIDTransaction");
    report(
        "IOHIDTransaction",
        &extract_c_functions("IOHIDTransaction", &header),
        &extract_rust_externs(),
    );
}

#[test]
fn iohid_queue_coverage() {
    let header = read_header("IOHIDQueue");
    report(
        "IOHIDQueue",
        &extract_c_functions("IOHIDQueue", &header),
        &extract_rust_externs(),
    );
}
