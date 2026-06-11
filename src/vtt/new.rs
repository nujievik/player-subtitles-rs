use super::{BodyState, ByteLines, CurrentState, SrtLines, VttLines, VttSourceLines};
use crate::{NewLines, srt::SrtSourceLines};
use std::io::BufRead;

impl<'a, T: BufRead> NewLines<'a> for VttLines<'a, T> {}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for VttLines<'a, T> {
    fn from(byte_lines: ByteLines<'a, T>) -> Self {
        VttLines {
            source: Box::new(VttSourceLines::Regular(byte_lines)),
            body_state: BodyState::Init,
            current_state: CurrentState::Outside,
        }
    }
}

impl<'a, T: BufRead> From<SrtLines<'a, T>> for VttLines<'a, T> {
    fn from(mut srt_lines: SrtLines<'a, T>) -> Self {
        match *srt_lines.source {
            SrtSourceLines::Regular(byte_lines) => {
                srt_lines.source = Box::new(SrtSourceLines::Regular(byte_lines));
                VttLines {
                    source: Box::new(VttSourceLines::Srt(srt_lines)),
                    body_state: BodyState::Init,
                    current_state: CurrentState::Outside,
                }
            }
            SrtSourceLines::Vtt(lines) => lines,
        }
    }
}
