use super::*;
use std::fs;

#[test]
fn write() {
    let dst = temp("vtt_write.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    vtt.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn write_with_bom() {
    let dst = temp("vtt_write_with_bom.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.bom = true;
    vtt.write_with(&dst, &opts).unwrap();

    let mut expected: Vec<u8> = "\u{feff}".as_bytes().to_vec();
    expected.extend_from_slice(SIMPLE);
    assert_eq!(fs::read(&dst).unwrap(), expected);
}

#[test]
fn write_with_start() {
    let dst = temp("vtt_write_with_start.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.start = Some(Time::new_unchecked(0, 0, 10, 0));
    vtt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"WEBVTT

00:00:10.000 --> 00:00:15.000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_end() {
    let dst = temp("vtt_write_with_end.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.end = Some(Time::new_unchecked(0, 0, 10, 0));
    vtt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"WEBVTT

00:00:05.000 --> 00:00:10.000
First block

";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_add_time() {
    let dst = temp("vtt_write_with_add_time.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.add_time = Some(Time::new_unchecked(0, 0, 5, 0));
    vtt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"WEBVTT

00:00:10.000 --> 00:00:15.000
First block

00:00:15.000 --> 00:00:20.000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_sub_time() {
    let dst = temp("vtt_write_with_sub_time.vtt");
    let mut vtt = VttLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.sub_time = Some(Time::new_unchecked(0, 0, 5, 0));
    vtt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"WEBVTT

00:00:00.000 --> 00:00:05.000
First block

00:00:05.000 --> 00:00:10.000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_from_ass() {
    let dst = temp("vtt_write_from_ass.vtt");
    let mut vtt = VttLines::from(AssLines::open_file(data("ass.ass")).unwrap());
    vtt.write(&dst).unwrap();

    let mut expected = SIMPLE.to_vec();
    expected.push(b'\n');
    assert_eq!(fs::read(&dst).unwrap(), expected);
}

#[test]
fn write_from_srt() {
    let dst = temp("vtt_write_from_srt.vtt");
    let mut vtt = VttLines::from(SrtLines::open_file(data("srt.srt")).unwrap());
    vtt.write(&dst).unwrap();

    const EXPECTED: &[u8] = br"WEBVTT

1
00:00:05.000 --> 00:00:10.000
First block

2
00:00:10.000 --> 00:00:15.000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}
