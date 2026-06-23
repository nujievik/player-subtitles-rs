use super::{BodyState, ByteLines, CurrentState, SrtLines, VttLines, VttSourceLines};
use crate::{AssLines, NewLines, ass::AssSourceLines, srt::SrtSourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for VttLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for VttLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> Self {
        Self::new_with_source(VttSourceLines::Regular(byte_lines))
    }
}

impl<'a, T: BufRead> From<AssLines<'a, T>> for VttLines<'a, T> {
    fn from(mut ass_lines: AssLines<'a, T>) -> Self {
        match *ass_lines.source {
            AssSourceLines::Regular(byte_lines) => {
                ass_lines.source = Box::new(AssSourceLines::Regular(byte_lines));
                Self::new_with_source(VttSourceLines::Ass(ass_lines))
            }
            AssSourceLines::Srt(srt_lines) => Self::new_with_source(VttSourceLines::Srt(srt_lines)),
            AssSourceLines::Vtt(lines) => lines,
        }
    }
}

impl<'a, T: BufRead> From<SrtLines<'a, T>> for VttLines<'a, T> {
    fn from(mut srt_lines: SrtLines<'a, T>) -> Self {
        match *srt_lines.source {
            SrtSourceLines::Regular(byte_lines) => {
                srt_lines.source = Box::new(SrtSourceLines::Regular(byte_lines));
                Self::new_with_source(VttSourceLines::Srt(srt_lines))
            }
            SrtSourceLines::Ass(ass_lines) => Self::new_with_source(VttSourceLines::Ass(ass_lines)),
            SrtSourceLines::Vtt(lines) => lines,
        }
    }
}

impl<'a, T: BufRead> VttLines<'a, T> {
    #[inline(always)]
    fn new_with_source(src: VttSourceLines<'a, T>) -> Self {
        Self {
            source: Box::new(src),
            body_state: BodyState::Init,
            current_state: CurrentState::Outside,
        }
    }
}
