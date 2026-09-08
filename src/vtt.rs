//! A WebVTT subtitles module.

pub mod line;

mod it;
mod new;
mod write;

use crate::{ByteLines, NewLines, Result, SourceLines, SrtLines};
use it::{BodyState, CurrentState, TransIterState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct VttLines<'a, T: BufRead> {
    pub(crate) source: SourceLines<'a, T>,
    buf: Vec<u8>,
    trans_state: TransIterState,
}

#[derive(Debug)]
pub(crate) struct RegularVttLines<'a, T: BufRead> {
    pub(crate) lines: ByteLines<'a, T>,
    pub(crate) body_state: BodyState,
    pub(crate) current_state: CurrentState,
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<VttLines<'a, BufReader<File>>> {
    VttLines::open_file(path)
}
