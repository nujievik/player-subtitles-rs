//! A SubRip subtitles module.

pub mod line;

pub(crate) mod it;
mod new;
mod write;

use crate::{ByteLines, NewLines, Result, SourceLines};
use it::{IterState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct SrtLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    pub(crate) buf: Vec<u8>,
    pub(crate) trans_state: TransIterState,
}

#[derive(Debug)]
pub(crate) struct RegularSrtLines<'a, T: BufRead> {
    lines: ByteLines<'a, T>,
    state: IterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<SrtLines<'a, BufReader<File>>> {
    SrtLines::open_file(path)
}
