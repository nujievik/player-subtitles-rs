use std::{error, fmt, io};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Io(io::Error),
    NonStandardBlock((usize, &'static str)),
    ValueValidation(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::NonStandardBlock((i, e)) => write!(
                f,
                "a block that starts at line {} is non-standard: {}",
                i, e
            ),
            Self::ValueValidation(e) => write!(f, "value validation: {}", e),
        }
    }
}
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Self::Io(e)
    }
}
