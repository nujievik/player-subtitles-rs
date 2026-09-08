use super::line::{Number, Text};
use crate::{
    Result, SrtLine, SrtLines, StreamingIterator, Time, WriteLines, WriteOptions,
    time::bufs::SrtTimeBuf,
};
use core::fmt::NumBuffer;
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
        let mut is_written_header = false;
        let mut time_range: Option<(Time, Time)> = None;

        let mut num_buf: NumBuffer<usize> = NumBuffer::new();
        let mut time_buf = SrtTimeBuf::new();

        while let Some(line) = self.next() {
            match line {
                SrtLine::Blank => {
                    is_written_header = false;
                    time_range = None;
                }
                SrtLine::TimeRange(bs) => {
                    let mut start = bs.start();
                    let mut end = bs.end();

                    if opts.start.is_some_and(|t| end <= t) || opts.end.is_some_and(|t| start >= t)
                    {
                        is_written_header = false;
                        time_range = None;
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
                SrtLine::Number(Number { bytes }) | SrtLine::Text(Text { bytes }) => {
                    if !is_written_header {
                        if let Some((start, end)) = time_range {
                            if number > 1 {
                                writer.write(b"\n")?;
                            }
                            writer.write(number.format_into(&mut num_buf).as_bytes())?;
                            writer.write(b"\n")?;
                            writer.write(time_buf.0.format_time(start))?;
                            writer.write(b" --> ")?;
                            writer.write(time_buf.0.format_time(end))?;
                            writer.write(b"\n")?;

                            number += 1;
                            is_written_header = true;
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
