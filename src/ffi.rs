use libc;
/// The opaque struct ezpp.
/// See https://github.com/Francesco149/oppai-ng/blob/71103a07954b403bc502120a4a752574491ab24b/oppai.c#L32
#[repr(C)]
pub struct ezpp {
    _private: [u8; 0],
}

extern "C" {
    /// Creates a new ezpp struct.
    pub fn ezpp_new() -> *mut ezpp;
    /// Frees an ezpp struct.
    pub fn ezpp_free(ez: *mut ezpp) -> ();
    /// Load a map.
    // pub fn ezpp(ez: *mut ezpp, map: *const libc::c_char) -> libc::c_int;
    pub fn ezpp_data(
        ez: *mut ezpp,
        data: *const libc::c_char,
        data_size: libc::c_int,
    ) -> libc::c_int;
    /// Set mods.
    pub fn ezpp_set_mods(ez: *mut ezpp, mods: libc::c_int) -> ();
    /// Get and set mode.
    pub fn ezpp_mode(ez: *const ezpp) -> libc::c_int;
    // Allows conversion from Std to others.
    pub fn ezpp_set_mode_override(ez: *mut ezpp, mode_override: libc::c_int) -> ();
    pub fn ezpp_set_mode(ez: *mut ezpp, mode: libc::c_int) -> ();
    /// Set combo.
    pub fn ezpp_max_combo(ez: *const ezpp) -> libc::c_int;
    pub fn ezpp_set_combo(ez: *mut ezpp, combo: libc::c_int) -> ();
    /// Set misses.
    pub fn ezpp_set_nmiss(ez: *mut ezpp, nmiss: libc::c_int) -> ();
    /// Set accuracy
    pub fn ezpp_set_accuracy_percent(ez: *mut ezpp, nmiss: libc::c_float) -> ();
    pub fn ezpp_set_accuracy(ez: *mut ezpp, n100: libc::c_int, n50: libc::c_int) -> ();

    /// Get pp.
    pub fn ezpp_pp(ez: *const ezpp) -> libc::c_float;
    /// Get stars.
    pub fn ezpp_stars(ez: *const ezpp) -> libc::c_float;
    /// Get object count.
    pub fn ezpp_nobjects(ez: *const ezpp) -> libc::c_int;
}
