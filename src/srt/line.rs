use crate::{Time, byte_helpers};

#[derive(Debug, PartialEq)]
pub enum SrtLine<'a> {
    Blank,
    Number(Number<'a>),
    TimeRange(TimeRange<'a>),
    Text(Text<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Number<'a> {
    pub(crate) bytes: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct TimeRange<'a> {
    pub(crate) bytes: &'a [u8],
    pub start: Time,
    pub end: Time,
}

#[derive(Debug, PartialEq)]
pub struct Text<'a> {
    pub(crate) bytes: &'a [u8],
}

impl<'a> SrtLine<'a> {
    pub fn new<B>(bytes: &'a B) -> Self
    where
        B: AsRef<[u8]> + ?Sized,
    {
        let bytes = byte_helpers::trim(bytes.as_ref());
        if bytes.is_empty() {
            SrtLine::Blank
        } else if bytes.iter().all(|b| b.is_ascii_digit()) {
            SrtLine::Number(Number { bytes })
        } else if let Some((start, end)) = get_time_range(bytes) {
            SrtLine::TimeRange(TimeRange { bytes, start, end })
        } else {
            SrtLine::Text(Text { bytes })
        }
    }

    /// Returns bytes of source line.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Blank => &[],
            Self::Number(bs) => bs.as_bytes(),
            Self::TimeRange(bs) => bs.as_bytes(),
            Self::Text(bs) => bs.as_bytes(),
        }
    }

    /// Gets subtitle start time if line is [`SrtLine::TimeRange`].
    #[inline]
    pub fn get_start(&self) -> Option<Time> {
        match self {
            Self::TimeRange(bs) => Some(bs.start),
            _ => None,
        }
    }

    /// Gets subtitle end time if line is [`SrtLine::TimeRange`].
    #[inline]
    pub fn get_end(&self) -> Option<Time> {
        match self {
            Self::TimeRange(bs) => Some(bs.end),
            _ => None,
        }
    }
}

macro_rules! impl_deref {
    ($t:ident) => {
        impl<'a> std::ops::Deref for $t<'a> {
            type Target = [u8];

            fn deref(&self) -> &[u8] {
                self.as_bytes()
            }
        }
    };
}
impl_deref!(SrtLine);

macro_rules! impl_as_ref_bytes {
    ($t:ident) => {
        impl<'a> AsRef<[u8]> for $t<'a> {
            #[inline]
            fn as_ref(&self) -> &[u8] {
                self.as_bytes()
            }
        }
    };
}
impl_as_ref_bytes!(SrtLine);

macro_rules! impls_as_bytes {
    ($t:ident) => {
        impl<'a> $t<'a> {
            /// Returns bytes of source line.
            #[inline]
            pub fn as_bytes(&self) -> &[u8] {
                self.bytes.as_ref()
            }
        }

        impl_as_ref_bytes!($t);
        impl_deref!($t);
    };
}
impls_as_bytes!(Number);
impls_as_bytes!(TimeRange);
impls_as_bytes!(Text);

impl<'a> TimeRange<'a> {
    /// Returns subtitle start time.
    #[inline]
    pub fn start(&self) -> Time {
        self.start
    }

    /// Returns subtitle end time.
    #[inline]
    pub fn end(&self) -> Time {
        self.end
    }
}

fn get_time_range(data: &[u8]) -> Option<(Time, Time)> {
    let pos = data.windows(3).position(|w| w == b"-->")?;

    let (mut left, mut right) = data.split_at(pos);
    if right.len() < 8 {
        return None;
    } else {
        left = byte_helpers::trim_end(left);
        right = byte_helpers::trim_start(&right[3..]);
    }

    Some((get_time(left)?, get_time(right)?))
}

fn get_time(data: &[u8]) -> Option<Time> {
    let mut it = data.split(|&b| b == b':');

    let hours = byte_helpers::get_u16(it.next()?)?;
    let mins = byte_helpers::get_u8(it.next()?)?;

    let sec_ms = it.next()?;
    let separator = sec_ms.iter().position(|&b| matches!(b, b',' | b'.'))?;
    let (secs_b, millis_b) = sec_ms.split_at(separator);

    let secs = byte_helpers::get_u8(secs_b)?;
    let millis = byte_helpers::get_u16(&millis_b[1..millis_b.len().min(4)])?;

    Time::new(hours, mins, secs, millis).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_blank() {
        ["", " ", "\n", "\r\n", "\r", "\t"].iter().for_each(|s| {
            assert_eq!(SrtLine::Blank, SrtLine::new(s.as_bytes()));
        })
    }

    #[test]
    fn new_number() {
        ["0", "4", " 0 ", "4\n", "\t4\n"].iter().for_each(|s| {
            let bytes = byte_helpers::trim(s.as_bytes());
            let exp = SrtLine::Number(Number { bytes });
            assert_eq!(exp, SrtLine::new(s.as_bytes()));
        })
    }

    #[test]
    fn new_time_range() {
        [
            (
                "00:00:00,000 --> 00:00:05,000",
                Time::new_unchecked(0, 0, 0, 0),
                Time::new_unchecked(0, 0, 5, 0),
            ),
            (
                "12:45:55,657 --> 87:55:44,321",
                Time::new_unchecked(12, 45, 55, 657),
                Time::new_unchecked(87, 55, 44, 321),
            ),
        ]
        .into_iter()
        .for_each(|(s, start, end)| {
            let bytes = byte_helpers::trim(s.as_bytes());
            let exp = SrtLine::TimeRange(TimeRange { bytes, start, end });
            assert_eq!(exp, SrtLine::new(s.as_bytes()));
        })
    }

    #[test]
    fn new_text() {
        ["x", "abc", " qwerty ", "text\n", "\tdef\n"]
            .iter()
            .for_each(|s| {
                let bytes = byte_helpers::trim(s.as_bytes());
                let exp = SrtLine::Text(Text { bytes });
                assert_eq!(exp, SrtLine::new(s.as_bytes()));
            })
    }
}
