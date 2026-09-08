use crate::Time;

/// A subtitles write options.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct WriteOptions {
    /// Write the UTF-8 byte order mark (BOM).
    pub bom: bool,
    /// Write events that a timestamp > `start`. Write all if `start` is None.
    pub start: Option<Time>,
    /// Write events that a timestamp < `end`. Write all if `end` is None.
    pub end: Option<Time>,
    /// Add a time to timestamps.
    pub add_time: Option<Time>,
    /// Sub a time from timestamps.
    pub sub_time: Option<Time>,
}

impl WriteOptions {
    pub const fn new() -> Self {
        Self {
            bom: false,
            start: None,
            end: None,
            add_time: None,
            sub_time: None,
        }
    }
}
