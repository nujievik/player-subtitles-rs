mod common;

use player_subtitles::*;
use common::*;

#[test]
fn from_srt_lines() {
    let mut srt = SrtLines::open_file(data("srt.srt")).unwrap();
    let mut ass = AssLines::from(srt);
    ass.write(&temp("ass_from_srt_lines.ass")).unwrap();
}
