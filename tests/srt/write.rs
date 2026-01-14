use super::*;
use std::sync::LazyLock;

static SRT: LazyLock<SrtSubtitles<'static>> =
    LazyLock::new(|| SrtSubtitles::read(&data("srt.srt")).unwrap());

#[test]
fn write() {
    let opath = temp("srt_write.srt");
    SRT.write(&opath).unwrap();
    let osrt = SrtSubtitles::read(&opath).unwrap();
    assert_eq!(&SRT[..3], &osrt[..3]);
    assert_eq!(SRT[3].ty, osrt[3].ty);
}

#[test]
fn write_with_bom() {
    let opath = temp("srt_write_with_bom.srt");
    let mut opts = WriteOptions::default();
    opts.bom = true;
    SRT.write_with(&opath, &opts).unwrap();
    let osrt = SrtSubtitles::read(&opath).unwrap();

    assert_eq!(
        line::new_with_ty("\u{feff}1\n", SrtLineType::Number),
        osrt[0]
    );
    assert_eq!(&SRT[1..3], &osrt[1..3]);
    assert_eq!(SRT[3].ty, osrt[3].ty);
}

#[test]
fn write_with_add_time() {
    let opath = temp("srt_write_with_add_time.srt");
    let mut opts = WriteOptions::default();
    let add = Time::new_unchecked(0, 1, 0, 0);
    opts.add_time = Some(add);
    SRT.write_with(&opath, &opts).unwrap();

    let osrt = SrtSubtitles::read(&opath).unwrap();
    assert_eq!(&SRT[0], &osrt[0]);
    assert_eq!(SrtLine::new("00:01:00,000 --> 00:01:05,000\n"), osrt[1]);
    assert_eq!(&SRT[2], &osrt[2]);
    assert_eq!(SRT[3].ty, osrt[3].ty);
}

#[test]
fn read_and_write_with_default() {
    let ipath = data("srt.srt");
    let opath = temp("srt_read_and_write_with_default.srt");
    let opts = WriteOptions::default();
    SrtSubtitles::read_and_write_with(&ipath, &opath, &opts).unwrap();

    let osrt = SrtSubtitles::read(&opath).unwrap();
    assert_eq!(&SRT[..3], &osrt[..3]);
    assert_eq!(SRT[3].ty, osrt[3].ty);
}

#[test]
fn raw_write() {
    let opath = temp("srt_raw_write.srt");

    ["srt.srt", "cp1251.srt", "bomed.srt"].iter().for_each(|f| {
        let isrt = SrtSubtitles::read(&data(f)).unwrap();
        isrt.raw_write(&opath).unwrap();
        assert_eq!(isrt, SrtSubtitles::read(&opath).unwrap());
    })
}
