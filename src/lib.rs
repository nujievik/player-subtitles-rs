mod error;
mod options;
pub mod srt;
mod time;

pub use error::Error;
pub use options::WriteOptions;
pub use srt::{SrtBlock, SrtLine, SrtLineType, SrtSubtitles};
pub use time::Time;

pub type Result<T> = std::result::Result<T, Error>;

const BOM: &[u8] = "\u{feff}".as_bytes();

// Returns a byte slice with leading and trailing whitespace removed.
fn trim(data: &[u8]) -> &[u8] {
    let mut start = 0;
    let mut end = data.len();

    while start < end && data[start].is_ascii_whitespace() {
        start += 1;
    }

    while end > start && data[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    &data[start..end]
}
