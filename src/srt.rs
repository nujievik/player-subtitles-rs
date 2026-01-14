//! A SubRip subtitles module.

macro_rules! deref_singleton_lifetime_struct {
    ($wrapper:ty, $inner:ty) => {
        impl<'a> std::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<'a> std::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

mod block;
mod line;
mod new;
mod standardize;
mod write;

pub use block::SrtBlock;
pub use line::{SrtLine, SrtLineType};

use std::{iter, ops::Range};

/// A SubRip subtitles in line-by-line representation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SrtSubtitles<'a>(pub Vec<SrtLine<'a>>);
deref_singleton_lifetime_struct!(SrtSubtitles<'a>, Vec<SrtLine<'a>>);

impl SrtSubtitles<'_> {
    /// Returns `true` if self is non-empty and all its blocks is standard.
    pub fn is_standard(&self) -> bool {
        !self.is_empty() && self.blocks().all(|b| b.is_standard())
    }

    /// Returns an iterator over all subtitle blocks.
    ///
    /// Subtitle block is a slice of lines: &[a non-blank line..a non blank line after a blank line].
    /// Last block is a slice: &[a non-blank line..=last line].
    pub fn blocks(&self) -> impl Iterator<Item = SrtBlock<'_>> {
        self.blocks_ranges().map(|rng| SrtBlock(&self[rng]))
    }

    fn blocks_ranges(&self) -> impl Iterator<Item = Range<usize>> {
        let mut was_blank = false;
        let mut start = 0;
        let mut end = 0;
        let len = self.len();
        iter::from_fn(move || {
            while end < len {
                let is_blank = self[end].ty.is_blank();
                if !is_blank && was_blank {
                    break;
                }
                was_blank = is_blank;
                end += 1;
            }
            if end == start {
                None
            } else {
                let st = start;
                start = end;
                was_blank = false;
                Some(st..end)
            }
        })
    }
}

fn set_newline_position(pos: &mut usize, data: &[u8], len: usize) {
    while *pos < len {
        match data[*pos] {
            b'\r' if data.get(*pos + 1) == Some(&b'\n') => {
                *pos += 2;
                break;
            }
            b'\r' | b'\n' => {
                *pos += 1;
                break;
            }
            _ => *pos += 1,
        }
    }
}
