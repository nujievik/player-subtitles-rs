use crate::Time;

/// A subtitles write options.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct WriteOptions {
    /// Write the UTF-8 byte order mark (BOM).
    pub bom: bool,
    /// Skip first blank block.
    pub skip_first_blank: bool,
    /// Skip non-standard blocks.
    pub skip_non_standards: bool,
    /// Return an error on non-standard block.
    pub error_on_non_standard: bool,
    /// Start write from a timestamp block, skipping a non-timestamp blocks.
    pub start_from: Option<Time>,
    /// End write on a timestamp block, skipping a non-timestamp blocks.
    pub end_on: Option<Time>,
    /// Add a time to timestamps.
    pub add_time: Option<Time>,
    /// Sub a time from timestamps.
    pub sub_time: Option<Time>,
}

impl WriteOptions {
    pub(crate) const fn new() -> Self {
        Self {
            bom: false,
            skip_first_blank: false,
            skip_non_standards: false,
            error_on_non_standard: false,
            start_from: None,
            end_on: None,
            add_time: None,
            sub_time: None,
        }
    }
}
