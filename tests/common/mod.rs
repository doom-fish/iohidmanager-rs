#![allow(dead_code)]

use std::path::PathBuf;
use std::process::Command;

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
