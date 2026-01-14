use super::{SrtLine, SrtLineType};
use crate::{Error, Result, Time};

/// A convenient SubRip subtitles organization. Is minimal playable part.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SrtBlock<'a>(pub &'a [SrtLine<'a>]);
deref_singleton_lifetime_struct!(SrtBlock<'a>, &'a [SrtLine<'a>]);

impl SrtBlock<'_> {
    /// Returns `true` if block is non-empty and all block lines is blank.
    pub fn is_blank(&self) -> bool {
        !self.0.is_empty() && self.0.iter().all(|l| l.ty.is_blank())
    }

    /// Same as [`SrtBlock::validate_standard`] but returns bool.
    pub fn is_standard(&self) -> bool {
        self.validate_standard().is_ok()
    }

    /// Validates a block standard.
    ///
    /// Does not checks raws.
    ///
    /// # Errors
    /// Returns an error if any following conditions is not true:
    /// 1. First block line is "number".
    /// 2. Second block line is "time tange" AND start time <= end time AND hours part < 100.
    /// 3. Third block line is "text".
    /// 4. Last block line is "blank"".
    /// 5. All other block lines before a "blank" line is "text".
    /// 6. All other block lines is "blank".
    pub fn validate_standard(&self) -> Result<()> {
        if self.len() < 4 {
            return Err(Error::ValueValidation("length < 4"));
        }
        if !self[0].ty.is_number() {
            return Err(Error::ValueValidation("first line is not number"));
        }

        match self[1].ty {
            SrtLineType::TimeRange(_) => self[1].time_range_validate()?,
            _ => return Err(Error::ValueValidation("second line is not time range")),
        };

        if !self[2].ty.is_text() {
            return Err(Error::ValueValidation("third line is not text"));
        }
        if !self.last().unwrap().ty.is_blank() {
            return Err(Error::ValueValidation("last line is not blank"));
        }

        let mut pos = 3;
        loop {
            match self[pos].ty {
                SrtLineType::Blank => break,
                SrtLineType::Text => pos += 1,
                _ => return Err(Error::ValueValidation("not text or blank line after text")),
            }
        }
        for l in &self[pos..] {
            if !l.ty.is_blank() {
                return Err(Error::ValueValidation("not blank line after blank"));
            }
        }

        Ok(())
    }

    pub(crate) fn validate_standard_at(&self, start_line: usize) -> Result<()> {
        match self.validate_standard() {
            Ok(()) => Ok(()),
            Err(Error::ValueValidation(s)) => Err(Error::NonStandardBlock((start_line, s))),
            Err(e) => Err(e),
        }
    }

    pub(crate) fn get_time_range(&self) -> Option<&(Time, Time)> {
        self.iter().find_map(|l| l.ty.get_time_range())
    }
}
