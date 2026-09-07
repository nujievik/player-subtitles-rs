#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SectionMark {
    ScriptInfo,
    V4Styles,
    V4StylesPlus,
    Events,
    Fonts,
    Graphics,
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

    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::ScriptInfo => b"[Script Info]",
            Self::V4Styles => b"[v4 Styles]",
            Self::V4StylesPlus => b"[v4 Styles+]",
            Self::Events => b"[Events]",
            Self::Fonts => b"[Fonts]",
            Self::Graphics => b"[Graphics]",
        }
    }
}
