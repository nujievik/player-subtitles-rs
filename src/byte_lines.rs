mod it;

use crate::Result;
use std::{
    fs::File,
    io::{BufRead, BufReader, Empty},
    path::Path,
};

#[derive(Debug)]
pub struct ByteLines<'a, T: BufRead> {
    ty: ByteLinesTy<'a, T>,
    pos: usize,
    buf: Vec<u8>,
}

#[derive(Debug)]
enum ByteLinesTy<'a, T: BufRead> {
    ByteSlice(&'a [u8]),
    BufReader(T),
}

impl<'a> ByteLines<'a, Empty> {
    pub fn from_bytes<B>(bytes: &'a B) -> Self
    where
        B: AsRef<[u8]> + ?Sized,
    {
        Self {
            ty: ByteLinesTy::ByteSlice(bytes.as_ref()),
            pos: 0,
            buf: Vec::new(),
        }
    }

    pub fn from_str<S>(s: &'a S) -> Self
    where
        S: AsRef<str> + ?Sized,
    {
        Self::from_bytes(s.as_ref())
    }
}

impl<'a, T: BufRead> ByteLines<'a, T> {
    pub fn from_reader(reader: T) -> Self {
        Self {
            ty: ByteLinesTy::BufReader(reader),
            pos: 0,
            buf: Vec::new(),
        }
    }
}

impl<'a> ByteLines<'a, BufReader<File>> {
    pub fn open_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let f = File::open(path)?;
        Ok(ByteLines::from_reader(BufReader::new(f)))
    }
}
