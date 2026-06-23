//! A WebVTT subtitles module.

mod it;
mod line;
mod new;
mod write;

pub use line::VttLine;

use crate::{AssLines, ByteLines, NewLines, Result, SrtLines};
use it::{BodyState, CurrentState};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct VttLines<'a, T: BufRead> {
    pub(crate) source: Box<VttSourceLines<'a, T>>,
    body_state: BodyState,
    current_state: CurrentState,
}

impl<'a, T: BufRead> VttLines<'a, T> {
    pub fn into_srt(self) -> SrtLines<'a, T> {
        SrtLines::from(self)
    }
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<VttLines<'a, BufReader<File>>> {
    VttLines::open_file(path)
}

pub(crate) enum VttSourceLines<'a, T: BufRead> {
    Regular(ByteLines<'a, T>),
    Ass(AssLines<'a, T>),
    Srt(SrtLines<'a, T>),
}
