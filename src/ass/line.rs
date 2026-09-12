mod comment;
mod event;
mod info;
mod mark;

pub use comment::Comment;
pub use event::{
    Event,
    format::{EventFormat, EventFormatPositions},
};
pub use info::{ScriptInfo, ScriptType, Title, WrapStyle};
pub use mark::SectionMark;

#[derive(Debug, PartialEq)]
pub enum AssLine<'a> {
    Blank,
    SectionMark(SectionMark),
    Comment(Comment<'a>),
    ScriptInfo(ScriptInfo<'a>),
    EventFormat(EventFormat<'a>),
    Event(Event<'a>),
    Unrecognized(&'a [u8]),
}

impl<'a> AssLine<'a> {
    /// Returns the source bytes `as is`, correctness is not guaranteed.
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Blank => &[],
            Self::SectionMark(x) => x.as_bytes(),
            Self::Comment(x) => x.as_bytes(),
            Self::ScriptInfo(x) => x.as_bytes(),
            Self::EventFormat(x) => x.as_bytes(),
            Self::Event(x) => x.bytes,
            Self::Unrecognized(bytes) => bytes,
        }
    }
}
