use super::{AssLines, IterState, RegularAssLines, TransIterState};
use crate::{ByteLines, NewLines, SourceLines, SrtLines, VttLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for AssLines<'a, T> {}
impl<'a, T: BufRead> NewLines<'a> for RegularAssLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for AssLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> AssLines<'a, T> {
        Self::new_with_source(SourceLines::Ass(RegularAssLines::from(byte_lines)))
    }
}
impl<'a, T: BufRead> From<ByteLines<'a, T>> for RegularAssLines<'a, T> {
    fn from(lines: ByteLines<'a, T>) -> RegularAssLines<'a, T> {
        Self {
            lines,
            state: IterState::Init,
        }
    }
}

impl<'a, T: BufRead> From<SrtLines<'a, T>> for AssLines<'a, T> {
    fn from(srt_lines: SrtLines<'a, T>) -> AssLines<'a, T> {
        Self::new_with_source(srt_lines.source)
    }
}

impl<'a, T: BufRead> From<VttLines<'a, T>> for AssLines<'a, T> {
    fn from(mut vtt_lines: VttLines<'a, T>) -> AssLines<'a, T> {
        Self::new_with_source(vtt_lines.source)
    }
}

impl<'a, T: BufRead> AssLines<'a, T> {
    #[inline(always)]
    fn new_with_source(source: SourceLines<'a, T>) -> Self {
        Self {
            source,
            buf: Vec::new(),
            state: IterState::Init,
            trans_state: TransIterState::Init,
        }
    }
}
