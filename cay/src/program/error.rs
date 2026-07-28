use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Buffer is not a DarwiNN package: the flatbuffer file identifier is not "DWN1".
    BadIdentifier,
    /// A flatbuffer field the reader needs was absent.
    MissingField(&'static str),
    /// A hand-walked flatbuffer region ran past the buffer or overflowed.
    Malformed(&'static str),
    /// The planus decoder rejected the buffer.
    Decode(planus::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::BadIdentifier => write!(f, "not a DarwiNN package (bad file identifier)"),
            Error::MissingField(name) => write!(f, "missing required field: {name}"),
            Error::Malformed(what) => write!(f, "malformed flatbuffer: {what}"),
            Error::Decode(e) => write!(f, "flatbuffer decode error: {e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<planus::Error> for Error {
    fn from(e: planus::Error) -> Self {
        Error::Decode(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
