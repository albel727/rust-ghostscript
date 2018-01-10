// Doesn't offer anything that UTF-8 doesn't solve.
// This was originally supposed to support WTF-16 encoding on Windows,
// to better handle windows broken file names, but it turned out
// that Ghostscript library internally converts everything to UTF-8
// anyway, so weird file names don't make it unscathed until _wfopen() call.
// I'm just gonna leave this module here for history, and in the hope
// that Ghostscript will eventually support WTF-16.

use gs_sys;
use std::ffi::OsStr;

extern crate widestring;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Utf16;

impl ::encoding::StringEncoding for Utf16 {
    const GHOSTSCRIPT_ENCODING: gs_sys::GsArgEncoding = gs_sys::encoding::UTF16LE;
    type RustType = OsStr;
    type FfiType = widestring::WideCString;

    fn from_rust_to_ffi<S: AsRef<Self::RustType>>(s: S) -> Self::FfiType {
        widestring::WideCString::from_str(s).unwrap()
    }
}
