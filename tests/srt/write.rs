use super::*;
use std::fs;

#[test]
fn write() {
    let ipath = data("srt.srt");
    let opath = temp("srt_write.srt");
    let _ = fs::remove_file(&opath);

    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write(&opath).unwrap();

    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    let mut olines = SrtLines::open_file(&opath).unwrap();
    while let Some(iline) = ilines.next() {
        assert_eq!(iline, olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}

#[test]
fn without_number() {
    let ipath = data("without_number.srt");
    let opath = temp("srt_write_without_number.srt");
    let _ = fs::remove_file(&opath);

    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write(&opath).unwrap();

    let mut ilines = SrtLines::open_file(&data("srt.srt")).unwrap();
    let mut olines = SrtLines::open_file(&opath).unwrap();
    while let Some(iline) = ilines.next() {
        assert_eq!(iline, olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}

#[test]
fn txt() {
    let ipath = data("four_lines.txt");
    let opath = temp("srt_write_txt.srt");
    let _ = fs::remove_file(&opath);

    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write(&opath).unwrap();
    let mut olines = ByteLines::open_file(&opath).unwrap();
    assert!(olines.next().is_none());
}

#[test]
fn with_bom() {
    let ipath = data("srt.srt");
    let opath = temp("srt_write_with_bom.srt");
    let _ = fs::remove_file(&opath);

    let mut opts = WriteOptions::new();
    opts.bom = true;
    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write_with(&opath, &opts).unwrap();

    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    let mut olines = SrtLines::open_file(&opath).unwrap();
    while let Some(iline) = ilines.next() {
        assert_eq!(iline, olines.next().unwrap());
    }
    assert!(olines.next().is_none());

    let mut ilines = ByteLines::open_file(&ipath).unwrap();
    let mut olines = ByteLines::open_file(&opath).unwrap();
    let mut exp = Vec::from("\u{feff}".as_bytes());
    exp.extend_from_slice(ilines.next().unwrap());
    assert_eq!(exp, olines.next().unwrap());
}

#[test]
fn with_add_time() {
    let ipath = data("srt.srt");
    let opath = temp("srt_write_with_add_time.srt");
    let _ = fs::remove_file(&opath);

    let mut opts = WriteOptions::default();
    opts.add_time = Some(Time::new_unchecked(0, 1, 0, 0));
    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write_with(&opath, &opts).unwrap();

    let mut olines = SrtLines::open_file(&opath).unwrap();
    for s in [
        "1",
        "00:01:00,000 --> 00:01:05,000",
        "It's simple subtitles",
    ] {
        assert_eq!(SrtLine::new(s), olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}

#[test]
fn with_sub_time() {
    let ipath = data("two_blocks.srt");
    let opath = temp("srt_write_with_sub_time.srt");
    let _ = fs::remove_file(&opath);

    let mut opts = WriteOptions::default();
    opts.sub_time = Some(Time::new_unchecked(0, 0, 5, 0));
    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write_with(&opath, &opts).unwrap();

    let mut olines = SrtLines::open_file(&opath).unwrap();
    for s in [
        "1",
        "00:00:05,000 --> 00:00:10,000",
        "10-15",
        "",
        "2",
        "00:00:10,000 --> 00:00:15,000",
        "15-20",
    ] {
        assert_eq!(SrtLine::new(s), olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}

#[test]
fn with_start_from() {
    let ipath = data("two_blocks.srt");
    let opath = temp("srt_write_with_start_from.srt");
    let _ = fs::remove_file(&opath);

    let mut opts = WriteOptions::default();
    opts.start_from = Some(Time::new_unchecked(0, 0, 15, 0));
    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write_with(&opath, &opts).unwrap();

    let mut olines = SrtLines::open_file(&opath).unwrap();
    for s in ["1", "00:00:15,000 --> 00:00:20,000", "15-20"] {
        assert_eq!(SrtLine::new(s), olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}

#[test]
fn with_end_on() {
    let ipath = data("two_blocks.srt");
    let opath = temp("srt_write_with_end_on.srt");
    let _ = fs::remove_file(&opath);

    let mut opts = WriteOptions::default();
    opts.end_on = Some(Time::new_unchecked(0, 0, 15, 0));
    let mut ilines = SrtLines::open_file(&ipath).unwrap();
    ilines.write_with(&opath, &opts).unwrap();

    let mut olines = SrtLines::open_file(&opath).unwrap();
    for s in ["1", "00:00:10,000 --> 00:00:15,000", "10-15"] {
        assert_eq!(SrtLine::new(s), olines.next().unwrap());
    }
    assert!(olines.next().is_none());
}
