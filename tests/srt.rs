mod common;
#[path = "srt/write.rs"]
mod write;

use common::*;
use player_subtitles::*;

macro_rules! test_iter_file {
    ($fn:ident, $file:expr, $lines:expr) => {
        #[test]
        fn $fn() {
            let mut lines = SrtLines::open_file(data($file)).unwrap();
            for s in $lines {
                let l = SrtLine::new(s.as_bytes());
                assert_eq!(lines.next().unwrap(), l);
            }
            assert!(lines.next().is_none());
        }
    };
}

test_iter_file!(iter_txt_file, "four_lines.txt", ["0", "1", "2", "3"]);
test_iter_file!(
    iter_srt_file,
    "srt.srt",
    [
        "1",
        "00:00:00,000 --> 00:00:05,000",
        "It's simple srt subtitles"
    ]
);
test_iter_file!(
    iter_bomed_srt_file,
    "bomed.srt",
    [
        "1",
        "00:00:00,000 --> 00:00:05,000",
        "It's simple srt subtitles"
    ]
);

#[test]
fn iter_cp1251_srt_file() {
    let mut lines = SrtLines::open_file(data("cp1251.srt")).unwrap();
    for s in ["1", "00:00:00,000 --> 00:00:05,000"] {
        let l = SrtLine::new(s.as_bytes());
        assert_eq!(lines.next().unwrap(), l);
    }
    let l = SrtLine::new(&[
        99, 112, 49, 50, 53, 49, 32, 241, 243, 225, 242, 232, 242, 240, 251,
    ]);
    assert_eq!(lines.next().unwrap(), l);
    assert!(lines.next().is_none());
}
