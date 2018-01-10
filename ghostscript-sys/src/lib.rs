#![deny(improper_ctypes)]

pub mod ffi;
pub mod error;
pub mod display;
pub mod revision;

use std::os::raw::{c_int, c_uint};

#[derive(Debug)]
pub enum GsRawInstance {}

pub type GsErrorType = c_int;
pub const GS_OK: GsErrorType = error::OK;

pub type GsPExitCode = c_int;

pub type GsArgEncoding = c_uint;
pub mod encoding {
    use GsArgEncoding;

    pub const LOCAL: GsArgEncoding = 0;
    pub const UTF8: GsArgEncoding = 1;
    pub const UTF16LE: GsArgEncoding = 2;
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_create_interpreter_instance() {
        let mut instance = ::std::ptr::null_mut();
        unsafe {
            match ::ffi::gsapi_new_instance(&mut instance, ::std::ptr::null_mut()) {
                ::GS_OK => {},
                err => {
                    panic!("Error creating gs instance: {}", err);
                },
            }

            ::ffi::gsapi_delete_instance(instance);
        }
    }
}
