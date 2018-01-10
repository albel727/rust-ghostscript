use std::os::raw::{c_char, c_long};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GsApiRevision {
    pub product: *const c_char,
    pub copyright: *const c_char,
    pub revision: c_long,
    pub revisiondate: c_long,
}
