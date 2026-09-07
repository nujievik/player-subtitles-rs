mod common;

use common::*;
use player_subtitles::*;

#[test]
fn from_srt_lines() {
    let mut srt = SrtLines::open_file(data("srt.srt")).unwrap();
    let mut ass = AssLines::from(srt);
    ass.write(&temp("ass_from_srt_lines.ass")).unwrap();
}

#[test]
fn from_vtt_lines() {
    let mut vtt = VttLines::open_file(data("vtt.vtt")).unwrap();
    let mut ass = AssLines::from(vtt);
    ass.write(&temp("ass_from_vtt_lines.ass")).unwrap();
}
