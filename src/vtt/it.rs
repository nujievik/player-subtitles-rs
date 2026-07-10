use super::line::{Comment, CueId, Metadata, Region, Style, Text, TimeRangeAndStyle, VttFileMark};
use super::{RegularVttLines, VttLine, VttLines};
use crate::{AssLines, ByteLines, SourceLines, SrtLine, SrtLines, StreamingIterator, byte_helpers};
use std::io::BufRead;

impl<T: BufRead> StreamingIterator for VttLines<'_, T> {
    type Item<'a>
        = VttLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.source {
            SourceLines::Vtt(lines) => lines.next(),
            _ => todo!(),
        }
    }
}

impl<T: BufRead> StreamingIterator for RegularVttLines<'_, T> {
    type Item<'a>
        = VttLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        next_regular(
            &mut self.lines,
            &mut self.body_state,
            &mut self.current_state,
        )
    }
}

pub enum BodyState {
    Init,
    RegionsAndStyles,
    Cues,
}

pub enum CurrentState {
    Outside,
    InRegion,
    InStyle,
    InComment,
    InCue,
    InMetadata,
    InText,
}

fn next_regular<'a, T: BufRead>(
    byte_lines: &'a mut ByteLines<'_, T>,
    body_state: &mut BodyState,
    current_state: &mut CurrentState,
) -> Option<VttLine<'a>> {
    let mut bytes = byte_lines.next()?;

    if let BodyState::Init = body_state {
        *body_state = BodyState::RegionsAndStyles;
        bytes = byte_helpers::trim_bom(bytes);
        bytes = byte_helpers::trim(bytes);
        if bytes.starts_with(b"WEBVTT") {
            return Some(VttLine::VttFileMark(VttFileMark { bytes }));
        }
    } else {
        bytes = byte_helpers::trim(bytes);
    }

    if bytes.is_empty() {
        *current_state = CurrentState::Outside;
        return Some(VttLine::Blank);
    }

    if let CurrentState::InComment = current_state {
        return Some(VttLine::Comment(Comment::new(bytes, bytes)));
    }
    if bytes.starts_with(b"NOTE") {
        *current_state = CurrentState::InComment;
        let text = if bytes.len() == 4 {
            &[]
        } else {
            byte_helpers::trim_start(&bytes[4..])
        };
        return Some(VttLine::Comment(Comment::new(bytes, text)));
    }

    if let BodyState::RegionsAndStyles = body_state {
        return Some(match current_state {
            CurrentState::Outside => {
                if bytes == b"REGION" {
                    *current_state = CurrentState::InRegion;
                    VttLine::RegionMark
                } else if bytes == b"STYLE" {
                    *current_state = CurrentState::InStyle;
                    VttLine::StyleMark
                } else {
                    *body_state = BodyState::Cues;
                    VttLine::CueId(CueId { bytes })
                }
            }
            CurrentState::InRegion => VttLine::Region(Region { bytes }),
            CurrentState::InStyle => VttLine::Style(Style { bytes }),
            _ => VttLine::Unrecognized(bytes),
        });
    }

    if let CurrentState::InText = current_state {
        return Some(VttLine::Text(Text { bytes }));
    }

    if let CurrentState::InMetadata = current_state {
        return Some(VttLine::Metadata(Metadata { bytes }));
    }

    if let CurrentState::InCue = current_state {
        return Some(if bytes.starts_with(b"{") {
            *current_state = CurrentState::InMetadata;
            VttLine::Metadata(Metadata { bytes })
        } else {
            *current_state = CurrentState::InText;
            VttLine::Text(Text { bytes })
        });
    }

    if let Some(ts) = TimeRangeAndStyle::get_new(bytes) {
        *body_state = BodyState::Cues;
        *current_state = CurrentState::InCue;
        return Some(VttLine::TimeRangeAndStyle(ts));
    }

    Some(VttLine::Unrecognized(bytes))
}

fn next_ass<'a, T: BufRead>(lines: &'a AssLines<'_, T>) -> Option<VttLine<'a>> {
    todo!();
}

fn next_srt<'a, T: BufRead>(srt_lines: &'a mut SrtLines<'_, T>) -> Option<VttLine<'a>> {
    let line = srt_lines
        .find(|l| matches!(l, SrtLine::Blank | SrtLine::TimeRange(_) | SrtLine::Text(_)))?;
    let line = match line {
        SrtLine::Blank => VttLine::Blank,
        SrtLine::TimeRange(bs) => VttLine::TimeRangeAndStyle(TimeRangeAndStyle {
            bytes: bs.bytes,
            start: bs.start,
            end: bs.end,
        }),
        SrtLine::Text(bs) => VttLine::Text(Text { bytes: bs.bytes }),
        _ => return None,
    };
    Some(line)
}
