use super::line::{Event, SectionMark};
use super::{AssLine, AssLines};
use crate::{Result, StreamingIterator, WriteLines, WriteOptions};
use std::io::{BufRead, Write};

impl<'a, T: BufRead> WriteLines for AssLines<'a, T> {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(crate::BOM)?;
        }

        let is_setted_time = opts.start_from.is_some()
            || opts.end_on.is_some()
            || opts.add_time.is_some()
            || opts.sub_time.is_some();
        let mut is_wrote_blank = false;
        let mut event_buf: Vec<u8> = Vec::new();

        while let Some(mut line) = self.next() {
            let bytes: &[u8] = match &mut line {
                AssLine::Blank => {
                    if !is_wrote_blank {
                        writer.write(b"\n")?;
                        is_wrote_blank = true;
                    }
                    continue;
                }
                AssLine::SectionMark(mark) => mark.as_bytes(),
                AssLine::Comment(comment) => comment.bytes,
                AssLine::ScriptInfo(info) => info.as_bytes(),
                AssLine::Style(style) => style.as_bytes(),
                AssLine::EventFormat(format) => format.as_bytes(),
                AssLine::Event(event) => {
                    if opts.start_from.is_some_and(|t| event.end <= t)
                        || opts.end_on.is_some_and(|t| event.start >= t)
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

                    event_buf.clear();
                    event.write_into_buf(&mut event_buf)?;
                    event_buf.as_slice()
                }
                AssLine::Unrecognized(bytes) => bytes,
            };
            writer.write(bytes)?;
            writer.write(b"\n")?;
            is_wrote_blank = false;
        }

        Ok(())
    }
}

impl<'a> Event<'a> {
    fn write_into_buf(&self, buf: &mut Vec<u8>) -> Result<()> {
        buf.extend_from_slice(self.ty.as_bytes());
        buf.extend_from_slice(b": ");

        buf.extend_from_slice(self.layer.to_string().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.start.into_ass().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.end.into_ass().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.style_name);
        buf.push(b',');
        buf.extend_from_slice(self.character_name);
        buf.push(b',');

        buf.extend_from_slice(self.margin_l.to_string().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.margin_r.to_string().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.margin_v.to_string().as_bytes());
        buf.push(b',');

        buf.extend_from_slice(self.effect.to_string().as_bytes());
        buf.push(b',');
        buf.extend_from_slice(self.text);

        Ok(())
    }
}
