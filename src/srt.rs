//! A SubRip subtitles module.

mod it;
pub mod line;
mod new;
mod write;

pub use line::{BytesText, BytesTimeRange, SrtLine};

use crate::{AssLines, ByteLines, NewLines, Result, VttLines};
use it::IterState;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct SrtLines<'a, T: BufRead> {
    pub(crate) source: Box<SrtSourceLines<'a, T>>,
    pub(crate) buf: Vec<u8>,
    pub(crate) state: IterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<SrtLines<'a, BufReader<File>>> {
    SrtLines::open_file(path)
}

pub(crate) enum SrtSourceLines<'a, T: BufRead> {
    Regular(ByteLines<'a, T>),
    Ass(AssLines<'a, T>),
    Vtt(VttLines<'a, T>),
}
