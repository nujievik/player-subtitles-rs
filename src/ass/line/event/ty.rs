#[derive(Debug, PartialEq)]
pub enum EventType<'a> {
    Dialogue,
    Comment,
    Picture,
    Sound,
    Movie,
    Command,
    Unrecognized(&'a [u8]),
}

impl<'a> EventType<'a> {
    pub(crate) fn as_bytes(&self) -> &[u8] {
        match self {
            Self::Dialogue => b"Dialogue",
            Self::Comment => b"Comment",
            Self::Picture => b"Picture",
            Self::Sound => b"Sound",
            Self::Movie => b"Movie",
            Self::Command => b"Command",
            Self::Unrecognized(bs) => bs,
        }
    }
}
