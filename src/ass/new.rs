use super::{AssLines, AssSourceLines, IterState, TransIterState};
use crate::{ByteLines, NewLines, SrtLines, VttLines, srt::SrtSourceLines, vtt::VttSourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for AssLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for AssLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> AssLines<'a, T> {
        Self::new_with_source(AssSourceLines::Regular(byte_lines))
    }
}

impl<'a, T: BufRead> From<SrtLines<'a, T>> for AssLines<'a, T> {
    fn from(mut srt_lines: SrtLines<'a, T>) -> AssLines<'a, T> {
        match *srt_lines.source {
            SrtSourceLines::Regular(byte_lines) => {
                srt_lines.source = Box::new(SrtSourceLines::Regular(byte_lines));
                Self::new_with_source(AssSourceLines::Srt(srt_lines))
            }
            SrtSourceLines::Ass(ass_lines) => ass_lines,
            SrtSourceLines::Vtt(vtt_lines) => Self::new_with_source(AssSourceLines::Vtt(vtt_lines)),
        }
    }
}

impl<'a, T: BufRead> From<VttLines<'a, T>> for AssLines<'a, T> {
    fn from(mut vtt_lines: VttLines<'a, T>) -> AssLines<'a, T> {
        match *vtt_lines.source {
            VttSourceLines::Regular(byte_lines) => {
                vtt_lines.source = Box::new(VttSourceLines::Regular(byte_lines));
                Self::new_with_source(AssSourceLines::Vtt(vtt_lines))
            }
            VttSourceLines::Ass(ass_lines) => ass_lines,
            VttSourceLines::Srt(srt_lines) => Self::new_with_source(AssSourceLines::Srt(srt_lines)),
        }
    }
}

impl<'a, T: BufRead> AssLines<'a, T> {
    #[inline(always)]
    fn new_with_source(source: AssSourceLines<'a, T>) -> Self {
        Self {
            source: Box::new(source),
            buf: Vec::new(),
            state: IterState::Init,
            trans_state: TransIterState::Init,
        }
    }
}
