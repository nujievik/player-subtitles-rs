mod comment;
mod event;
mod info;
mod mark;
mod style;

pub use comment::Comment;
pub use event::{Event, EventFormat};
pub use info::{ScriptInfo, ScriptType, Title, WrapStyle};
pub use mark::SectionMark;
pub use style::StyleLine;

pub enum AssLine<'a> {
    Blank,
    SectionMark(SectionMark),
    Comment(Comment<'a>),
    ScriptInfo(ScriptInfo<'a>),
    Style(StyleLine),
    EventFormat(EventFormat),
    Event(Event<'a>),
    Unrecognized(&'a [u8]),
}
