use gs_sys;
use std::ffi::CString;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Utf8;

impl ::encoding::StringEncoding for Utf8 {
    const GHOSTSCRIPT_ENCODING: gs_sys::GsArgEncoding = gs_sys::encoding::UTF8;
    type RustType = str;
    type FfiType = CString;

    fn from_rust_to_ffi<S: AsRef<Self::RustType>>(s: S) -> Self::FfiType {
        CString::new(s.as_ref()).expect("Init args don't contain nul characters")
    }
}
