use crate::{Error, Result};
use std::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

const THOUSAND: u16 = 1000;
const SIXTY: u8 = 60;

/// A subtitle timestamp.
///
/// Each [`Time`] must ensures:
/// - minutes < 60
/// - seconds < 60
/// - milliseconds < 1000
///
/// Use [`Time::new`] for correct construct.
#[derive(Copy, Clone, Debug, Default, Ord, Eq, PartialOrd, PartialEq)]
pub struct Time {
    pub hours: u16,
    pub mins: u8,
    pub secs: u8,
    pub millis: u16,
}

impl Time {
    /// Tries construct a new [`Time`].
    ///
    /// # Errors
    ///
    /// Returns an error in the next cases:
    /// - minutes >= 60
    /// - seconds >= 60
    /// - milliseconds >= 1000
    pub const fn new(hours: u16, mins: u8, secs: u8, millis: u16) -> Result<Time> {
        if mins >= SIXTY {
            Err(Error::ValueValidation("minutes must be < 60"))
        } else if secs >= SIXTY {
            Err(Error::ValueValidation("seconds must be < 60"))
        } else if millis >= THOUSAND {
            Err(Error::ValueValidation("milliseconds must be < 1000"))
        } else {
            Ok(Self::new_unchecked(hours, mins, secs, millis))
        }
    }

    /// Constructs a new [`Time`] without checks. User must ensures:
    /// - minutes < 60
    /// - seconds < 60
    /// - milliseconds < 1000
    pub const fn new_unchecked(hours: u16, mins: u8, secs: u8, millis: u16) -> Time {
        Self {
            hours,
            mins,
            secs,
            millis,
        }
    }

    pub(crate) fn to_srt(self) -> String {
        format!(
            "{:02}:{:02}:{:02},{:03}",
            self.hours,
            self.mins,
            self.secs,
            self.millis % 1000
        )
    }
}

impl From<Duration> for Time {
    fn from(dur: Duration) -> Time {
        let millis = dur.subsec_millis() as u16;
        let total_secs = dur.as_secs();
        let sixty_u64 = SIXTY as u64;

        let secs = (total_secs % sixty_u64) as u8;
        let total_mins = total_secs / sixty_u64;

        let mins = (total_mins % sixty_u64) as u8;
        let hours = (total_mins / sixty_u64) as u16;

        Time::new_unchecked(hours, mins, secs, millis)
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (millis, add) = match self.millis + other.millis {
            x if x < THOUSAND => (x, 0),
            x => (x - THOUSAND, 1),
        };
        let (secs, add) = match self.secs + other.secs + add {
            x if x < SIXTY => (x, 0),
            x => (x - SIXTY, 1),
        };
        let (mins, add) = match self.mins + other.mins + add {
            x if x < SIXTY => (x, 0),
            x => (x - SIXTY, 1),
        };

        Self {
            hours: self.hours + other.hours + add,
            mins,
            secs,
            millis,
        }
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (millis, borrow) = if self.millis >= other.millis {
            (self.millis - other.millis, 0)
        } else {
            (self.millis + THOUSAND - other.millis, 1)
        };

        let osecs = other.secs + borrow;
        let (secs, borrow) = if self.secs >= osecs {
            (self.secs - osecs, 0)
        } else {
            (self.secs + SIXTY - osecs, 1)
        };

        let omins = other.mins + borrow;
        let (mins, borrow) = if self.mins >= omins {
            (self.mins - omins, 0)
        } else {
            (self.mins + SIXTY - omins, 1)
        };

        Self {
            hours: self.hours - other.hours - borrow,
            mins,
            secs,
            millis,
        }
    }
}

impl Add<Duration> for Time {
    type Output = Self;

    fn add(self, dur: Duration) -> Self {
        self.add(Time::from(dur))
    }
}
impl Sub<Duration> for Time {
    type Output = Self;

    fn sub(self, dur: Duration) -> Self {
        self.sub(Time::from(dur))
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
impl SubAssign for Time {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}
