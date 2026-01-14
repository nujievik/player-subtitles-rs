#[path = "srt/block.rs"]
mod block;
mod common;
#[path = "srt/line.rs"]
mod line;
#[path = "srt/new.rs"]
mod new;
#[path = "srt/standardize.rs"]
mod standardize;
#[path = "srt/write.rs"]
mod write;

use common::*;
use player_subtitles::*;

fn new_standard() -> SrtSubtitles<'static> {
    let st = Time::new_unchecked(0, 0, 0, 0);
    let end = Time::new_unchecked(0, 0, 5, 0);
    let lines = vec![
        line::new_with_ty("1\n", SrtLineType::Number),
        line::new_with_ty(
            "00:00:00,000 --> 00:00:05,000\n",
            SrtLineType::TimeRange((st, end)),
        ),
        line::new_with_ty("x\n", SrtLineType::Text),
        line::new_with_ty("\n", SrtLineType::Blank),
    ];
    SrtSubtitles(lines)
}

#[test]
fn is_standard() {
    let mut srt = new_standard();
    assert!(srt.is_standard());
    for _ in 0..4 {
        srt.pop();
        assert!(!srt.is_standard());
    }
}

#[test]
fn blocks() {
    let mut srt = new_standard();
    for _ in 0..4 {
        {
            let mut blocks = srt.blocks();
            assert_eq!(Some(SrtBlock(&srt[..])), blocks.next());
            assert_eq!(None, blocks.next());
        }
        srt.pop();
    }
}

#[test]
fn multiple_blocks() {
    let lines = vec![
        SrtLine::new("a"),
        SrtLine::new(""),
        SrtLine::new("b"),
        SrtLine::new("c"),
        SrtLine::new(""),
        SrtLine::new("de"),
    ];
    let srt = SrtSubtitles(lines);
    let mut blocks = srt.blocks();
    assert_eq!(Some(SrtBlock(&srt[0..2])), blocks.next());
    assert_eq!(Some(SrtBlock(&srt[2..5])), blocks.next());
    assert_eq!(Some(SrtBlock(&srt[5..6])), blocks.next());
    assert_eq!(None, blocks.next());
}
