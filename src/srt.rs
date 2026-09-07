//! A SubRip subtitles module.

pub(crate) mod it;
pub mod line;
mod new;
mod write;

pub use line::{BytesText, BytesTimeRange, SrtLine};

use crate::{AssLines, ByteLines, NewLines, Result, SourceLines, VttLines};
use it::{IterState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct SrtLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    pub(crate) buf: Vec<u8>,
    pub(crate) trans_state: TransIterState,
}

pub(crate) struct RegularSrtLines<'a, T: BufRead> {
    lines: ByteLines<'a, T>,
    state: IterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<SrtLines<'a, BufReader<File>>> {
    SrtLines::open_file(path)
}
