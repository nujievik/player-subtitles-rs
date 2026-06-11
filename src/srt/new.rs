use super::{IterState, NewLines, SrtLines, SrtSourceLines};
use crate::{ByteLines, VttLines, vtt::VttSourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for SrtLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for SrtLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> SrtLines<'a, T> {
        SrtLines {
            source: Box::new(SrtSourceLines::Regular(byte_lines)),
            buf: Vec::new(),
            state: IterState::Init,
        }
    }
}

impl<'a, T: BufRead> From<VttLines<'a, T>> for SrtLines<'a, T> {
    fn from(mut vtt_lines: VttLines<'a, T>) -> SrtLines<'a, T> {
        match *vtt_lines.source {
            VttSourceLines::Regular(byte_lines) => {
                vtt_lines.source = Box::new(VttSourceLines::Regular(byte_lines));
                SrtLines {
                    source: Box::new(SrtSourceLines::Vtt(vtt_lines)),
                    buf: Vec::new(),
                    state: IterState::Init,
                }
            }
            VttSourceLines::Srt(lines) => lines,
        }
    }
}
