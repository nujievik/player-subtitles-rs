#[derive(Debug, PartialEq)]
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

impl StyleLine {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        todo!()
    }
}
