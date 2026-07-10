//! A WebVTT subtitles module.

mod it;
mod line;
mod new;
mod write;

pub use line::VttLine;

use crate::{AssLines, ByteLines, NewLines, Result, SourceLines, SrtLines};
use it::{BodyState, CurrentState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct VttLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    body_state: BodyState,
    current_state: CurrentState,
}

pub(crate) struct RegularVttLines<'a, T: BufRead> {
    pub(crate) lines: ByteLines<'a, T>,
    pub(crate) body_state: BodyState,
    pub(crate) current_state: CurrentState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<VttLines<'a, BufReader<File>>> {
    VttLines::open_file(path)
}
