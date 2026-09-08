use super::line::{
    Comment, CueId, Metadata, Region, Style, Text, TimeRangeAndStyle, VttFileMark, VttLine,
};
use super::{RegularVttLines, VttLines};
use crate::{
    AssLine, ByteLines, RegularAssLines, RegularSrtLines, SourceLines, SrtLine, StreamingIterator,
    Time, byte_helpers,
};
use std::io::BufRead;

impl<T: BufRead> StreamingIterator for VttLines<'_, T> {
    type Item<'a>
        = VttLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.source {
            SourceLines::Ass(lines) => next_from_ass(lines, &mut self.buf, &mut self.trans_state),
            SourceLines::Srt(lines) => next_from_srt(lines, &mut self.buf, &mut self.trans_state),
            SourceLines::Vtt(lines) => lines.next(),
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

#[derive(Debug)]
pub enum BodyState {
    Init,
    RegionsAndStyles,
    Cues,
}

#[derive(Debug)]
pub enum CurrentState {
    Outside,
    InRegion,
    InStyle,
    InComment,
    InCue,
    InMetadata,
    InText,
}

#[derive(Debug)]
pub enum TransIterState {
    Init,
    Outside,
    TimeRange(Time, Time),
    Text(usize),
    Blank,
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

                    if let Some(ts) = TimeRangeAndStyle::get_new(bytes) {
                        *current_state = CurrentState::InCue;
                        VttLine::TimeRangeAndStyle(ts)
                    } else {
                        VttLine::CueId(CueId { bytes })
                    }
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

fn next_from_ass<'a, T: BufRead>(
    lines: &mut RegularAssLines<'_, T>,
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<VttLine<'a>> {
    let fake_buf = unsafe { &mut *(buf as *mut Vec<u8>) };

    if let Some(line) = next_from_trans_state(fake_buf, trans_state) {
        return Some(line);
    }

    let event = lines.find_map(|l| match l {
        AssLine::Event(event) => Some(event),
        _ => None,
    })?;
    *trans_state = TransIterState::TimeRange(event.start, event.end);
    buf.clear();
    buf.extend_from_slice(event.text);

    next_from_trans_state(buf, trans_state)
}

fn next_from_srt<'a, T: BufRead>(
    srt_lines: &'a mut RegularSrtLines<'_, T>,
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<VttLine<'a>> {
    if let Some(line) = next_from_trans_state(buf, trans_state) {
        return Some(line);
    }

    srt_lines.next().map(|line| match line {
        SrtLine::Blank => VttLine::Blank,
        SrtLine::Number(num) => VttLine::CueId(CueId { bytes: num.bytes }),
        SrtLine::TimeRange(bs) => VttLine::TimeRangeAndStyle(TimeRangeAndStyle {
            bytes: bs.bytes,
            start: bs.start,
            end: bs.end,
        }),
        SrtLine::Text(bs) => VttLine::Text(Text { bytes: bs.bytes }),
    })
}

fn next_from_trans_state<'a>(
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<VttLine<'a>> {
    match *trans_state {
        TransIterState::Init => {
            *trans_state = TransIterState::Blank;
            Some(VttLine::VttFileMark(VttFileMark { bytes: b"WEBVTT" }))
        }
        TransIterState::Outside => None,
        TransIterState::TimeRange(start, end) => {
            *trans_state = TransIterState::Text(0);
            Some(VttLine::TimeRangeAndStyle(TimeRangeAndStyle {
                bytes: &[],
                start,
                end,
            }))
        }
        TransIterState::Text(start) => {
            if buf.is_empty() {
                return Some(VttLine::Text(Text { bytes: &[] }));
            }

            let mut end = start;
            let mut is_previous_sep = false;

            while end < buf.len() {
                match buf[end] {
                    b'\\' => {
                        is_previous_sep = true;
                        continue;
                    }
                    b'N' | b'n' => {
                        if is_previous_sep {
                            end -= 1;
                            break;
                        }
                    }
                    _ => (),
                }
                is_previous_sep = false;
                end += 1;
            }

            *trans_state = if end + 2 < buf.len() {
                TransIterState::Text(end + 2)
            } else {
                TransIterState::Blank
            };

            Some(VttLine::Text(Text {
                bytes: &buf[start..end],
            }))
        }
        TransIterState::Blank => {
            *trans_state = TransIterState::Outside;
            Some(VttLine::Blank)
        }
    }
}
