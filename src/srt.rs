//! A SubRip subtitles module.

pub mod line;
mod write;

use crate::{ByteLines, Result, StreamingIterator, byte_helpers};
pub use line::SrtLine;
use std::{
    fs::File,
    io::{BufRead, BufReader, Empty},
    path::Path,
};

pub struct SrtLines<'a, T: BufRead> {
    blines: ByteLines<'a, T>,
    is_first_taken: bool,
}

impl<T: BufRead> StreamingIterator for SrtLines<'_, T> {
    type Item<'a>
        = SrtLine<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        self.blines.next().map(|l| {
            if self.is_first_taken {
                SrtLine::new(l)
            } else {
                self.is_first_taken = true;
                SrtLine::new(byte_helpers::trim_bom(l))
            }
        })
    }
}

impl<'a, T: BufRead> From<ByteLines<'a, T>> for SrtLines<'a, T> {
    fn from(blines: ByteLines<'a, T>) -> SrtLines<'a, T> {
        SrtLines {
            blines,
            is_first_taken: false,
        }
    }
}

impl<'a> SrtLines<'a, Empty> {
    pub fn from_bytes<B>(bytes: &'a B) -> Self
    where
        B: AsRef<[u8]> + ?Sized,
    {
        ByteLines::from_bytes(bytes).into()
    }

    pub fn from_str<S>(s: &'a S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        Self::from_bytes(s.as_ref())
    }
}

impl<'a, T: BufRead> SrtLines<'a, T> {
    pub fn from_reader(reader: T) -> Self {
        ByteLines::from_reader(reader).into()
    }
}

impl<'a> SrtLines<'a, BufReader<File>> {
    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path)?;
        Ok(SrtLines::from_reader(BufReader::new(f)))
    }
}

pub fn open_file<'a, P: AsRef<Path>>(path: P) -> Result<SrtLines<'a, BufReader<File>>> {
    SrtLines::open_file(path)
}
