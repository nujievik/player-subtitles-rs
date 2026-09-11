mod common;
#[path = "ass/write.rs"]
mod write;

use common::*;
use player_subtitles::{ass::line::*, *};
use std::io::BufRead;

const SIMPLE: &[u8] = br"[Script Info]
ScriptType: v4.00+
WrapStyle: 0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:05:00,0:00:10:00,Default,,0,0,0,,First block
Dialogue: 0,0:00:10:00,0:00:15:00,Default,,0,0,0,,Second block
";

fn assert_iter_simple<T: BufRead>(ass: &mut AssLines<T>) {
    assert_eq!(
        ass.next().unwrap(),
        AssLine::SectionMark(SectionMark::ScriptInfo)
    );
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::ScriptInfo(ScriptInfo::ScriptType(_))
    ));
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::ScriptInfo(ScriptInfo::WrapStyle(_))
    ));
    assert!(matches!(ass.next().unwrap(), AssLine::Blank));
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::SectionMark(SectionMark::Events)
    ));
    assert!(matches!(ass.next().unwrap(), AssLine::EventFormat(_)));
    assert!(matches!(ass.next().unwrap(), AssLine::Event(_)));
    assert!(matches!(ass.next().unwrap(), AssLine::Event(_)));

    assert!(ass.next().is_none());
}

#[test]
fn iter() {
    let mut ass = AssLines::from_bytes(SIMPLE);
    assert_iter_simple(&mut ass);
}

#[test]
fn iter_file() {
    let mut byte_lines = AssLines::from_bytes(SIMPLE);
    let mut file_lines = AssLines::open_file(data("ass.ass")).unwrap();
    while let Some(l) = byte_lines.next() {
        assert_eq!(l, file_lines.next().unwrap());
    }
    assert!(file_lines.next().is_none());
}

#[test]
fn iter_bom_file() {
    let mut regular_lines = AssLines::open_file(data("ass.ass")).unwrap();
    let mut bomed_lines = AssLines::open_file(data("ass_with_bom.ass")).unwrap();
    while let Some(l) = regular_lines.next() {
        assert_eq!(l, bomed_lines.next().unwrap());
    }
    assert!(bomed_lines.next().is_none());
}

#[test]
fn iter_cp1251_file() {
    let mut ass = AssLines::open_file(data("cp1251.ass")).unwrap();
    assert_eq!(
        ass.next().unwrap(),
        AssLine::SectionMark(SectionMark::ScriptInfo)
    );
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::ScriptInfo(ScriptInfo::ScriptType(_))
    ));
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::ScriptInfo(ScriptInfo::WrapStyle(_))
    ));
    assert!(matches!(ass.next().unwrap(), AssLine::Blank));
    assert!(matches!(
        ass.next().unwrap(),
        AssLine::SectionMark(SectionMark::Events)
    ));
    assert!(matches!(ass.next().unwrap(), AssLine::EventFormat(_)));
    assert!(matches!(ass.next().unwrap(), AssLine::Event(_)));

    assert!(ass.next().is_none());
}

#[test]
fn iter_from_srt_lines() {
    let mut ass = AssLines::from(SrtLines::open_file(data("srt.srt")).unwrap());
    assert_iter_simple(&mut ass)
}

#[test]
fn iter_from_vtt_lines() {
    let mut ass = AssLines::from(VttLines::open_file(data("vtt.vtt")).unwrap());
    assert_iter_simple(&mut ass)
}
