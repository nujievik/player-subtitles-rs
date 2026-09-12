use super::{
    AssLines,
    line::{AssLine, Event, EventFormatPositions},
};
use crate::{Result, StreamingIterator, WriteLines, WriteOptions, time::bufs::AssTimeBuf};
use std::io::{BufRead, Write};

impl<'a, T: BufRead> WriteLines for AssLines<'a, T> {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(crate::BOM)?;
        }

        let mut is_written_blank = false;
        let mut time_buf = AssTimeBuf::new();
        let mut positions = EventFormatPositions::new();

        while let Some(mut line) = self.next() {
            let bytes: &[u8] = match &mut line {
                AssLine::Blank => {
                    if !is_written_blank {
                        writer.write(b"\n")?;
                        is_written_blank = true;
                    }
                    continue;
                }
                AssLine::SectionMark(mark) => mark.as_bytes(),
                AssLine::Comment(comment) => comment.bytes,
                AssLine::ScriptInfo(info) => info.as_bytes(),
                AssLine::EventFormat(format) => {
                    positions = *format.positions();
                    format.as_bytes()
                }
                AssLine::Event(event) => {
                    if opts.start.is_some_and(|t| event.end <= t)
                        || opts.end.is_some_and(|t| event.start >= t)
                    {
                        continue;
                    }
                    if let Some(add) = opts.add_time {
                        event.start += add;
                        event.end += add;
                    }
                    if let Some(sub) = opts.sub_time {
                        event.start -= sub;
                        event.end -= sub;
                    }

                    event.write_into_writer(writer, &mut time_buf, &positions)?;
                    is_written_blank = false;
                    continue;
                }
                AssLine::Unrecognized(bytes) => bytes,
            };
            writer.write(bytes)?;
            writer.write(b"\n")?;
            is_written_blank = false;
        }

        Ok(())
    }
}

impl<'a> Event<'a> {
    fn write_into_writer<W>(
        &self,
        writer: &mut W,
        buf: &mut AssTimeBuf,
        positions: &EventFormatPositions,
    ) -> Result<()>
    where
        W: Write + ?Sized,
    {
        writer.write(self.ty.as_bytes())?;
        writer.write(b": ")?;

        for i in 0..EventFormatPositions::NUMBER_OF_FIELDS {
            let bytes = match i {
                i if i == positions.layer() => self.layer.format_into(&mut buf.num_buf).as_bytes(),
                i if i == positions.start() => buf.format_time(self.start),
                i if i == positions.end() => buf.format_time(self.end),
                i if i == positions.style_name() => self.style_name,
                i if i == positions.character_name() => self.character_name,
                i if i == positions.margin_l() => {
                    self.margin_l.format_into(&mut buf.num_buf).as_bytes()
                }
                i if i == positions.margin_r() => {
                    self.margin_r.format_into(&mut buf.num_buf).as_bytes()
                }
                i if i == positions.margin_v() => {
                    self.margin_v.format_into(&mut buf.num_buf).as_bytes()
                }
                i if i == positions.effect() => self.effect,
                _ => continue,
            };
            writer.write(bytes)?;
            writer.write(b",")?;
        }

        writer.write(self.text)?;
        writer.write(b"\n")?;

        Ok(())
    }
}
