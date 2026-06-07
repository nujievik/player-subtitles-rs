use super::line::{BytesNumber, BytesText};
use crate::{Result, SrtLine, SrtLines, StreamingIterator, Time, WriteOptions};
use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::Path,
};

impl<'a, T: BufRead> SrtLines<'a, T> {
    /// Writes all correct time-and-text blocks to a file, skipping incorrect/unrecognized.
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn write<P>(&mut self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer(&mut writer)
    }

    /// Writes all correct time-and-text blocks to a file, skipping incorrect/unrecognized,
    /// with additional [`WriteOptions`].
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn write_with<P>(&mut self, path: &P, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer_with(&mut writer, opts)
    }

    /// Writes all correct time-and-text blocks to a writer, skipping incorrect/unrecognized.
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn write_to_writer<W>(&mut self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        static DEFAULT_OPTIONS: WriteOptions = WriteOptions::new();
        self.write_to_writer_with(writer, &DEFAULT_OPTIONS)
    }

    /// Writes all correct time-and-text blocks to a writer, skipping incorrect/unrecognized,
    /// with additional [`WriteOptions`].
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(crate::BOM)?;
        }

        let mut number = 1usize;
        let mut is_wrote_header = false;
        let mut time_range: Option<(Time, Time)> = None;

        while let Some(line) = self.next() {
            match line {
                SrtLine::Blank => {
                    is_wrote_header = false;
                    time_range = None;
                }
                SrtLine::TimeRange(bs) => {
                    let mut start = bs.start();
                    let mut end = bs.end();

                    if opts.start_from.is_some_and(|t| end <= t) {
                        continue;
                    }
                    if opts.end_on.is_some_and(|t| start >= t) {
                        continue;
                    }
                    if let Some(add) = opts.add_time {
                        start += add;
                        end += add;
                    }
                    if let Some(sub) = opts.sub_time {
                        start -= sub;
                        end -= sub;
                    }

                    time_range = Some((start, end));
                }
                SrtLine::Number(BytesNumber { bytes }) | SrtLine::Text(BytesText { bytes }) => {
                    if !is_wrote_header {
                        if let Some((start, end)) = time_range {
                            if number > 1 {
                                writer.write("\n".as_bytes())?;
                            }
                            writer.write(format!("{}\n", number).as_bytes())?;
                            writer.write(
                                format!("{} --> {}\n", start.to_srt(), end.to_srt()).as_bytes(),
                            )?;
                            number += 1;
                            is_wrote_header = true;
                        } else {
                            continue;
                        }
                    }
                    writer.write(bytes)?;
                    writer.write("\n".as_bytes())?;
                }
            }
        }

        Ok(())
    }
}
