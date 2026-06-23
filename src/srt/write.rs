use super::line::{BytesNumber, BytesText};
use crate::{Result, SrtLine, SrtLines, StreamingIterator, Time, WriteLines, WriteOptions};
use std::io::{BufRead, Write};

impl<'a, T: BufRead> WriteLines for SrtLines<'a, T> {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(crate::BOM)?;
        }

        let mut number = 1usize;
        let mut is_wrote_header = false;
        let mut time_range: Option<(Time, Time)> = None;

        while let Some(line) = self.next() {
            match line {
                SrtLine::Blank => {
                    is_wrote_header = false;
                    time_range = None;
                }
                SrtLine::TimeRange(bs) => {
                    let mut start = bs.start();
                    let mut end = bs.end();

                    if opts.start_from.is_some_and(|t| end <= t) {
                        continue;
                    }
                    if opts.end_on.is_some_and(|t| start >= t) {
                        continue;
                    }
                    if let Some(add) = opts.add_time {
                        start += add;
                        end += add;
                    }
                    if let Some(sub) = opts.sub_time {
                        start -= sub;
                        end -= sub;
                    }

                    time_range = Some((start, end));
                }
                SrtLine::Number(BytesNumber { bytes }) | SrtLine::Text(BytesText { bytes }) => {
                    if !is_wrote_header {
                        if let Some((start, end)) = time_range {
                            if number > 1 {
                                writer.write("\n".as_bytes())?;
                            }
                            writer.write(format!("{}\n", number).as_bytes())?;
                            writer.write(
                                format!("{} --> {}\n", start.into_srt(), end.into_srt()).as_bytes(),
                            )?;
                            number += 1;
                            is_wrote_header = true;
                        } else {
                            continue;
                        }
                    }
                    writer.write(bytes)?;
                    writer.write("\n".as_bytes())?;
                }
            }
        }

        Ok(())
    }
}
