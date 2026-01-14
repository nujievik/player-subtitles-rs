# player-subtitles

A subtitle library focused on player-compatible subtitle files.

Unlike many other crates player-subtitles works with raw bytes, 
correctly processing an ASCII-compatible encoding subtitles (UTF-8,
Windows-1251, etc.).

[![Tests](https://github.com/nujievik/player-subtitles-rs/actions/workflows/tests.yml/badge.svg)](
https://github.com/nujievik/player-subtitles-rs/actions/workflows/tests.yml)

## Format supported
- SRT (SubRip)

TODO: Add support for other formats.

## Constructors
The library provides fallible and infallible constructors. This allows
not only ensures default normalization, but implements
user-normalization also. See documentation for details.

## Features
- Allows writing files byte-for-byte identical to the original
- Borrowed constructors from a bytes or string and owned constructors
from a file
- Clone-on-write architecture with minimal allocations
- Default standardization method
- Fallible and infallible constructors
- Line-by-line subtitle representation.
- Reading from and writing to files
- Support for ASCII-compatible encodings (UTF-8, Windows-1251, etc.)
