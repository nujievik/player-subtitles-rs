mod common;
#[path = "srt/write.rs"]
mod write;

use common::*;
use player_subtitles::{srt::line::SrtLine, *};

const SIMPLE: &[u8] = br"1
00:00:05,000 --> 00:00:10,000
First block

2
00:00:10,000 --> 00:00:15,000
Second block
";

#[test]
fn iter() {
    let mut srt = SrtLines::from_bytes(SIMPLE);
    for i in 0..2 {
        if i > 0 {
            assert!(matches!(srt.next().unwrap(), SrtLine::Blank));
        }
        assert!(matches!(srt.next().unwrap(), SrtLine::Number(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::TimeRange(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::Text(_)));
    }
    assert!(srt.next().is_none());
}

#[test]
fn iter_file() {
    let mut byte_lines = SrtLines::from_bytes(SIMPLE);
    let mut file_lines = SrtLines::open_file(data("srt.srt")).unwrap();
    while let Some(l) = byte_lines.next() {
        assert_eq!(l, file_lines.next().unwrap());
    }
    assert!(file_lines.next().is_none());
}

#[test]
fn iter_bom_file() {
    let mut regular_lines = SrtLines::open_file(data("srt.srt")).unwrap();
    let mut bomed_lines = SrtLines::open_file(data("srt_with_bom.srt")).unwrap();
    while let Some(l) = regular_lines.next() {
        assert_eq!(l, bomed_lines.next().unwrap());
    }
    assert!(bomed_lines.next().is_none());
}

#[test]
fn iter_cp1251_file() {
    let mut srt = SrtLines::open_file(data("cp1251.srt")).unwrap();
    assert!(matches!(srt.next().unwrap(), SrtLine::Number(_)));
    assert!(matches!(srt.next().unwrap(), SrtLine::TimeRange(_)));
    assert!(matches!(srt.next().unwrap(), SrtLine::Text(_)));
    assert!(srt.next().is_none());
}

#[test]
fn iter_from_ass_lines() {
    let mut srt = SrtLines::from(AssLines::open_file(data("ass.ass")).unwrap());
    for _ in 0..2 {
        assert!(matches!(srt.next().unwrap(), SrtLine::Number(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::TimeRange(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::Text(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::Blank));
    }
    assert!(srt.next().is_none());
}

#[test]
fn iter_from_vtt_lines() {
    let mut srt = SrtLines::from(VttLines::open_file(data("vtt.vtt")).unwrap());
    for _ in 0..2 {
        assert!(matches!(srt.next().unwrap(), SrtLine::Number(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::TimeRange(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::Text(_)));
        assert!(matches!(srt.next().unwrap(), SrtLine::Blank));
    }
    assert!(srt.next().is_none());
}
