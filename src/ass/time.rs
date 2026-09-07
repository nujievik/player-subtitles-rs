use crate::Time;
use core::fmt::NumBuffer;

pub struct AssTime {
    buf: [u8; 10],
    time: Time,
}

impl AssTime {
    pub fn new(mut time: Time) -> AssTime {
        time.hours = time.hours.min(9);
        if time.millis > 99 {
            time.millis = time.millis / 10;
        }

        AssTime {
            buf: *b"0:00:00:00",
            time,
        }
    }
}

impl AssTime {
    pub fn format_using(&mut self, buf: &mut NumBuffer<u16>) -> &[u8] {
        let hours_bytes = self.time.hours.format_into(buf).as_bytes();
        self.buf[0] = hours_bytes[0];

        let mut write_two_digits = |start_idx: usize, value: u16| {
            let bytes = value.format_into(buf).as_bytes();
            match bytes.len() {
                1 => {
                    self.buf[start_idx] = b'0';
                    self.buf[start_idx + 1] = bytes[0];
                }
                _ => self.buf[start_idx..start_idx + 2].copy_from_slice(bytes),
            }
        };

        write_two_digits(2, self.time.mins as u16);
        write_two_digits(5, self.time.secs as u16);
        write_two_digits(8, self.time.millis as u16);

        &self.buf
    }
}
