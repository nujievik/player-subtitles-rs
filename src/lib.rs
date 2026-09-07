pub mod ass;
pub mod srt;
pub mod vtt;

mod byte_helpers;
mod byte_lines;
mod error;
mod options;
mod time;
mod traits;

pub use ass::{AssLine, AssLines};
pub use byte_lines::ByteLines;
pub use error::Error;
pub use options::WriteOptions;
pub use srt::{SrtLine, SrtLines};
pub use time::Time;
pub use traits::{NewLines, StreamingIterator, WriteLines};
pub use vtt::{VttLine, VttLines};

pub type Result<T> = std::result::Result<T, Error>;

use ass::RegularAssLines;
use srt::{RegularSrtLines, it::TransIterState};
use vtt::RegularVttLines;

const BOM: &[u8] = "\u{feff}".as_bytes();

enum SourceLines<'a, T: std::io::BufRead> {
    Ass(RegularAssLines<'a, T>),
    Srt(RegularSrtLines<'a, T>),
    Vtt(RegularVttLines<'a, T>),
}
