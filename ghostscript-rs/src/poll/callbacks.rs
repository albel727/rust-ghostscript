use super::*;
use gs_sys;
use std::os::raw::c_void;
use std::panic::catch_unwind;

pub type Poll = gs_sys::ffi::PollCallback;

pub unsafe extern "C" fn poll_callback<T: PollCallback>(handle: *mut c_void) -> gs_sys::GsErrorType {
    catch_unwind(|| {
        trace!("poll_callback! Handle: {:p}", handle);
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.poll()
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "poll_callback", e))
        .raw_err()
}
