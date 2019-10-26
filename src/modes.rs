use libc::c_int;
use std::convert::TryFrom;

use crate::Error;

/// Available modes for osu!
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Std,
    Taiko,
    // unimplemented!
    // Mania,
    // CTB,
}

impl From<Mode> for c_int {
    /// See https://github.com/Francesco149/oppai-ng/blob/71103a07954b403bc502120a4a752574491ab24b/oppai.c#L162
    fn from(m: Mode) -> Self {
        match m {
            Mode::Std => 0,
            Mode::Taiko => 1,
        }
    }
}

impl TryFrom<c_int> for Mode {
    type Error = Error;
    fn try_from(value: c_int) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Mode::Std,
            1 => Mode::Taiko,
            _ => return Err(Error::OppaiUnknownMode(value)),
        })
    }
}
