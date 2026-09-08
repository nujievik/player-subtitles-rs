use super::{
    AssLines,
    line::{AssLine, Event},
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
                AssLine::Style(style) => style.as_bytes(),
                AssLine::EventFormat(format) => format.as_bytes(),
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

                    event.write_into_writer(writer, &mut time_buf)?;
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
    fn write_into_writer<W>(&self, writer: &mut W, buf: &mut AssTimeBuf) -> Result<()>
    where
        W: Write + ?Sized,
    {
        writer.write(self.ty.as_bytes())?;
        writer.write(b": ")?;

        writer.write(self.layer.format_into(&mut buf.num_buf).as_bytes())?;
        writer.write(b",")?;
        writer.write(buf.format_time(self.start))?;
        writer.write(b",")?;
        writer.write(buf.format_time(self.end))?;
        writer.write(b",")?;

        writer.write(self.style_name)?;
        writer.write(b",")?;
        writer.write(self.character_name)?;
        writer.write(b",")?;

        writer.write(self.margin_l.format_into(&mut buf.num_buf).as_bytes())?;
        writer.write(b",")?;
        writer.write(self.margin_r.format_into(&mut buf.num_buf).as_bytes())?;
        writer.write(b",")?;
        writer.write(self.margin_v.format_into(&mut buf.num_buf).as_bytes())?;
        writer.write(b",")?;
        writer.write(self.effect.as_bytes())?;
        writer.write(b",")?;
        writer.write(self.text)?;
        writer.write(b"\n")?;

        Ok(())
    }
}
