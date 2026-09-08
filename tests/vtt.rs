mod common;

use common::*;
use player_subtitles::{vtt::line::*, *};
use std::fs;

const SIMPLE: &[u8] = br"WEBVTT

00:00:00.000 --> 00:00:05.000
It's simple subtitles
";

#[test]
fn iter() {
    let mut vtt = VttLines::from_bytes(SIMPLE);
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    assert!(vtt.next().is_none());
}

#[test]
fn write() {
    let dst = temp("write_vtt.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    vtt.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn from_ass_lines() {
    let mut vtt = VttLines::from(AssLines::open_file(data("ass.ass")).unwrap());
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(vtt.next().is_none());
}

#[test]
fn from_srt_lines() {
    let mut vtt = VttLines::from(SrtLines::open_file(data("srt.srt")).unwrap());
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(matches!(vtt.next().unwrap(), VttLine::CueId(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    assert!(vtt.next().is_none());
}
