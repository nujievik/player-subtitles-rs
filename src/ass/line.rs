mod comment;
mod event;
mod info;
mod mark;

pub use comment::Comment;
pub use event::{Event, EventFormat};
pub use info::{ScriptInfo, ScriptType, Title, WrapStyle};
pub use mark::SectionMark;

#[derive(Debug, PartialEq)]
pub enum AssLine<'a> {
    Blank,
    SectionMark(SectionMark),
    Comment(Comment<'a>),
    ScriptInfo(ScriptInfo<'a>),
    EventFormat(EventFormat),
    Event(Event<'a>),
    Unrecognized(&'a [u8]),
}
