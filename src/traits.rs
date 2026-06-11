use crate::{ByteLines, Result, WriteOptions};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Empty, Write},
    path::Path,
};

pub trait NewLines<'a> {
    fn from_bytes<B>(bytes: &'a B) -> Self
    where
        Self: From<ByteLines<'a, Empty>>,
        B: AsRef<[u8]> + ?Sized,
    {
        ByteLines::from_bytes(bytes).into()
    }

    fn from_str<S>(s: &'a S) -> Self
    where
        Self: From<ByteLines<'a, Empty>>,
        S: AsRef<str> + ?Sized,
    {
        Self::from_bytes(s.as_ref())
    }

    fn from_reader<R: BufRead>(reader: R) -> Self
    where
        Self: From<ByteLines<'a, R>>,
    {
        ByteLines::from_reader(reader).into()
    }

    fn open_file<P: AsRef<Path>>(path: P) -> Result<Self>
    where
        Self: From<ByteLines<'a, BufReader<File>>>,
    {
        let f = File::open(path)?;
        Ok(Self::from_reader(BufReader::new(f)))
    }
}

pub trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;

    fn find<'a, F>(&'a mut self, mut predicate: F) -> Option<Self::Item<'a>>
    where
        F: FnMut(&Self::Item<'a>) -> bool,
    {
        let this = self as *mut Self;

        while let Some(item) = unsafe { (&mut *this).next() } {
            if predicate(&item) {
                return Some(item);
            }
        }
        None
    }
}

pub trait WriteLines {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized;

    fn write<P>(&mut self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer(&mut writer)
    }

    fn write_with<P>(&mut self, path: &P, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer_with(&mut writer, opts)
    }

    fn write_to_writer<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        static DEFAULT_OPTIONS: WriteOptions = WriteOptions::new();
        self.write_to_writer_with(writer, &DEFAULT_OPTIONS)
    }
}
