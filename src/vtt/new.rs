use super::{
    BodyState, ByteLines, CurrentState, RegularVttLines, SrtLines, TransIterState, VttLines,
};
use crate::{AssLines, NewLines, SourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for VttLines<'a, T> {}
impl<'a, T: BufRead> NewLines<'a> for RegularVttLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for VttLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> Self {
        Self::new_with_source(SourceLines::Vtt(RegularVttLines::from(byte_lines)))
    }
}
impl<'a, T: BufRead> From<ByteLines<'a, T>> for RegularVttLines<'a, T> {
    fn from(lines: ByteLines<'a, T>) -> Self {
        Self {
            lines,
            body_state: BodyState::Init,
            current_state: CurrentState::Outside,
        }
    }
}

impl<'a, T: BufRead> From<AssLines<'a, T>> for VttLines<'a, T> {
    fn from(ass_lines: AssLines<'a, T>) -> Self {
        Self::new_with_source(ass_lines.source)
    }
}

impl<'a, T: BufRead> From<SrtLines<'a, T>> for VttLines<'a, T> {
    fn from(srt_lines: SrtLines<'a, T>) -> Self {
        Self::new_with_source(srt_lines.source)
    }
}

impl<'a, T: BufRead> VttLines<'a, T> {
    #[inline(always)]
    fn new_with_source(source: SourceLines<'a, T>) -> Self {
        Self {
            source,
            buf: Vec::new(),
            body_state: BodyState::Init,
            current_state: CurrentState::Outside,
            trans_state: TransIterState::Outside,
        }
    }
}
