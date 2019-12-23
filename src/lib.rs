//! Provides a safe wrapper for [`oppai-ng`](https://github.com/Francesco149/oppai-ng).
//!
//! # Example
//!
//! ```ignore
//! extern crate oppai_rs;
//! use oppai_rs::*;
//! use std::path::Path;
//! let map = {
//!   Oppai::new(Path::new("path/to/map"))
//!             .mods(Mods::HD | Mods::DT)?
//!             .combo(Combo::PERFECT)?
//!             .accuracy(100.0)
//! }.expect("OK!");
//! let pp = map.pp();
//! let stars = map.stars();
//! ```
extern crate bitflags;
extern crate libc;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod errors;
mod ffi; // The ffi signatures.
mod modes;
mod mods;

#[cfg(test)]
mod tests;

pub use errors::{Error, Result};
pub use modes::Mode;
pub use mods::Mods;

use errors::OppaiError;
use std::{convert::TryInto, ffi::CString, path::Path};

/// The main oppai struct, which is a thin wrapper that provides safe
/// API to the linked ezpp_* functions.
pub struct Oppai {
    map_content: CString,
    map_mode: Mode,
    max_combo: u32,

    // Delay the application because oppai is dumb
    combo: Option<Combo>,
    accuracy: Option<f32>,
    mode: Option<Mode>,
    mods: Option<Mods>,
}

impl Oppai {
    /// Creates a new, empty Oppai from a map.
    pub fn new(path_to_map: &Path) -> Result<Oppai> {
        let file = std::fs::read(path_to_map)?;
        Self::new_from_content(CString::new(file)?)
    }

    /// Creates a new Oppai from a map's content.
    pub fn new_from_content(content: impl Into<CString>) -> Result<Oppai> {
        Self::load_map(content.into())
    }

    /// Creates a new oppai and loads a new map into it.
    fn load_map(content: CString) -> Result<Oppai> {
        // Extract the Path into a *const u8
        // Construct a *mut ezpp to collect map data
        let (map_mode, max_combo) = {
            let p = unsafe { ffi::ezpp_new() };
            OppaiError::resolve(unsafe {
                ffi::ezpp_data(p, content.as_ptr(), content.as_bytes().len() as libc::c_int)
            })?;
            // Get the map's information
            let x = (
                unsafe { ffi::ezpp_mode(p) }.try_into()?,
                unsafe { ffi::ezpp_max_combo(p) } as u32,
            );
            unsafe {
                ffi::ezpp_free(p);
            }
            x
        };

        Ok(Oppai {
            map_content: content,
            map_mode,
            max_combo,

            combo: None,
            accuracy: None,
            mode: None,
            mods: None,
        })
    }

    /// Sets the mods for the play.
    pub fn mods(&mut self, mods: Mods) -> &mut Self {
        self.mods = Some(mods);
        self
    }
    fn set_mods(&self, ezpp: *mut ffi::ezpp, mods: Mods) {
        unsafe { ffi::ezpp_set_mods(ezpp, mods.bits()) }
    }

    /// Sets the mode for the play.
    /// A play can be mode-set if and only if its original map is Std.
    pub fn mode(&mut self, mode: Mode) -> Result<&mut Self> {
        match self.map_mode {
            Mode::Std => Ok(()),
            v if mode == v => Ok(()),
            _ => Err(Error::CannotConvertMode),
        }?;
        self.mode = Some(mode);
        Ok(self)
    }
    fn set_mode(&self, ezpp: *mut ffi::ezpp, mode: Mode) {
        match self.map_mode {
            Mode::Std => {
                unsafe {
                    ffi::ezpp_set_mode_override(ezpp, if mode == Mode::Std { 0 } else { 1 });
                    ffi::ezpp_set_mode(ezpp, mode.into());
                };
            }
            _ => (),
        }
    }

    /// Gets the maximum possible combo of a map.
    pub fn max_combo(&self) -> u32 {
        self.max_combo
    }

    /// Sets the combo.
    pub fn combo(&mut self, combo: Combo) -> Result<&mut Self> {
        match combo {
            Combo::FC(slider_ends_missed) if slider_ends_missed < self.max_combo() => Ok(()),
            Combo::NonFC { max_combo, misses }
                if (max_combo as u64) + (misses as u64) <= self.max_combo() as u64 =>
            {
                Ok(())
            }

            _ => Err(Error::InvalidCombo),
        }?;
        self.combo = Some(combo);
        Ok(self)
    }
    fn set_combo(&self, ezpp: *mut ffi::ezpp, combo: Combo) {
        match combo {
            Combo::FC(slider_ends_missed) => unsafe {
                ffi::ezpp_set_combo(ezpp, (self.max_combo() - slider_ends_missed) as i32)
            },
            Combo::NonFC { max_combo, misses } => unsafe {
                ffi::ezpp_set_combo(ezpp, max_combo as i32);
                ffi::ezpp_set_nmiss(ezpp, misses as i32);
            },
        };
    }

    /// Sets the accuracy.
    pub fn accuracy(&mut self, accuracy: f32) -> Result<&mut Self> {
        if accuracy < 0.0 || accuracy > 100.0 {
            return Err(Error::InvalidAccuracy);
        } else {
            self.accuracy = Some(accuracy);
            Ok(self)
        }
    }
    fn set_accuracy(&self, ezpp: *mut ffi::ezpp, accuracy: f32) {
        unsafe { ffi::ezpp_set_accuracy_percent(ezpp, accuracy) }
    }

    /// PP of the play.
    pub fn pp(&self) -> f32 {
        self.run().0
    }

    /// Star difficulty of the play.
    pub fn stars(&self) -> f32 {
        self.run().1
    }

    /// Runs oppai and returns the pp and star difficulty of the play.
    pub fn run(&self) -> (f32, f32) {
        let ezpp = unsafe { ffi::ezpp_new() };
        if let Some(v) = self.combo {
            self.set_combo(ezpp, v)
        }
        if let Some(v) = self.mode {
            self.set_mode(ezpp, v)
        }
        if let Some(v) = self.accuracy {
            self.set_accuracy(ezpp, v)
        }
        if let Some(v) = self.mods {
            self.set_mods(ezpp, v)
        }
        unsafe {
            ffi::ezpp_data(
                ezpp,
                self.map_content.as_ptr(),
                self.map_content.as_bytes().len() as libc::c_int,
            );
            let res = (ffi::ezpp_pp(ezpp), ffi::ezpp_stars(ezpp));
            ffi::ezpp_free(ezpp);
            res
        }
    }
}

/// Combo can be an FC or a non-FC with max combo and # of misses.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Combo {
    /// Full combo, no misses, u32 is the # of slider ends missed.
    FC(u32),
    /// Not a full combo.
    NonFC { max_combo: u32, misses: u32 },
}

impl Combo {
    /// A maximal combo FC.
    pub const PERFECT: Combo = Combo::FC(0);
    /// Constructs a non-fc Combo.
    pub fn non_fc(max_combo: u32, misses: u32) -> Combo {
        Combo::NonFC { max_combo, misses }
    }
}
