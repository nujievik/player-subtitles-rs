use crate::byte_helpers;

#[derive(Debug, PartialEq)]
pub struct Comment<'a> {
    pub(crate) bytes: &'a [u8],
    pub(crate) text: &'a [u8],
}

impl<'a> Comment<'a> {
    pub fn as_bytes(&self) -> &[u8] {
        self.bytes
    }

    // requires non-empty line
    pub(crate) fn get_new(line: &'a [u8]) -> Option<Self> {
        let x = Self::new(line);
        if x.text.len() != x.bytes.len() {
            Some(x)
        } else {
            None
        }
    }

    // requires non-empty line
    pub(crate) fn new(line: &'a [u8]) -> Self {
        let text = if matches!(line[0], b';') {
            if line.len() > 1 { &line[1..] } else { b"" }
        } else {
            byte_helpers::trim_prefix(line, "!:")
        };
        Self::new_with(line, byte_helpers::trim_start(text))
    }

    fn new_with(bytes: &'a [u8], text: &'a [u8]) -> Self {
        Self { bytes, text }
    }
}
