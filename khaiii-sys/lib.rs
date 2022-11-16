#![allow(non_camel_case_types, unused_extern_crates)]
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct khaiii_morph {
    pub lex: *const c_char,
    pub tag: *const c_char,
    pub begin: c_int,
    pub length: c_int,
    pub reserved: [c_char; 8usize],
    pub next: *const khaiii_morph,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct khaiii_word {
    pub begin: c_int,
    pub length: c_int,
    pub reserved: [c_char; 8usize],
    pub morphs: *const khaiii_morph,
    pub next: *const khaiii_word,
}

extern "C" {
    pub fn khaiii_analyze(
        handle: c_int,
        input: *const c_char,
        opt_str: *const c_char,
    ) -> *const khaiii_word;
    pub fn khaiii_analyze_bfr_errpatch(
        handle: c_int,
        input: *const c_char,
        opt_str: *const c_char,
        output: *mut i16,
    ) -> c_int;
    pub fn khaiii_close(handle: c_int);
    pub fn khaiii_free_results(handle: c_int, results: *const khaiii_word);
    pub fn khaiii_open(rsc_dir: *const c_char, opt_str: *const c_char) -> c_int;
    pub fn khaiii_last_error(handle: c_int) -> *const c_char;
    pub fn khaiii_set_log_level(name: *const c_char, level: *const c_char) -> c_int;
    pub fn khaiii_set_log_levels(name_level_pairs: *const c_char) -> c_int;
    pub fn khaiii_version() -> *const c_char;
}

#[doc(hidden)]
pub fn vendored() -> bool {
    cfg!(khaiii_vendored)
}
