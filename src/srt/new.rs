use super::{SrtBlock, SrtLine, SrtLineType, SrtSubtitles};
use crate::Result;
use std::{
    fs,
    io::{BufRead, BufReader},
    mem,
    path::Path,
};

impl<'a> SrtSubtitles<'a> {
    /// Infallible construct a borrowed subtitles from a bytes.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// Does not checks correct, use [`SrtSubtitles::try_from_bytes`] OR
    /// [`SrtSubtitles::standardize`] for ensures correct.
    pub fn from_bytes<B>(data: &'a B) -> SrtSubtitles<'a>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let mut lines: Vec<SrtLine> = Vec::new();
        let mut pos = 0;
        let data = data.as_ref();
        let len = data.len();

        while pos < len {
            let i_start = pos;
            super::set_newline_position(&mut pos, data, len);
            if pos > len {
                break;
            }
            lines.push(SrtLine::new(&data[i_start..pos]));
        }

        if let Some(line) = lines.get_mut(0) {
            line.correct_first_bomed();
        }
        if !lines.last_mut().is_some_and(|l| l.ty.is_blank()) {
            lines.push(SrtLine::new_with_ty(&[], SrtLineType::Blank));
        }

        SrtSubtitles(lines)
    }

    /// Infallible construct a borrowed subtitles from a string.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// Does not checks correct, use [`SrtSubtitles::try_from_str`] OR
    /// [`SrtSubtitles::standardize`] for ensures correct.
    pub fn from_str<S>(data: &'a S) -> SrtSubtitles<'a>
    where
        S: AsRef<str> + ?Sized,
    {
        Self::from_bytes(data.as_ref().as_bytes())
    }

    /// Tries construct a borrowed standard subtitles from a bytes.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// # Errors
    ///
    /// Returns an error **only if** a subtitle block is non-standard.
    pub fn try_from_bytes<B>(data: &'a B) -> Result<SrtSubtitles<'a>>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let mut lines: Vec<SrtLine> = Vec::new();
        let data = data.as_ref();
        let len = data.len();
        let mut pos = 0;
        let mut block_start = 0;
        let mut was_blank = false;

        while pos < len {
            let i_start = pos;
            super::set_newline_position(&mut pos, data, len);
            if pos > len {
                break;
            }

            let mut line = SrtLine::new(&data[i_start..pos]);
            if i_start == 0 {
                line.correct_first_bomed();
            }
            let is_blank = line.ty.is_blank();

            if !is_blank && was_blank {
                let block = SrtBlock(&lines[block_start..]);
                block.validate_standard_at(block_start)?;
                block_start = lines.len();
            }
            was_blank = is_blank;
            lines.push(line);
        }

        if !lines.last_mut().is_some_and(|l| l.ty.is_blank()) {
            lines.push(SrtLine::new_with_ty(&[], SrtLineType::Blank));
        }
        let block = SrtBlock(&lines[block_start..]);
        block.validate_standard_at(block_start)?;

        Ok(SrtSubtitles(lines))
    }

    /// Tries construct a borrowed standard subtitles from a string.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// # Errors
    ///
    /// Returns an error **only if** a subtitle block is non-standard.
    pub fn try_from_str<S>(data: &'a S) -> Result<SrtSubtitles<'a>>
    where
        S: AsRef<str> + ?Sized,
    {
        Self::try_from_bytes(data.as_ref().as_bytes())
    }
}

impl SrtSubtitles<'static> {
    /// Tries read the entire contents of a file into an owned subtitles.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// Does not checks correct, use [`SrtSubtitles::try_read`] OR [`SrtSubtitles::standardize`]
    /// for ensures correct.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// - if `path` does not already exist.
    /// - on other I/O errors.
    pub fn read<P>(path: &P) -> Result<SrtSubtitles<'static>>
    where
        P: AsRef<Path> + ?Sized,
    {
        let f = fs::File::open(path.as_ref())?;
        let mut reader = BufReader::new(f);
        Self::from_reader(&mut reader)
    }

    /// Tries read the entire contents of a file into an owned subtitles.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// # Errors
    /// Returns an error in the following cases:
    /// - if `path` does not already exist.
    /// - on other I/O errors.
    /// - a subtitle block is non-standard.
    pub fn try_read<P>(path: &P) -> Result<SrtSubtitles<'static>>
    where
        P: AsRef<Path> + ?Sized,
    {
        let f = fs::File::open(path.as_ref())?;
        let mut reader = BufReader::new(f);
        Self::try_from_reader(&mut reader)
    }

    /// Tries read an owned subtitles from a reader.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// Does not checks correct, use [`SrtSubtitles::try_from_reader`] OR
    /// [`SrtSubtitles::standardize`] for ensures correct.
    ///
    /// # Errors
    ///
    /// Returns an error **only if** reader returns an [`std::io::Error`].
    pub fn from_reader<R>(reader: &mut R) -> Result<SrtSubtitles<'static>>
    where
        R: BufRead + ?Sized,
    {
        let mut lines: Vec<SrtLine> = Vec::new();
        loop {
            let len = lines.len();
            read_block(reader, &mut lines)?;
            if len == lines.len() {
                break;
            }
        }
        if let Some(line) = lines.get_mut(0) {
            line.correct_first_bomed();
        }
        Ok(SrtSubtitles(lines))
    }

    /// Tries read an owned standard subtitles from a reader.
    ///
    /// Adds missing end blank line with empty raw.
    ///
    /// # Errors
    ///
    /// Returns an error in the following cases:
    /// - reader returns an [`std::io::Error`].
    /// - a subtitle block is non-standard.
    pub fn try_from_reader<R>(reader: &mut R) -> Result<SrtSubtitles<'static>>
    where
        R: BufRead + ?Sized,
    {
        let mut lines: Vec<SrtLine> = Vec::new();
        loop {
            let len = lines.len();
            read_block(reader, &mut lines)?;
            if len == lines.len() {
                break;
            }
            if len == 0 {
                lines[0].correct_first_bomed();
            }

            let block = SrtBlock(&lines[len..]);
            block.validate_standard_at(len)?;
        }

        Ok(SrtSubtitles(lines))
    }
}

pub(crate) fn read_block<R>(reader: &mut R, lines: &mut Vec<SrtLine>) -> Result<()>
where
    R: BufRead + ?Sized,
{
    let mut raw_line: Vec<u8> = Vec::new();
    let mut was_blank = false;

    loop {
        let mut pos = 0usize;
        let data = reader.fill_buf()?;
        let len = data.len();
        if len == 0 {
            break;
        }
        super::set_newline_position(&mut pos, data, len);

        if pos == len && !matches!(data.last(), Some(b'\n') | Some(b'\r')) {
            raw_line.extend(&data[..pos]);
            reader.consume(pos);
            continue;
        }

        let raw = if raw_line.is_empty() {
            Vec::<u8>::from(&data[..pos])
        } else {
            raw_line.extend(&data[..pos]);
            mem::take(&mut raw_line)
        };
        let line = SrtLine::new_owned(raw);
        reader.consume(pos);

        let is_blank = line.ty.is_blank();
        if !is_blank && was_blank {
            return Ok(());
        }
        was_blank = is_blank;
        lines.push(line);
    }

    if !lines.last().is_some_and(|l| l.ty.is_blank()) {
        lines.push(SrtLine::new_with_ty(&[], SrtLineType::Blank));
    }

    Ok(())
}
