#![allow(dead_code)]

use std::path::PathBuf;

pub fn data(s: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.push("tests");
    p.push("data");
    for s in s.split('/') {
        p.push(s);
    }
    p
}

pub fn temp(s: &str) -> PathBuf {
    let mut p = PathBuf::from(data(""));
    p.push("temp");
    for s in s.split('/') {
        p.push(s);
    }
    p
}
