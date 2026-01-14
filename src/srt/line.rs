use crate::{Error, Result, Time};
use std::borrow::Cow;

/// A SubRip subtitles line.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SrtLine<'a> {
    /// A source line byte-to-byte.
    pub raw: Cow<'a, [u8]>,
    /// A type of line.
    pub ty: SrtLineType,
}

/// A type of SubRip subtitles line.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SrtLineType {
    Blank,
    Number,
    /// (start, end) subtitle range.
    TimeRange((Time, Time)),
    Text,
}

impl<'a> SrtLine<'a> {
    /// Constructs a borrowed line with byte-to-byte raw bytes and ty from [`SrtLineType::new`].
    ///
    /// Does not checks raw and time range correct, use [`SrtLine::try_new`] for ensures correct
    /// its.
    pub fn new<B>(raw: &'a B) -> SrtLine<'a>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let raw = Cow::Borrowed(raw.as_ref());
        let ty = SrtLineType::new(&*raw);
        Self { raw, ty }
    }

    /// Tries construct a borrowed line with byte-to-byte raw bytes and ty from
    /// [`SrtLineType::new`].
    ///
    /// # Errors
    /// Returns an error if raw bytes:
    /// - not ends with newline byte.
    /// - has multiple newline byte.
    ///
    /// Also returns an error if is time range line and:
    /// - end hours >= 100.
    /// - start time > end time.
    pub fn try_new<B>(raw: &'a B) -> Result<SrtLine<'a>>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let raw = raw.as_ref();
        Self::new_line_raw_validate(raw)?;
        let new = Self::new(raw);
        new.time_range_validate()?;
        Ok(new)
    }

    pub(crate) fn new_with_ty<B>(raw: &'a B, ty: SrtLineType) -> SrtLine<'a>
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let raw = Cow::Borrowed(raw.as_ref());
        Self { raw, ty }
    }

    pub(crate) fn new_line_raw_validate(raw: &[u8]) -> Result<()> {
        if !raw.last().is_some_and(|b| matches!(b, b'\r' | b'\n')) {
            return Err(Error::ValueValidation("last byte is not newline"));
        }
        let mut pos = 0;
        let len = raw.len();
        super::set_newline_position(&mut pos, raw, len);
        if pos == len {
            Ok(())
        } else {
            Err(Error::ValueValidation("multiple newline bytes"))
        }
    }

    pub(crate) fn time_range_validate(&self) -> Result<()> {
        let err = match self.ty {
            SrtLineType::TimeRange((_, end)) if end.hours >= 100 => "end time hours >= 100",
            SrtLineType::TimeRange((st, end)) if st > end => "start time > end time",
            _ => return Ok(()),
        };
        Err(Error::ValueValidation(err))
    }

    pub(crate) fn correct_first_bomed(&mut self) {
        if !self.ty.is_text() {
            return;
        }
        if let Some(bytes) = self.raw.strip_prefix(crate::BOM) {
            self.ty = SrtLine::new(bytes).ty;
        }
    }
}

impl SrtLine<'static> {
    /// Constructs an owned line with byte-to-byte raw bytes and ty from [`SrtLineType::new`].
    ///
    /// Does not checks raw correct, use [`SrtLine::try_new_owned`] for ensures correct its.
    pub fn new_owned<B>(raw: B) -> SrtLine<'static>
    where
        B: Into<Vec<u8>>,
    {
        let raw = Cow::Owned(raw.into());
        let ty = SrtLineType::new(&*raw);
        SrtLine { raw, ty }
    }

    /// Same as [`SrtLine::try_new`] but constructs owned line.
    pub fn try_new_owned<B>(raw: B) -> Result<SrtLine<'static>>
    where
        B: AsRef<[u8]> + Into<Vec<u8>>,
    {
        Self::new_line_raw_validate(raw.as_ref())?;
        let new = Self::new_owned(raw.into());
        new.time_range_validate()?;
        Ok(new)
    }
}

impl SrtLineType {
    /// Constructs a line from a bytes by the following rules:
    /// 1. [`SrtLineType::Blank`] if all bytes is whitespace.
    /// 2. [`SrtLineType::Number`] if all bytes of trimmed is ascii digit.
    /// 3. [`SrtLineType::TimeRange`] if all following predicates `true`:
    ///     - bytes contains a formatted time range `hh:mm:ss,mil --> hh:mm:ss,mil`. Additional
    /// recognizes dot instead comma, less or more a part of time bytes.
    ///     - minutes and seconds < 60.
    /// 4. Otherwise, [`SrtLineType::Text`].
    pub fn new<B>(bytes: &B) -> SrtLineType
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let trimmed = crate::trim(bytes.as_ref());
        if trimmed.is_empty() {
            SrtLineType::Blank
        } else if trimmed.iter().all(|b| b.is_ascii_digit()) {
            SrtLineType::Number
        } else if let Some(ts) = get_time_range(trimmed) {
            SrtLineType::TimeRange(ts)
        } else {
            SrtLineType::Text
        }
    }

    /// Returns `true` if self is [`SrtLineType::Blank`].
    #[inline]
    pub const fn is_blank(&self) -> bool {
        matches!(self, SrtLineType::Blank)
    }

    /// Returns `true` if self is [`SrtLineType::Number`].
    #[inline]
    pub const fn is_number(&self) -> bool {
        matches!(self, SrtLineType::Number)
    }

    /// Returns `true` if self is [`SrtLineType::TimeRange`].
    #[inline]
    pub const fn is_time_range(&self) -> bool {
        matches!(self, SrtLineType::TimeRange(_))
    }

    /// Returns `true` if self is [`SrtLineType::Text`].
    #[inline]
    pub const fn is_text(&self) -> bool {
        matches!(self, SrtLineType::Text)
    }

    pub(crate) fn is_standard_time_range(&self) -> bool {
        matches!(self, SrtLineType::TimeRange((st, end)) if st <= end && end.hours < 100)
    }

    pub(crate) fn get_time_range(&self) -> Option<&(Time, Time)> {
        match self {
            Self::TimeRange(ts) => Some(ts),
            _ => None,
        }
    }

    pub(crate) fn unwrap_time_range(&self) -> &(Time, Time) {
        self.get_time_range().expect("is not a time range")
    }
}

fn get_time_range(data: &[u8]) -> Option<(Time, Time)> {
    let pos = data.windows(3).position(|w| w == b"-->")?;
    let (left, right) = data.split_at(pos);
    let right = &right[3..];

    let start = get_time(left)?;
    let end = get_time(right)?;
    Some((start, end))
}

fn get_time(data: &[u8]) -> Option<Time> {
    let trimmed = crate::trim(data);
    let mut it = trimmed.split(|&b| b == b':');

    let hours = get_u16(it.next()?)?;
    let mins = get_u8(it.next()?)?;

    let sec_ms = it.next()?;
    let separator = sec_ms.iter().position(|&b| matches!(b, b',' | b'.'))?;
    let (secs_b, millis_b) = sec_ms.split_at(separator);

    let secs = get_u8(secs_b)?;
    let millis = get_u16(&millis_b[1..millis_b.len().min(4)])?;

    Time::new(hours, mins, secs, millis).ok()
}

macro_rules! get_an_u_number {
    ($fn:ident, $u:ident, $buf_u:ident) => {
        fn $fn(data: &[u8]) -> Option<$u> {
            if data.is_empty() {
                return None;
            }
            let mut v: $buf_u = 0;
            for &b in data {
                if !b.is_ascii_digit() {
                    return None;
                }
                v = v * 10 + (b - b'0') as $buf_u;
                if v > $u::MAX as $buf_u {
                    return None;
                }
            }
            Some(v as $u)
        }
    };
}

get_an_u_number!(get_u8, u8, u16);
get_an_u_number!(get_u16, u16, u32);
