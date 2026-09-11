use crate::{Time, byte_helpers};

#[derive(Debug, PartialEq)]
pub enum VttLine<'a> {
    VttFileMark(VttFileMark<'a>),
    Blank,
    RegionMark,
    StyleMark,
    Region(Region<'a>),
    Style(Style<'a>),
    Comment(Comment<'a>),
    CueId(CueId<'a>),
    TimeRangeAndStyle(TimeRangeAndStyle<'a>),
    Metadata(Metadata<'a>),
    Text(Text<'a>),
    Unrecognized(&'a [u8]),
}

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    pub(crate) bytes: &'a [u8],
    text: &'a [u8],
}

#[derive(Debug, PartialEq)]
pub struct TimeRangeAndStyle<'a> {
    pub(crate) bytes: &'a [u8],
    pub start: Time,
    pub end: Time,
}

bytes_field_struct!(VttFileMark);
bytes_field_struct!(Region);
bytes_field_struct!(Style);
bytes_field_struct!(CueId);
bytes_field_struct!(Metadata);
bytes_field_struct!(Text);

impl<'a> VttLine<'a> {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::VttFileMark(VttFileMark { bytes }) => bytes,
            Self::Blank => b"",
            Self::RegionMark => b"REGION",
            Self::StyleMark => b"STYLE",
            Self::Region(Region { bytes }) => bytes,
            Self::Style(Style { bytes }) => bytes,
            Self::Comment(Comment { bytes, .. }) => bytes,
            Self::CueId(CueId { bytes }) => bytes,
            Self::TimeRangeAndStyle(TimeRangeAndStyle { bytes, .. }) => bytes,
            Self::Metadata(Metadata { bytes, .. }) => bytes,
            Self::Text(Text { bytes }) => bytes,
            Self::Unrecognized(bytes) => bytes,
        }
    }
}

impl<'a> Comment<'a> {
    pub(crate) fn new(bytes: &'a [u8], text: &'a [u8]) -> Self {
        Comment { bytes, text }
    }

    pub fn text(&self) -> &[u8] {
        self.text
    }
}

impl<'a> TimeRangeAndStyle<'a> {
    pub(crate) fn get_new(bytes: &'a [u8]) -> Option<Self> {
        let mut words = byte_helpers::words(bytes);
        if let (Some(start), Some(b"-->"), Some(end)) = (words.next(), words.next(), words.next()) {
            let start = get_time(start)?;
            let end = get_time(end)?;
            Some(Self { bytes, start, end })
        } else {
            None
        }
    }
}

impl<'a> Text<'a> {
    pub fn text(&self) -> impl Iterator<Item = &[u8]> + use<'_> {
        let mut pos = 0usize;
        let mut tag_depth = 0usize;
        std::iter::from_fn(move || {
            let len = self.bytes.len();
            while pos < len {
                if tag_depth > 0 {
                    while pos < len {
                        let is_close = self.bytes[pos] == b'>';
                        pos += 1;
                        if is_close {
                            tag_depth -= 1;
                            break;
                        }
                    }
                    continue;
                }

                let start = pos;
                while pos < len && self.bytes[pos] != b'<' {
                    pos += 1;
                }

                if pos < len {
                    tag_depth += 1;
                }

                return Some(&self.bytes[start..pos]);
            }
            None
        })
    }
}

fn get_time(data: &[u8]) -> Option<Time> {
    let mut it = data.split(|b| matches!(b, b'.'));
    let remainder = it.next()?;
    let millis = it.next()?;

    let mut it = remainder.split(|b| matches!(b, b':')).rev();
    let secs = it.next()?;

    let millis = byte_helpers::get_u16(millis)?;
    let secs = byte_helpers::get_u8(secs)?;
    let mins = match it.next() {
        Some(mins) => byte_helpers::get_u8(mins)?,
        None => 0,
    };
    let hours = match it.next() {
        Some(hours) => byte_helpers::get_u16(hours)?,
        None => 0,
    };

    Time::new(hours, mins, secs, millis).ok()
}
