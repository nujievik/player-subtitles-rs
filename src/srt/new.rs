use super::{IterState, NewLines, RegularSrtLines, SrtLines, TransIterState};
use crate::{AssLines, ByteLines, SourceLines, VttLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for SrtLines<'a, T> {}
impl<'a, T: BufRead> NewLines<'a> for RegularSrtLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for SrtLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> SrtLines<'a, T> {
        Self::new_with_source(SourceLines::Srt(RegularSrtLines::from(byte_lines)))
    }
}
impl<'a, T: BufRead> From<ByteLines<'a, T>> for RegularSrtLines<'a, T> {
    fn from(lines: ByteLines<'a, T>) -> Self {
        Self {
            lines,
            state: IterState::Init,
        }
    }
}

impl<'a, T: BufRead> From<AssLines<'a, T>> for SrtLines<'a, T> {
    fn from(ass_lines: AssLines<'a, T>) -> SrtLines<'a, T> {
        Self::new_with_source(ass_lines.source)
    }
}

impl<'a, T: BufRead> From<VttLines<'a, T>> for SrtLines<'a, T> {
    fn from(vtt_lines: VttLines<'a, T>) -> SrtLines<'a, T> {
        Self::new_with_source(vtt_lines.source)
    }
}

impl<'a, T: BufRead> SrtLines<'a, T> {
    #[inline(always)]
    fn new_with_source(source: SourceLines<'a, T>) -> Self {
        Self {
            source,
            buf: Vec::new(),
            trans_state: TransIterState::Outside,
        }
    }
}
