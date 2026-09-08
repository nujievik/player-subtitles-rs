use super::Time;
use core::fmt::NumBuffer;

pub struct TimeBuf<const N: usize> {
    pub(crate) num_buf: NumBuffer<u16>,
    pub(crate) buf: [u8; N],
}

pub type AssTimeBuf = TimeBuf<10>;
pub struct SrtTimeBuf(pub TimeBuf<12>);
pub struct VttTimeBuf(pub TimeBuf<12>);

impl AssTimeBuf {
    pub const fn new() -> AssTimeBuf {
        Self {
            num_buf: NumBuffer::new(),
            buf: *b"0:00:00:00",
        }
    }

    pub fn format_time(&mut self, mut time: Time) -> &[u8] {
        time.hours = time.hours.min(9);
        if time.millis > 99 {
            time.millis = time.millis / 10;
        }

        let hours_bytes = time.hours.format_into(&mut self.num_buf).as_bytes();
        self.buf[0] = hours_bytes[0];
        self.write_two_digits(2, time.mins as u16);
        self.write_two_digits(5, time.secs as u16);
        self.write_two_digits(8, time.millis);

        &self.buf
    }
}

impl SrtTimeBuf {
    pub const fn new() -> SrtTimeBuf {
        Self(TimeBuf {
            num_buf: NumBuffer::new(),
            buf: *b"00:00:00,000",
        })
    }
}

impl VttTimeBuf {
    pub const fn new() -> VttTimeBuf {
        Self(TimeBuf {
            num_buf: NumBuffer::new(),
            buf: *b"00:00:00.000",
        })
    }
}

impl TimeBuf<12> {
    pub fn format_time(&mut self, mut time: Time) -> &[u8] {
        time.hours = time.hours.min(99);

        self.write_two_digits(0, time.hours);
        self.write_two_digits(3, time.mins as u16);
        self.write_two_digits(6, time.secs as u16);
        self.write_three_digits(9, time.millis);

        &self.buf
    }
}

impl<const N: usize> TimeBuf<N> {
    fn write_two_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = bytes[0];
            }
            _ => self.buf[start_idx..start_idx + 2].copy_from_slice(bytes),
        }
    }

    fn write_three_digits(&mut self, start_idx: usize, value: u16) {
        let bytes = value.format_into(&mut self.num_buf).as_bytes();
        match bytes.len() {
            1 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1] = b'0';
                self.buf[start_idx + 2] = bytes[0];
            }
            2 => {
                self.buf[start_idx] = b'0';
                self.buf[start_idx + 1..start_idx + 3].copy_from_slice(bytes);
            }
            _ => self.buf[start_idx..start_idx + 3].copy_from_slice(bytes),
        }
    }
}
