mod common;
#[path = "vtt/write.rs"]
mod write;

use common::*;
use player_subtitles::{vtt::line::*, *};

const SIMPLE: &[u8] = br"WEBVTT

00:00:05.000 --> 00:00:10.000
First block

00:00:10.000 --> 00:00:15.000
Second block
";

#[test]
fn iter() {
    let mut vtt = VttLines::from_bytes(SIMPLE);
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    for _ in 0..2 {
        assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
        assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
        assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    }
    assert!(vtt.next().is_none());
}

#[test]
fn iter_file() {
    let mut byte_lines = VttLines::from_bytes(SIMPLE);
    let mut file_lines = VttLines::open_file(data("vtt.vtt")).unwrap();
    while let Some(l) = byte_lines.next() {
        assert_eq!(l, file_lines.next().unwrap());
    }
    assert!(file_lines.next().is_none());
}

#[test]
fn iter_bom_file() {
    let mut regular_lines = VttLines::open_file(data("vtt.vtt")).unwrap();
    let mut bomed_lines = VttLines::open_file(data("vtt_with_bom.vtt")).unwrap();
    while let Some(l) = regular_lines.next() {
        assert_eq!(l, bomed_lines.next().unwrap());
    }
    assert!(bomed_lines.next().is_none());
}

#[test]
fn iter_cp1251_file() {
    let mut vtt = VttLines::open_file(data("cp1251.vtt")).unwrap();
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
    assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    assert!(vtt.next().is_none());
}

#[test]
fn iter_from_ass_lines() {
    let mut vtt = VttLines::from(AssLines::open_file(data("ass.ass")).unwrap());
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    for _ in 0..2 {
        assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
        assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
        assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    }
    assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
    assert!(vtt.next().is_none());
}

#[test]
fn iter_from_srt_lines() {
    let mut vtt = VttLines::from(SrtLines::open_file(data("srt.srt")).unwrap());
    assert!(matches!(vtt.next().unwrap(), VttLine::VttFileMark(_)));
    for _ in 0..2 {
        assert!(matches!(vtt.next().unwrap(), VttLine::Blank));
        assert!(matches!(vtt.next().unwrap(), VttLine::CueId(_)));
        assert!(matches!(vtt.next().unwrap(), VttLine::TimeRangeAndStyle(_)));
        assert!(matches!(vtt.next().unwrap(), VttLine::Text(_)));
    }
    assert!(vtt.next().is_none());
}
