mod it;
pub mod line;
mod new;
mod write;

pub use line::AssLine;

use crate::{ByteLines, NewLines, Result, SourceLines, SrtLines, VttLines};
use it::{IterState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct AssLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    buf: Vec<u8>,
    state: IterState,
    trans_state: TransIterState,
}

pub(crate) struct RegularAssLines<'a, T: BufRead> {
    pub(crate) lines: ByteLines<'a, T>,
    pub(crate) state: IterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<AssLines<'a, BufReader<File>>> {
    AssLines::open_file(path)
}
