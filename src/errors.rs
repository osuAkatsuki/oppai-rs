use std::{error, fmt, path::PathBuf};

/// The result type.
pub type Result<T> = std::result::Result<T, Error>;

/// A generic error.
#[derive(Debug)]
pub enum Error {
    // From Oppai
    OppaiReturns(OppaiError),
    OppaiFails(&'static str),
    /// Unknown mode returned from Oppai
    OppaiUnknownMode(libc::c_int),

    // From user
    InvalidPath(PathBuf),
    /// Path contains a \0
    InvalidPathNull(std::ffi::NulError),
    /// Cannot convert non-standard mode to another mode.
    CannotConvertMode,
    /// max_combo invalid, # of misses invalid
    InvalidCombo,
    /// Accuracy not between 0 and 100
    InvalidAccuracy,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match self {
            Error::InvalidPathNull(err) => Some(err),
            _ => None,
        }
    }
}

/// An error returned from Oppai.
/// See https://github.com/Francesco149/oppai-ng/blob/71103a07954b403bc502120a4a752574491ab24b/oppai.c#L137
#[derive(Copy, Clone, Debug)]
pub enum OppaiError {
    /// ERR_MORE
    ErrMore = -1,
    /// ERR_SYNTAX
    ErrSyntax = -2,
    /// ERR_TRUNCATED
    ErrTruncated = -3,
    /// ERR_NOTIMPLEMENTED
    ErrNotImplemented = -4,
    /// ERR_IO
    ErrIO = -5,
    /// ERR_FORMAT
    ErrFormat = -6,
    /// ERR_OOM
    ErrOOM = -7,
}

impl OppaiError {
    /// Resolves a libc returns from a call into a Result.
    pub(crate) fn resolve(x: libc::c_int) -> Result<()> {
        use OppaiError::*;
        Err(Error::OppaiReturns(match x {
            0 => return Ok(()),
            -1 => ErrMore,
            -2 => ErrSyntax,
            -3 => ErrTruncated,
            -4 => ErrNotImplemented,
            -5 => ErrIO,
            -6 => ErrFormat,
            -7 => ErrOOM,
            _ => return Err(Error::OppaiFails("Unknown error code")),
        }))
    }
}
