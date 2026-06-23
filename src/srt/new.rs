use super::{IterState, NewLines, SrtLines, SrtSourceLines};
use crate::{AssLines, ByteLines, VttLines, ass::AssSourceLines, vtt::VttSourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for SrtLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for SrtLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> SrtLines<'a, T> {
        Self::new_with_source(SrtSourceLines::Regular(byte_lines))
    }
}

impl<'a, T: BufRead> From<AssLines<'a, T>> for SrtLines<'a, T> {
    fn from(mut ass_lines: AssLines<'a, T>) -> SrtLines<'a, T> {
        match *ass_lines.source {
            AssSourceLines::Regular(byte_lines) => {
                ass_lines.source = Box::new(AssSourceLines::Regular(byte_lines));
                Self::new_with_source(SrtSourceLines::Ass(ass_lines))
            }
            AssSourceLines::Srt(lines) => lines,
            AssSourceLines::Vtt(vtt_lines) => Self::new_with_source(SrtSourceLines::Vtt(vtt_lines)),
        }
    }
}

impl<'a, T: BufRead> From<VttLines<'a, T>> for SrtLines<'a, T> {
    fn from(mut vtt_lines: VttLines<'a, T>) -> SrtLines<'a, T> {
        match *vtt_lines.source {
            VttSourceLines::Regular(byte_lines) => {
                vtt_lines.source = Box::new(VttSourceLines::Regular(byte_lines));
                Self::new_with_source(SrtSourceLines::Vtt(vtt_lines))
            }
            VttSourceLines::Ass(ass_lines) => Self::new_with_source(SrtSourceLines::Ass(ass_lines)),
            VttSourceLines::Srt(lines) => lines,
        }
    }
}

impl<'a, T: BufRead> SrtLines<'a, T> {
    #[inline(always)]
    fn new_with_source(src: SrtSourceLines<'a, T>) -> Self {
        Self {
            source: Box::new(src),
            buf: Vec::new(),
            state: IterState::Init,
        }
    }
}
