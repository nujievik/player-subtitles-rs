use crate::Time;

/// A subtitles write options.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct WriteOptions {
    /// Write the UTF-8 byte order mark (BOM).
    pub bom: bool,
    /// Start write from a timestamp block.
    pub start_from: Option<Time>,
    /// End write on a timestamp block.
    pub end_on: Option<Time>,
    /// Add a time to timestamps.
    pub add_time: Option<Time>,
    /// Sub a time from timestamps.
    pub sub_time: Option<Time>,
}

impl WriteOptions {
    pub const fn new() -> Self {
        Self {
            bom: false,
            start_from: None,
            end_on: None,
            add_time: None,
            sub_time: None,
        }
    }
}
