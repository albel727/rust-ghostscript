use super::*;
use gs_sys;
use std::os::raw::{c_char, c_int, c_void};
use std::panic::catch_unwind;

pub type Input = gs_sys::ffi::StdioInputCallback;
pub type Output = gs_sys::ffi::StdioOutputCallback;

pub unsafe extern "C" fn stdin_callback<T: StdioCallback>(handle: *mut c_void, buf: *mut c_char, len: c_int) -> c_int {
    catch_unwind(|| {
        debug!(
            "stdin_callback! Handle: {:p}, Buffer: {:p}, Len: {}",
            handle, buf, len
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.read_stdin(::std::slice::from_raw_parts_mut(buf as *mut u8, len as _))
    }).unwrap_or_else(|e| {
        T::on_callback_panic(handle as *mut T, "stdin_callback", e);
        None
    })
        .map(|u| u as _)
        .unwrap_or(-1)
}

pub unsafe extern "C" fn stdout_callback<T: StdioCallback>(handle: *mut c_void, buf: *const c_char, len: c_int) -> c_int {
    catch_unwind(|| {
        debug!(
            "stdout_callback! Handle: {:p}, Buffer: {:p}, Len: {}",
            handle, buf, len
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.write_stdout(::std::slice::from_raw_parts(buf as *mut u8, len as _)) as c_int
    }).unwrap_or_else(|e| {
        T::on_callback_panic(handle as *mut T, "stdout_callback", e);
        0
    })
}

pub unsafe extern "C" fn stderr_callback<T: StdioCallback>(handle: *mut c_void, buf: *const c_char, len: c_int) -> c_int {
    catch_unwind(|| {
        debug!(
            "stderr_callback! Handle: {:p}, Buffer: {:p}, Len: {}",
            handle, buf, len
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.write_stderr(::std::slice::from_raw_parts(buf as *mut u8, len as _)) as c_int
    }).unwrap_or_else(|e| {
        T::on_callback_panic(handle as *mut T, "stderr_callback", e);
        0
    })
}
