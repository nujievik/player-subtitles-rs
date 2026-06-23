use super::{
    AssLine, AssLines, AssSourceLines,
    line::{Event, EventFormat, ScriptInfo, SectionMark},
};
use crate::{ByteLines, SrtLine, SrtLines, Time, byte_helpers};
use std::io::BufRead;

use crate::StreamingIterator;

impl<T: BufRead> StreamingIterator for AssLines<'_, T> {
    type Item<'a>
        = AssLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut *self.source {
            AssSourceLines::Regular(_) => todo!(),
            AssSourceLines::Srt(lines) => next_srt(lines, &mut self.buf, &mut self.trans_state),
            AssSourceLines::Vtt(_) => todo!(),
        }
    }
}

pub enum IterState {
    Init,
    ScriptInfo,
    Blank,
    SectionMark(SectionMark),
    Styles,
    Events(EventFormat),
    Graphics,
    Unrecognized,
}

pub enum TransIterState {
    Init,
    Header(TransIterStateHeader),
    Blank,
    Events(TransIterStateEvents),
}
enum TransIterStateHeader {
    Title,
    ScriptType,
    WrapStyle,
}
enum TransIterStateEvents {
    Mark,
    Format,
    AfterFormat,
    TimeRange(Time, Time),
}

fn next_regular<'a, T: BufRead>(
    blines: &'a mut ByteLines<'_, T>,
    state: &mut IterState,
) -> Option<AssLine<'a>> {
    let line = {
        let mut line = blines.next()?;

        if let IterState::Init = state {
            *state = IterState::ScriptInfo;
            line = byte_helpers::trim_bom(line);
            line = byte_helpers::trim(line);
            if line == b"[Script Info]" {
                return Some(AssLine::SectionMark(SectionMark::ScriptInfo));
            }
        } else {
            line = byte_helpers::trim(line);
        }
        line
    };

    if line.is_empty() {
        *state = IterState::Blank;
        return Some(AssLine::Blank);
    }

    if line[0] == b'[' && *line.last().unwrap() == b']' {
        return Some(if let Some(mark) = SectionMark::get_from_bytes(line) {
            *state = IterState::SectionMark(mark);
            AssLine::SectionMark(mark)
        } else {
            *state = IterState::Unrecognized;
            AssLine::Unrecognized(line)
        });
    }

    if let IterState::Unrecognized = state {
        return Some(AssLine::Unrecognized(line));
    }

    if let IterState::ScriptInfo | IterState::SectionMark(SectionMark::ScriptInfo) = state {
        todo!();
    }

    if let IterState::SectionMark(SectionMark::V4Styles)
    | IterState::SectionMark(SectionMark::V4StylesPlus) = state
    {
        todo!();
    }

    if let IterState::SectionMark(SectionMark::Events) = state {
        todo!();
    }

    if let IterState::SectionMark(SectionMark::Fonts) = state {
        todo!();
    }

    if let IterState::SectionMark(SectionMark::Graphics) = state {
        todo!();
    }

    todo!();
}

fn next_srt<'a, T: BufRead>(
    srt_lines: &'a mut SrtLines<'_, T>,
    buf: &mut Vec<u8>,
    state: &mut TransIterState,
) -> Option<AssLine<'a>> {
    let line = match state {
        TransIterState::Init => {
            *state = TransIterState::Header(TransIterStateHeader::Title);
            AssLine::SectionMark(SectionMark::ScriptInfo)
        }
        TransIterState::Header(TransIterStateHeader::Title) => {
            *state = TransIterState::Header(TransIterStateHeader::ScriptType);
            AssLine::ScriptInfo(ScriptInfo::Title)
        }
        TransIterState::Header(TransIterStateHeader::ScriptType) => {
            *state = TransIterState::Header(TransIterStateHeader::WrapStyle);
            AssLine::ScriptInfo(ScriptInfo::ScriptType)
        }
        TransIterState::Header(TransIterStateHeader::WrapStyle) => {
            *state = TransIterState::Blank;
            AssLine::ScriptInfo(ScriptInfo::WrapStyle)
        }
        TransIterState::Blank => {
            *state = TransIterState::Events(TransIterStateEvents::Mark);
            AssLine::Blank
        }
        TransIterState::Events(TransIterStateEvents::Mark) => {
            *state = TransIterState::Events(TransIterStateEvents::Format);
            AssLine::SectionMark(SectionMark::Events)
        }
        TransIterState::Events(TransIterStateEvents::Format) => {
            *state = TransIterState::Events(TransIterStateEvents::AfterFormat);
            AssLine::EventFormat(EventFormat::new())
        }
        TransIterState::Events(TransIterStateEvents::AfterFormat) => {
            return next_srt_event(srt_lines, buf, state, Event::new(), false);
        }
        TransIterState::Events(TransIterStateEvents::TimeRange(start, end)) => {
            let mut event = Event::new();
            event.start = *start;
            event.end = *end;
            return next_srt_event(srt_lines, buf, state, event, true);
        }
    };
    Some(line)
}

fn next_srt_event<'a, T: BufRead>(
    srt_lines: &'a mut SrtLines<'_, T>,
    buf: &mut Vec<u8>,
    state: &mut TransIterState,
    mut event: Event<'a>,
    mut updated_times: bool,
) -> Option<AssLine<'a>> {
    buf.clear();
    let mut updated_text = false;

    while let Some(line) = srt_lines.next() {
        match line {
            SrtLine::Blank if updated_text => return Some(AssLine::Event(event)),
            SrtLine::Blank => {
                if updated_times {
                    updated_times = false;
                    event.start = Time::new_unchecked(0, 0, 0, 0);
                    event.end = Time::new_unchecked(0, 0, 0, 0);
                }
            }
            SrtLine::TimeRange(bs) => {
                if updated_text {
                    *state =
                        TransIterState::Events(TransIterStateEvents::TimeRange(bs.start, bs.end));
                    return Some(AssLine::Event(event));
                }
                event.start = bs.start;
                event.end = bs.end;
                updated_times = true;
                updated_text = false;
            }
            SrtLine::Text(bs) if updated_times => {
                buf.extend_from_slice(bs.bytes);
                updated_text = true;
            }
            _ => continue,
        }
    }
    updated_text.then(|| AssLine::Event(event))
}
