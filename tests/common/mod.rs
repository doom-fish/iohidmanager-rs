#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;

use iohidmanager::{keys, DeviceMatch, HidManager, HidUsage};

pub fn live_tests_enabled() -> bool {
    let enabled = std::env::var("IOHIDMANAGER_LIVE_TESTS").as_deref() == Ok("1");
    if !enabled {
        eprintln!("skip: set IOHIDMANAGER_LIVE_TESTS=1 to run tests that open HID devices");
    }
    enabled
}

pub fn match_non_keyboards(manager: &HidManager) {
    let matches: Vec<DeviceMatch> = [HidUsage::Mouse, HidUsage::Joystick, HidUsage::GamePad]
        .into_iter()
        .map(|usage| {
            let (page, usage) = usage.as_pair();
            DeviceMatch::new()
                .with_u32(keys::PRIMARY_USAGE_PAGE_KEY, page)
                .with_u32(keys::PRIMARY_USAGE_KEY, usage)
        })
        .collect();
    manager
        .set_device_matching_multiple(&matches)
        .expect("matching");
}

pub fn live_manager() -> Option<HidManager> {
    if !live_tests_enabled() {
        return None;
    }
    let manager = HidManager::new().expect("manager");
    match_non_keyboards(&manager);
    Some(manager)
}

pub fn hid_header(name: &str) -> String {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    let sdk = PathBuf::from(String::from_utf8(out.stdout).expect("utf8").trim());
    let path = sdk.join(format!(
        "System/Library/Frameworks/IOKit.framework/Headers/hid/{name}.h"
    ));
    let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    String::from_utf8_lossy(&bytes).into_owned()
}

pub fn parse_integer(text: &str) -> u64 {
    let text = text.trim();
    text.strip_prefix("0x")
        .or_else(|| text.strip_prefix("0X"))
        .map_or_else(
            || text.parse().expect("decimal"),
            |hex| u64::from_str_radix(hex, 16).expect("hex"),
        )
}

pub fn enum_value(header: &str, symbol: &str) -> u64 {
    let pattern = format!(r"\b{symbol}\s*=\s*(?:\(\s*)?(0x[0-9A-Fa-f]+|\d+)(?:\s*<<\s*(\d+))?");
    let re = regex_lite::Regex::new(&pattern).expect("regex");
    let caps = re
        .captures(header)
        .unwrap_or_else(|| panic!("{symbol} not found in SDK header"));
    let base = parse_integer(&caps[1]);
    caps.get(2)
        .map_or(base, |shift| base << parse_integer(shift.as_str()))
}
