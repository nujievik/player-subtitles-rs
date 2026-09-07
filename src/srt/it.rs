use super::line::{Number, SrtLine, Text, TimeRange};
use super::{RegularSrtLines, SrtLines};
use crate::{
    AssLine, RegularAssLines, RegularVttLines, SourceLines, StreamingIterator, Time, VttLine,
    byte_helpers,
};
use std::io::BufRead;

impl<T: BufRead> StreamingIterator for SrtLines<'_, T> {
    type Item<'a>
        = SrtLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.source {
            SourceLines::Ass(lines) => next_from_ass(lines, &mut self.buf, &mut self.trans_state),
            SourceLines::Srt(lines) => lines.next(),
            SourceLines::Vtt(lines) => next_from_vtt(lines, &mut self.buf, &mut self.trans_state),
        }
    }
}

impl<T: BufRead> StreamingIterator for RegularSrtLines<'_, T> {
    type Item<'a>
        = SrtLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        let mut line = self.lines.next()?;
        if let IterState::Init = self.state {
            self.state = IterState::Outside;
            line = byte_helpers::trim_bom(line);
        }

        let mut line = SrtLine::new(line);

        match &line {
            SrtLine::Blank => self.state = IterState::Outside,
            SrtLine::TimeRange(_) => self.state = IterState::InBlock,
            _ => {}
        }

        if let IterState::InBlock = self.state {
            if let SrtLine::Number(bs) = &mut line {
                line = SrtLine::Text(Text { bytes: bs.bytes });
            }
        }

        Some(line)
    }
}

#[derive(Debug)]
pub enum IterState {
    Init,
    Outside,
    InBlock,
}

#[derive(Debug)]
pub enum TransIterState {
    Outside,
    TimeRange(Time, Time),
    Text(usize),
    Blank,
}

fn next_from_ass<'a, T: BufRead>(
    lines: &mut RegularAssLines<'_, T>,
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<SrtLine<'a>> {
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

    Some(SrtLine::Number(Number { bytes: &[] }))
}

fn next_from_vtt<'a, T: BufRead>(
    lines: &'a mut RegularVttLines<'_, T>,
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<SrtLine<'a>> {
    let fake_buf = unsafe { &mut *(buf as *mut Vec<u8>) };

    if let Some(line) = next_from_trans_state(fake_buf, trans_state) {
        return Some(line);
    }

    let mut time_range: Option<(Time, Time)> = None;
    buf.clear();

    while let Some(l) = lines.next() {
        match l {
            VttLine::TimeRangeAndStyle(tr) => time_range = Some((tr.start, tr.end)),
            VttLine::Text(text) if time_range.is_some() => {
                // newline
                if !buf.is_empty() {
                    buf.push(b'\\');
                    buf.push(b'N');
                }

                for text in text.text() {
                    buf.extend_from_slice(text);
                    buf.push(b' ');
                }
            }
            _ if time_range.is_some() && !buf.is_empty() => break,
            _ => time_range = None,
        }
    }

    match time_range {
        Some((start, end)) if !buf.is_empty() => {
            *trans_state = TransIterState::TimeRange(start, end);
            Some(SrtLine::Number(Number { bytes: &[] }))
        }
        _ => None,
    }
}

fn next_from_trans_state<'a>(
    buf: &'a mut Vec<u8>,
    trans_state: &mut TransIterState,
) -> Option<SrtLine<'a>> {
    match *trans_state {
        TransIterState::Outside => None,
        TransIterState::TimeRange(start, end) => {
            *trans_state = TransIterState::Text(0);
            Some(SrtLine::TimeRange(TimeRange {
                bytes: &[],
                start,
                end,
            }))
        }
        TransIterState::Text(start) => {
            if buf.is_empty() {
                return Some(SrtLine::Text(Text { bytes: &[] }));
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
                            end -= 2;
                            break;
                        }
                    }
                    _ => (),
                }
                is_previous_sep = false;
            }

            *trans_state = if end + 2 < buf.len() {
                TransIterState::Text(end + 2)
            } else {
                TransIterState::Blank
            };

            Some(SrtLine::Text(Text {
                bytes: &buf[start..=end],
            }))
        }
        TransIterState::Blank => {
            *trans_state = TransIterState::Outside;
            Some(SrtLine::Blank)
        }
    }
}
