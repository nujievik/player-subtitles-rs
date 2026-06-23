use super::{
    SrtLine, SrtLines, SrtSourceLines,
    line::{BytesText, BytesTimeRange},
};
use crate::{AssLines, ByteLines, StreamingIterator, VttLine, VttLines, byte_helpers};
use std::io::BufRead;

impl<T: BufRead> StreamingIterator for SrtLines<'_, T> {
    type Item<'a>
        = SrtLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut *self.source {
            SrtSourceLines::Regular(lines) => next_regular(lines, &mut self.state),
            SrtSourceLines::Ass(ass_lines) => todo!(),
            SrtSourceLines::Vtt(lines) => next_vtt(lines, &mut self.buf),
        }
    }
}

pub enum IterState {
    Init,
    Outside,
    InBlock,
}

fn next_regular<'a, T: BufRead>(
    lines: &'a mut ByteLines<'_, T>,
    state: &mut IterState,
) -> Option<SrtLine<'a>> {
    let mut line = lines.next()?;
    if let IterState::Init = state {
        *state = IterState::Outside;
        line = byte_helpers::trim_bom(line);
    }

    let mut line = SrtLine::new(line);
    let mut is_number_text = false;

    match &line {
        SrtLine::Blank => *state = IterState::Outside,
        SrtLine::Number(_) => {
            if let IterState::InBlock = state {
                is_number_text = true;
            }
        }
        SrtLine::TimeRange(_) => *state = IterState::InBlock,
        _ => {}
    }

    if is_number_text {
        if let SrtLine::Number(bs) = &mut line {
            line = SrtLine::Text(BytesText { bytes: bs.bytes });
        }
    }

    Some(line)
}

fn next_ass<'a, T: BufRead>(lines: &'a AssLines<'_, T>) -> Option<SrtLine<'a>> {
    todo!();
}

fn next_vtt<'a, T: BufRead>(
    lines: &'a mut VttLines<'_, T>,
    buf: &'a mut Vec<u8>,
) -> Option<SrtLine<'a>> {
    let line = lines.find(|l| {
        matches!(
            l,
            VttLine::Blank | VttLine::TimeRangeAndStyle(_) | VttLine::Text(_)
        )
    })?;
    let srt_line = match line {
        VttLine::Blank => SrtLine::Blank,
        VttLine::TimeRangeAndStyle(tr) => {
            let start = tr.start;
            let end = tr.end;
            SrtLine::TimeRange(BytesTimeRange {
                bytes: &[],
                start,
                end,
            })
        }
        VttLine::Text(x) => {
            let mut parts = x.text();
            if parts.next().is_some_and(|p| p == x.bytes) {
                SrtLine::Text(BytesText { bytes: x.bytes })
            } else {
                buf.clear();
                for p in x.text() {
                    buf.extend_from_slice(p);
                    buf.push(b' ');
                }
                buf.pop();
                SrtLine::Text(BytesText { bytes: &buf[..] })
            }
        }
        _ => return None,
    };
    Some(srt_line)
}
