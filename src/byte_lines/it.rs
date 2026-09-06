use super::{ByteLines, ByteLinesTy};
use std::io::BufRead;

impl<T: BufRead> crate::StreamingIterator for ByteLines<'_, T> {
    type Item<'a>
        = &'a [u8]
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        match &mut self.ty {
            ByteLinesTy::ByteSlice(bytes) => get_slice_line(bytes, &mut self.pos),
            ByteLinesTy::BufReader(reader) => {
                reader.consume(self.pos);
                self.pos = 0;

                let internal_buf = reader.fill_buf().ok()?;
                let line = get_slice_line(internal_buf, &mut self.pos)?;
                let line_len = line.len();

                if line < internal_buf || line_len == 0 {
                    let internal_buf = reader.fill_buf().ok()?;
                    return Some(&internal_buf[..line_len]);
                }

                self.buf.clear();
                loop {
                    self.pos = 0;
                    let internal_buf = reader.fill_buf().ok()?;

                    let line = match get_slice_line(internal_buf, &mut self.pos) {
                        Some(l) => l,
                        None => break,
                    };
                    self.buf.extend_from_slice(line);

                    let is_end = line < internal_buf || line.len() == 0;
                    reader.consume(self.pos);

                    if is_end {
                        break;
                    }
                }

                Some(&self.buf)
            }
        }
    }
}

fn get_slice_line<'a>(data: &'a [u8], pos: &mut usize) -> Option<&'a [u8]> {
    if *pos >= data.len() {
        return None;
    }

    let start = *pos;
    let mut newline_is_r = false;
    while *pos < data.len() {
        match data[*pos] {
            b'\r' => {
                newline_is_r = true;
                break;
            }
            b'\n' => break,
            _ => *pos += 1,
        }
    }
    let end = *pos;

    *pos += 1;
    if newline_is_r && data.get(*pos).is_some_and(|b| matches!(b, b'\n')) {
        *pos += 1;
    }

    Some(&data[start..end])
}
