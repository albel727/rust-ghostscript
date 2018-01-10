pub mod utf8;
//pub mod utf16;

pub trait StringEncoding {
    const GHOSTSCRIPT_ENCODING: ::gs_sys::GsArgEncoding;
    type RustType: ?Sized;
    type FfiType;

    fn from_rust_to_ffi<S: AsRef<Self::RustType>>(s: S) -> Self::FfiType;
}
