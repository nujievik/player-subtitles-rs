macro_rules! bytes_field_struct {
    ($struct:ident) => {
        #[derive(Debug, PartialEq)]
        pub struct $struct<'a> {
            pub(crate) bytes: &'a [u8],
        }

        impl<'a> $struct<'a> {
            pub fn as_bytes(&self) -> &[u8] {
                &self.bytes
            }
        }
    };
}

pub mod ass;
pub mod srt;
pub mod vtt;

mod byte_helpers;
mod byte_lines;
mod error;
mod options;
mod time;
mod traits;

pub use ass::AssLines;
pub use byte_lines::ByteLines;
pub use error::Error;
pub use options::WriteOptions;
pub use srt::SrtLines;
pub use time::Time;
pub use traits::{NewLines, StreamingIterator, WriteLines};
pub use vtt::VttLines;

pub type Result<T> = std::result::Result<T, Error>;

use ass::{RegularAssLines, line::AssLine};
use srt::{RegularSrtLines, line::SrtLine};
use vtt::{RegularVttLines, line::VttLine};

const BOM: &[u8] = "\u{feff}".as_bytes();

#[derive(Debug)]
enum SourceLines<'a, T: std::io::BufRead> {
    Ass(RegularAssLines<'a, T>),
    Srt(RegularSrtLines<'a, T>),
    Vtt(RegularVttLines<'a, T>),
}
