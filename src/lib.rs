pub mod srt;

mod byte_helpers;
mod byte_lines;
mod error;
mod options;
mod time;

pub use byte_lines::ByteLines;
pub use error::Error;
pub use options::WriteOptions;
pub use srt::{SrtLine, SrtLines};
pub use time::Time;

pub type Result<T> = std::result::Result<T, Error>;

pub trait StreamingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

const BOM: &[u8] = "\u{feff}".as_bytes();
