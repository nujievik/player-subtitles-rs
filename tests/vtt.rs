mod common;

use common::*;
use player_subtitles::*;

#[test]
fn from_srt_lines() {
    let srt = SrtLines::open_file(data("srt.srt")).unwrap();
    let mut vtt = VttLines::from(srt);
    vtt.write(&temp("vtt_from_srt_lines.vtt")).unwrap();
}
