use super::*;
use callback::get_cb;
use gs_sys;
use std::os::raw::c_void;
use std::panic::catch_unwind;

pub type Poll = gs_sys::ffi::PollCallback;

pub unsafe extern "C" fn poll_callback<T: PollCallback>(handle: *mut c_void) -> gs_sys::GsErrorType {
    catch_unwind(|| {
        trace!("poll_callback! Handle: {:p}", handle);
        get_cb::<T>(handle).poll()
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "poll_callback", e))
        .raw_err()
}
