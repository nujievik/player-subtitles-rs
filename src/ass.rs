pub mod line;

mod it;
mod new;
mod time;
mod write;

use crate::{ByteLines, NewLines, Result, SourceLines};
use it::{IterState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct AssLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    buf: Vec<u8>,
    trans_state: TransIterState,
}

#[derive(Debug)]
pub(crate) struct RegularAssLines<'a, T: BufRead> {
    pub(crate) lines: ByteLines<'a, T>,
    pub(crate) state: IterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<AssLines<'a, BufReader<File>>> {
    AssLines::open_file(path)
}
