use super::{VttLines, VttSourceLines, line::CueId};
use crate::{Result, StreamingIterator, VttLine, WriteLines, WriteOptions};
use std::io::{BufRead, Write};

impl<'a, T: BufRead> WriteLines for VttLines<'a, T> {
    fn write_to_writer_with<W>(&mut self, writer: &mut W, opts: &WriteOptions) -> Result<()>
    where
        W: Write + ?Sized,
    {
        if opts.bom {
            writer.write(crate::BOM)?;
        }
        writer.write(b"WEBVTT\n\n")?;

        let is_srt_source = matches!(&*self.source, VttSourceLines::Srt(_));
        let is_setted_time = opts.start_from.is_some()
            || opts.end_on.is_some()
            || opts.add_time.is_some()
            || opts.sub_time.is_some();
        let (mut cue_id_buf, mut time_buf): (Vec<u8>, Vec<u8>) = if is_setted_time {
            (Vec::with_capacity(32), Vec::with_capacity(31))
        } else {
            (Vec::new(), Vec::new())
        };
        let mut is_wrote_blank = true;

        while let Some(line) = self.next() {
            let bytes: &[u8] = match &line {
                VttLine::VttFileMark(_) => continue,
                VttLine::Blank => {
                    if !is_wrote_blank {
                        writer.write(b"\n")?;
                        is_wrote_blank = true;
                    }
                    continue;
                }
                VttLine::CueId(CueId { bytes }) if is_setted_time => {
                    cue_id_buf.clear();
                    cue_id_buf.extend_from_slice(bytes);
                    continue;
                }
                VttLine::TimeRangeAndStyle(tr) if is_srt_source || is_setted_time => {
                    let mut start = tr.start;
                    let mut end = tr.end;

                    if opts.start_from.is_some_and(|t| end <= t)
                        || opts.end_on.is_some_and(|t| start >= t)
                    {
                        cue_id_buf.clear();
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

                    if !cue_id_buf.is_empty() {
                        writer.write(&cue_id_buf)?;
                        cue_id_buf.clear();
                    }

                    time_buf.clear();
                    write!(&mut time_buf, "{} --> {}", start.to_vtt(), end.to_vtt())?;
                    time_buf.as_slice()
                }
                line => line.as_bytes(),
            };
            writer.write(bytes)?;
            writer.write(b"\n")?;
            is_wrote_blank = false;
        }

        Ok(())
    }
}
