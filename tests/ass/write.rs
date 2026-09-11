use super::*;
use std::fs;

#[test]
fn write() {
    let dst = temp("ass_write.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    ass.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn write_with_bom() {
    let dst = temp("ass_write_with_bom.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.bom = true;
    ass.write_with(&dst, &opts).unwrap();

    let mut expected: Vec<u8> = "\u{feff}".as_bytes().to_vec();
    expected.extend_from_slice(SIMPLE);
    assert_eq!(fs::read(&dst).unwrap(), expected);
}

#[test]
fn write_with_start() {
    let dst = temp("ass_write_with_start.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.start = Some(Time::new_unchecked(0, 0, 10, 0));
    ass.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"[Script Info]
ScriptType: v4.00+
WrapStyle: 0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:10:00,0:00:15:00,Default,,0,0,0,,Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_end() {
    let dst = temp("ass_write_with_end.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.end = Some(Time::new_unchecked(0, 0, 10, 0));
    ass.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"[Script Info]
ScriptType: v4.00+
WrapStyle: 0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:05:00,0:00:10:00,Default,,0,0,0,,First block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_add_time() {
    let dst = temp("ass_write_with_add_time.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.add_time = Some(Time::new_unchecked(0, 0, 5, 0));
    ass.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"[Script Info]
ScriptType: v4.00+
WrapStyle: 0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:10:00,0:00:15:00,Default,,0,0,0,,First block
Dialogue: 0,0:00:15:00,0:00:20:00,Default,,0,0,0,,Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_sub_time() {
    let dst = temp("ass_write_with_sub_time.ass");
    let mut ass = AssLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.sub_time = Some(Time::new_unchecked(0, 0, 5, 0));
    ass.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"[Script Info]
ScriptType: v4.00+
WrapStyle: 0

[Events]
Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text
Dialogue: 0,0:00:00:00,0:00:05:00,Default,,0,0,0,,First block
Dialogue: 0,0:00:05:00,0:00:10:00,Default,,0,0,0,,Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_from_srt() {
    let dst = temp("ass_write_from_srt.ass");
    let mut ass = AssLines::from(SrtLines::open_file(data("srt.srt")).unwrap());
    ass.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn write_from_vtt() {
    let dst = temp("ass_write_from_vtt.ass");
    let mut ass = AssLines::from(VttLines::open_file(data("vtt.vtt")).unwrap());
    ass.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}
