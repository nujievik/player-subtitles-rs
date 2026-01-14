use super::{SrtBlock, SrtLine, SrtLineType, SrtSubtitles, new};
use crate::{BOM, Result, WriteOptions};
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

impl SrtSubtitles<'_> {
    /// Writes all lines of subtitles as the entire contents of a file.
    ///
    /// Writes lines in 'as is' order does not checks. Use [`SrtSubtitles::standardize`] for
    /// ensures correct.
    ///
    /// Writes:
    /// - A [`SrtLineType::Blank`] as `\n`
    /// - A [`SrtLineType::Number`] as `{num}\n`
    /// - A [`SrtLineType::TimeRange((start, end))`] as `{start} --> {end}\n`
    /// - A [`SrtLineType::Text`] as `{text}\n`
    ///
    /// Ensures monotone increase index (number) of output block from 1.
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn write<P>(&self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer(&mut writer)
    }

    /// Same as [`SrtSubtitles::write`] with additional [`WriteOptions`].
    pub fn write_with<P>(&self, path: &P, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.write_to_writer_with(&mut writer, opts)
    }

    /// Writes all raw bytes of subtitles as the entire contents of a file.
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn raw_write<P>(&self, path: &P) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
    {
        let ofile = File::create(path.as_ref())?;
        let mut writer = BufWriter::new(ofile);
        self.raw_write_to_writer(&mut writer)
    }

    /// Per-block read and write with options.
    ///
    /// # Errors
    /// - Returns an I/O error.
    /// - Also returns validation error with some options.
    pub fn read_and_write_with<P, Q>(from: &P, to: &Q, opts: &WriteOptions) -> Result<()>
    where
        P: AsRef<Path> + ?Sized,
        Q: AsRef<Path> + ?Sized,
    {
        let ifile = File::open(from.as_ref())?;
        let ofile = File::create(to.as_ref())?;
        let mut reader = BufReader::new(ifile);
        let mut writer = BufWriter::new(ofile);
        Self::read_from_and_write_to_with(&mut reader, &mut writer, opts)
    }

    /// Writes all lines in 'as is' order does not checks.
    /// Use [`SrtSubtitles::standardize`] to ensures correct.
    ///
    /// Writes:
    /// - A [`SrtLineType::Blank`] as `\n`
    /// - A [`SrtLineType::Number`] as `{num}\n`
    /// - A [`SrtLineType::TimeRange((start, end))`] as `{start} --> {end}\n`
    /// - A [`SrtLineType::Text`] as `{text}\n`
    ///
    /// Ensures monotone increase index (number) of output block from 1.
    pub fn write_to_writer<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        static DEFAULT_OPTIONS: WriteOptions = WriteOptions::new();
        let mut num = 1usize;
        let mut len = 0usize;
        for block in self.blocks() {
            block.write_with(writer, &DEFAULT_OPTIONS, &mut num, len)?;
            len += block.len();
        }
        Ok(())
    }

    /// Same as [`SrtSubtitles::write_to_writer`] with additional [`WriteOptions`].
    pub fn write_to_writer_with<W>(&self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(BOM)?;
        }
        let mut num = 1usize;
        let mut len = 0usize;
        let mut blocks = self.blocks();
        if let Some(first) = blocks.next() {
            if !opts.skip_first_blank || !first.is_blank() {
                first.write_with(writer, opts, &mut num, len)?;
                len += first.len();
            }
        }
        for block in blocks {
            block.write_with(writer, opts, &mut num, len)?;
            len += block.len();
        }
        Ok(())
    }

    /// Writes all raw bytes to writer.
    ///
    /// # Errors
    /// Returns an I/O error.
    pub fn raw_write_to_writer<W>(&self, writer: &mut W) -> Result<()>
    where
        W: Write + ?Sized,
    {
        for line in &self.0 {
            writer.write(&line.raw)?;
        }
        Ok(())
    }

    /// Per block read from reader and write to writer with [`WriteOptions`].
    ///
    /// Adds missing end blank line to last block.
    pub fn read_from_and_write_to_with<R, W>(
        reader: &mut R,
        writer: &mut W,
        opts: &WriteOptions,
    ) -> Result<()>
    where
        R: BufRead + ?Sized,
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(BOM)?;
        }

        let mut lines: Vec<SrtLine> = Vec::new();
        let mut num = 1;
        let mut len = 0;

        new::read_block(reader, &mut lines)?;
        if let Some(line) = lines.get_mut(0) {
            line.correct_first_bomed();
        }
        let first = SrtBlock(&lines[..]);
        if !opts.skip_first_blank || !first.is_blank() {
            first.write_with(writer, opts, &mut num, len)?;
            len += first.len();
        }

        loop {
            lines.clear();
            new::read_block(reader, &mut lines)?;
            let block_len = lines.len();
            if block_len == 1 {
                break;
            }
            let block = SrtBlock(&lines[..]);
            block.write_with(writer, opts, &mut num, len)?;
            len += block_len;
        }

        Ok(())
    }
}

impl SrtBlock<'_> {
    fn write_with<W>(
        &self,
        writer: &mut W,
        opts: &WriteOptions,
        num: &mut usize,
        start_line: usize,
    ) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.error_on_non_standard {
            self.validate_standard_at(start_line)?;
        }
        if opts.skip_non_standards && !self.is_standard() {
            return Ok(());
        }
        let time_range = self.get_time_range();

        let need_write = match (opts.start_from, time_range) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(start), Some((st, _))) => *st >= start,
        };
        if !need_write {
            return Ok(());
        }

        let need_write = match (opts.end_on, time_range) {
            (None, _) => true,
            (Some(_), None) => false,
            (Some(end), Some((_, e))) => *e <= end,
        };
        if !need_write {
            return Ok(());
        }

        for line in self.0 {
            match line.ty {
                SrtLineType::Blank => (),
                SrtLineType::Number => {
                    let _ = writer.write(num.to_string().as_bytes())?;
                    *num += 1;
                }
                SrtLineType::TimeRange((mut st, mut end)) => {
                    if let Some(add) = opts.add_time {
                        st += add;
                        end += add;
                    }
                    if let Some(sub) = opts.sub_time {
                        st -= sub;
                        end -= sub;
                    }
                    let line = format!("{} --> {}", st.to_srt(), end.to_srt());
                    let _ = writer.write(line.as_bytes())?;
                }
                SrtLineType::Text => {
                    let _ = writer.write(crate::trim(&line.raw))?;
                }
            };
            let _ = writer.write(&[b'\n'])?;
        }
        Ok(())
    }
}
