use super::line::{
    AssLine, Comment, Event, EventFormat, ScriptInfo, ScriptType, SectionMark, WrapStyle,
};
use super::{AssLines, RegularAssLines};
use crate::{
    ByteLines, RegularSrtLines, RegularVttLines, SourceLines, SrtLine, StreamingIterator, Time,
    VttLine, byte_helpers,
};
use std::io::BufRead;

impl<T: BufRead> StreamingIterator for AssLines<'_, T> {
    type Item<'a>
        = AssLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.source {
            SourceLines::Ass(lines) => lines.next(),
            SourceLines::Srt(lines) => next_from_srt(lines, &mut self.buf, &mut self.trans_state),
            SourceLines::Vtt(lines) => next_from_vtt(lines, &mut self.buf, &mut self.trans_state),
        }
    }
}

impl<T: BufRead> StreamingIterator for RegularAssLines<'_, T> {
    type Item<'a>
        = AssLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        next_regular(&mut self.lines, &mut self.state)
    }
}

#[derive(Debug)]
pub enum IterState {
    Init,
    ScriptInfo,
    Blank,
    SectionMark(SectionMark),
    Events(EventFormat),
    Unrecognized,
}

#[derive(Debug)]
pub enum TransIterState {
    Init,
    Header(TransIterStateHeader),
    Blank,
    Events(TransIterStateEvents),
}
#[derive(Debug)]
pub enum TransIterStateHeader {
    ScriptType,
    WrapStyle,
}
#[derive(Debug)]
pub enum TransIterStateEvents {
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

    if let Some(comment) = Comment::get_new(line) {
        return Some(AssLine::Comment(comment));
    }

    if line[0] == b'[' && *line.last().unwrap() == b']' {
        return Some(
            if let Some(mark) = SectionMark::get_from_bytes(&line[1..line.len() - 1]) {
                *state = IterState::SectionMark(mark);
                AssLine::SectionMark(mark)
            } else {
                *state = IterState::Unrecognized;
                AssLine::Unrecognized(line)
            },
        );
    }

    if let IterState::Unrecognized = state {
        return Some(AssLine::Unrecognized(line));
    }

    if let IterState::ScriptInfo | IterState::SectionMark(SectionMark::ScriptInfo) = state {
        return Some(if let Some(info) = ScriptInfo::get_new(line) {
            *state = IterState::ScriptInfo;
            AssLine::ScriptInfo(info)
        } else {
            *state = IterState::ScriptInfo;
            AssLine::Unrecognized(line)
        });
    }

    if let IterState::SectionMark(mark) = state {
        if let SectionMark::Events = mark {
            return Some(if let Some(format) = EventFormat::get_new(line) {
                *state = IterState::Events(format);
                AssLine::EventFormat(format)
            } else {
                *state = IterState::Unrecognized;
                AssLine::Unrecognized(line)
            });
        }
    }

    if let IterState::Events(format) = state {
        return Some(if let Some(event) = Event::get_new(line, *format) {
            AssLine::Event(event)
        } else {
            *state = IterState::Unrecognized;
            AssLine::Unrecognized(line)
        });
    }

    *state = IterState::Unrecognized;
    Some(AssLine::Unrecognized(line))
}

fn next_from_srt<'a, T: BufRead>(
    srt_lines: &'a mut RegularSrtLines<'_, T>,
    buf: &'a mut Vec<u8>,
    state: &mut TransIterState,
) -> Option<AssLine<'a>> {
    let line = match state {
        TransIterState::Init => {
            *state = TransIterState::Header(TransIterStateHeader::ScriptType);
            AssLine::SectionMark(SectionMark::ScriptInfo)
        }
        TransIterState::Header(TransIterStateHeader::ScriptType) => {
            *state = TransIterState::Header(TransIterStateHeader::WrapStyle);
            AssLine::ScriptInfo(ScriptInfo::ScriptType(ScriptType {
                bytes: b"ScriptType: v4.00+",
            }))
        }
        TransIterState::Header(TransIterStateHeader::WrapStyle) => {
            *state = TransIterState::Blank;
            AssLine::ScriptInfo(ScriptInfo::WrapStyle(WrapStyle {
                bytes: b"WrapStyle: 0",
            }))
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
            return next_from_srt_event(srt_lines, buf, state, Event::new(), false);
        }
        TransIterState::Events(TransIterStateEvents::TimeRange(start, end)) => {
            let mut event = Event::new();
            event.start = *start;
            event.end = *end;
            return next_from_srt_event(srt_lines, buf, state, event, true);
        }
    };
    Some(line)
}

fn next_from_srt_event<'a, T: BufRead>(
    srt_lines: &'a mut RegularSrtLines<'_, T>,
    buf: &'a mut Vec<u8>,
    state: &mut TransIterState,
    mut event: Event<'a>,
    mut updated_times: bool,
) -> Option<AssLine<'a>> {
    buf.clear();
    let mut updated_text = false;

    while let Some(line) = srt_lines.next() {
        match line {
            SrtLine::Blank if updated_text => break,
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
                if updated_text {
                    buf.push(b'\\');
                    buf.push(b'n');
                }
                buf.extend_from_slice(bs.bytes);
                updated_text = true;
            }
            _ => continue,
        }
    }

    updated_text.then(|| {
        event.text = buf.as_slice();
        AssLine::Event(event)
    })
}

fn next_from_vtt<'a, T: BufRead>(
    vtt_lines: &'a mut RegularVttLines<'_, T>,
    buf: &'a mut Vec<u8>,
    state: &mut TransIterState,
) -> Option<AssLine<'a>> {
    let line = match state {
        TransIterState::Init => {
            *state = TransIterState::Header(TransIterStateHeader::ScriptType);
            AssLine::SectionMark(SectionMark::ScriptInfo)
        }
        TransIterState::Header(TransIterStateHeader::ScriptType) => {
            *state = TransIterState::Header(TransIterStateHeader::WrapStyle);
            AssLine::ScriptInfo(ScriptInfo::ScriptType(ScriptType {
                bytes: b"ScriptType: v4.00+",
            }))
        }
        TransIterState::Header(TransIterStateHeader::WrapStyle) => {
            *state = TransIterState::Blank;
            AssLine::ScriptInfo(ScriptInfo::WrapStyle(WrapStyle {
                bytes: b"WrapStyle: 0",
            }))
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
            return next_from_vtt_event(vtt_lines, buf, state, Event::new(), false);
        }
        TransIterState::Events(TransIterStateEvents::TimeRange(start, end)) => {
            let mut event = Event::new();
            event.start = *start;
            event.end = *end;
            return next_from_vtt_event(vtt_lines, buf, state, event, true);
        }
    };
    Some(line)
}

fn next_from_vtt_event<'a, T: BufRead>(
    vtt_lines: &'a mut RegularVttLines<'_, T>,
    buf: &'a mut Vec<u8>,
    state: &mut TransIterState,
    mut event: Event<'a>,
    mut updated_times: bool,
) -> Option<AssLine<'a>> {
    buf.clear();
    let mut updated_text = false;

    while let Some(line) = vtt_lines.next() {
        match line {
            VttLine::Blank if updated_text => break,
            VttLine::Blank => {
                if updated_times {
                    updated_times = false;
                    event.start = Time::new_unchecked(0, 0, 0, 0);
                    event.end = Time::new_unchecked(0, 0, 0, 0);
                }
            }
            VttLine::TimeRangeAndStyle(bs) => {
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
            VttLine::Text(bs) if updated_times => {
                if updated_text {
                    buf.push(b'\\');
                    buf.push(b'n');
                }
                buf.extend_from_slice(bs.bytes);
                updated_text = true;
            }
            _ => continue,
        }
    }

    updated_text.then(|| {
        event.text = buf.as_slice();
        AssLine::Event(event)
    })
}
