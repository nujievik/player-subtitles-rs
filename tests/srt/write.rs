use super::*;
use std::fs;

#[test]
fn write() {
    let dst = temp("srt_write.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    srt.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn write_with_bom() {
    let dst = temp("srt_write_with_bom.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.bom = true;
    srt.write_with(&dst, &opts).unwrap();

    let mut expected: Vec<u8> = "\u{feff}".as_bytes().to_vec();
    expected.extend_from_slice(SIMPLE);
    assert_eq!(fs::read(&dst).unwrap(), expected);
}

#[test]
fn write_with_start() {
    let dst = temp("srt_write_with_start.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.start = Some(Time::new_unchecked(0, 0, 10, 0));
    srt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"1
00:00:10,000 --> 00:00:15,000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_end() {
    let dst = temp("srt_write_with_end.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.end = Some(Time::new_unchecked(0, 0, 10, 0));
    srt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"1
00:00:05,000 --> 00:00:10,000
First block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_add_time() {
    let dst = temp("srt_write_with_add_time.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.add_time = Some(Time::new_unchecked(0, 0, 5, 0));
    srt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"1
00:00:10,000 --> 00:00:15,000
First block

2
00:00:15,000 --> 00:00:20,000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_with_sub_time() {
    let dst = temp("srt_write_with_sub_time.srt");
    let mut srt = SrtLines::from_bytes(SIMPLE);
    let mut opts = WriteOptions::new();
    opts.sub_time = Some(Time::new_unchecked(0, 0, 5, 0));
    srt.write_with(&dst, &opts).unwrap();

    const EXPECTED: &[u8] = br"1
00:00:00,000 --> 00:00:05,000
First block

2
00:00:05,000 --> 00:00:10,000
Second block
";
    assert_eq!(fs::read(&dst).unwrap(), EXPECTED);
}

#[test]
fn write_from_ass() {
    let dst = temp("srt_write_from_ass.srt");
    let mut srt = SrtLines::from(AssLines::open_file(data("ass.ass")).unwrap());
    srt.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}

#[test]
fn write_from_vtt() {
    let dst = temp("srt_write_from_vtt.srt");
    let mut srt = SrtLines::from(VttLines::open_file(data("vtt.vtt")).unwrap());
    srt.write(&dst).unwrap();
    assert_eq!(fs::read(&dst).unwrap(), SIMPLE);
}
