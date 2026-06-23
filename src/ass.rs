mod it;
pub mod line;
mod new;
mod write;

pub use line::AssLine;

use crate::{ByteLines, NewLines, Result, SrtLines, VttLines};
use it::{IterState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct AssLines<'a, T: BufRead> {
    pub(crate) source: Box<AssSourceLines<'a, T>>,
    buf: Vec<u8>,
    state: IterState,
    trans_state: TransIterState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<AssLines<'a, BufReader<File>>> {
    AssLines::open_file(path)
}

pub(crate) enum AssSourceLines<'a, T: BufRead> {
    Regular(ByteLines<'a, T>),
    Srt(SrtLines<'a, T>),
    Vtt(VttLines<'a, T>),
}
