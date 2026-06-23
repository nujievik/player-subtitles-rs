mod event;

pub use event::{Event, EventFormat};

pub enum AssLine<'a> {
    Blank,
    SectionMark(SectionMark),
    Comment(Comment<'a>),
    ScriptInfo(ScriptInfo),
    Style(StyleLine),
    EventFormat(EventFormat),
    Event(Event<'a>),
    Unrecognized(&'a [u8]),
}

#[derive(Copy, Clone)]
pub enum SectionMark {
    ScriptInfo,
    V4Styles,
    V4StylesPlus,
    Events,
    Fonts,
    Graphics,
}

pub enum ScriptInfo {
    Title,
    OriginalScript,
    OriginalTranslation,
    OriginalEditing,
    OriginalTiming,
    SynchPoint,
    ScriptUpdatedBy,
    UpdateDetails,
    ScriptType,
    Collisions,
    PlayResY,
    PlayResX,
    PlayDepth,
    Timer,
    WrapStyle,
}

pub enum StyleLine {
    Name,
    FontName,
    FontSize,
    PrimaryColour,
    SecondaryColour,
    OutlineColor,
    BackColour,
    Bold,
    Italic,
    Underline,
    Strikeout,
    ScaleX,
    ScaleY,
    Spacing,
    Angle,
    BorderStyle,
    Outline,
    Shadow,
    Alignment,
    AlignmentAfterNumpad,
    MarginL,
    MarginR,
    MarginV,
    AlphaLevel,
    Encoding,
}

pub struct Comment<'a> {
    bytes: &'a [u8],
}

impl SectionMark {
    pub(crate) fn get_from_bytes(bs: &[u8]) -> Option<Self> {
        let mark = match bs {
            b"v4 Styles" => SectionMark::V4Styles,
            b"v4 Styles+" | b"V4+ Styles" => SectionMark::V4StylesPlus,
            b"Events" => SectionMark::Events,
            b"Fonts" => SectionMark::Fonts,
            b"Graphics" => SectionMark::Graphics,
            b"Script Info" => SectionMark::ScriptInfo,
            _ => return None,
        };
        Some(mark)
    }
}
