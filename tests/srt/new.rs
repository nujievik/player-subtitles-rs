use super::*;

#[test]
fn from_bytes() {
    let srt = SrtSubtitles::from_bytes("1\nx");
    assert_eq!(srt.len(), 3);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("x"));
    assert_eq!(srt[2], SrtLine::new(""));
}

#[test]
fn from_str() {
    let srt = SrtSubtitles::from_str("1\nx");
    assert_eq!(srt.len(), 3);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("x"));
    assert_eq!(srt[2], SrtLine::new(""));
}

#[test]
fn try_from_bytes() {
    SrtSubtitles::try_from_bytes("1\nx").unwrap_err();

    let srt = SrtSubtitles::try_from_bytes("1\n00:00:00,000 --> 00:00:05,000\nx\n\n").unwrap();
    assert_eq!(srt.len(), 4);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("00:00:00,000 --> 00:00:05,000\n"));
    assert_eq!(srt[2], SrtLine::new("x\n"));
    assert_eq!(srt[3], SrtLine::new("\n"));
}

#[test]
fn try_from_str() {
    SrtSubtitles::try_from_str("1\nx").unwrap_err();

    let srt = SrtSubtitles::try_from_str("1\n00:00:00,000 --> 00:00:05,000\nx\n\n").unwrap();
    assert_eq!(srt.len(), 4);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("00:00:00,000 --> 00:00:05,000\n"));
    assert_eq!(srt[2], SrtLine::new("x\n"));
    assert_eq!(srt[3], SrtLine::new("\n"));
}

#[test]
fn read() {
    let srt = SrtSubtitles::read(&data("srt.srt")).unwrap();
    assert_eq!(srt.len(), 4);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("00:00:00,000 --> 00:00:05,000\n"));
    assert_eq!(srt[2], SrtLine::new("It's simple srt subtitles\n"));
    assert_eq!(srt[3], SrtLine::new(""));
}

#[test]
fn try_read() {
    let srt = SrtSubtitles::try_read(&data("srt.srt")).unwrap();
    assert_eq!(srt.len(), 4);
    assert_eq!(srt[0], SrtLine::new("1\n"));
    assert_eq!(srt[1], SrtLine::new("00:00:00,000 --> 00:00:05,000\n"));
    assert_eq!(srt[2], SrtLine::new("It's simple srt subtitles\n"));
    assert_eq!(srt[3], SrtLine::new(""));
}

#[test]
fn read_non_standard() {
    let srt = SrtSubtitles::read(&data("non_standard.srt")).unwrap();
    assert_eq!(srt.len(), 3);
    assert_eq!(srt[0], SrtLine::new("00:00:00,000 --> 00:00:05,000\n"));
    assert_eq!(srt[1], SrtLine::new("It's simple srt subtitles\n"));
    assert_eq!(srt[2], SrtLine::new(""));
}

#[test]
fn try_read_non_standard() {
    SrtSubtitles::try_read(&data("non_standard.srt")).unwrap_err();
}
