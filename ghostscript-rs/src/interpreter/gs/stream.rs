use error::ErrCode;
use gs_sys;
use instance::Ghostscript;
use interpreter::InterpreterResult;
use interpreter::PostscriptExitCode;
use std::os::raw::c_char;

// This is the limit of run_string() calls.
// Data chunk delivered per call will be at most this size.
const MAX_BYTES_PER_WRITE: usize = 65_535;

#[derive(Debug)]
enum State<'a, T: 'a> {
    Running(&'a mut Ghostscript<T>),
    Completed(InterpreterResult),
    Closed,
}

#[derive(Debug)]
pub struct GhostscriptStream<'a, T: 'a>(State<'a, T>);

impl<'a, T: 'a> ::std::io::Write for GhostscriptStream<'a, T> {
    fn write(&mut self, buf: &[u8]) -> ::std::io::Result<usize> {
        let write_len = ::std::cmp::min(buf.len(), MAX_BYTES_PER_WRITE);

        let completed: InterpreterResult = match self.0 {
            State::Running(ref mut instance) => {
                let mut pexit_code: PostscriptExitCode = 0;
                let err = unsafe {
                    gs_sys::ffi::gsapi_run_string_continue(
                        instance.as_raw_instance(),
                        buf.as_ptr() as *const c_char,
                        write_len as _,
                        0,
                        &mut pexit_code,
                    )
                };

                if err == gs_sys::error::NEED_INPUT {
                    return Ok(write_len);
                }

                InterpreterResult(ErrCode(err), pexit_code)
            },
            State::Completed(interpreter_result) => {
                // Silently ignore the rest of the program or error-out, depending on ErrCode.
                interpreter_result
            },
            State::Closed => unreachable!("Bug! This variant should never be visible outside of drop/close!"),
        };

        self.0 = State::Completed(completed);
        match completed {
            InterpreterResult(::error::consts::QUIT, _) => Ok(write_len),
            InterpreterResult(other, _) => Err(::std::io::Error::new(
                ::std::io::ErrorKind::Other,
                ::error::error_code_to_str_unwrap(other),
            )),
        }
    }

    fn flush(&mut self) -> ::std::io::Result<()> {
        Ok(())
    }
}

impl<'a, T> GhostscriptStream<'a, T> {
    pub(crate) fn new(instance: &'a mut Ghostscript<T>) -> Result<Self, InterpreterResult> {
        let mut pexit_code: PostscriptExitCode = 0;
        let err = unsafe { gs_sys::ffi::gsapi_run_string_begin(instance.as_raw_instance(), 0, &mut pexit_code) };
        if err != gs_sys::GS_OK {
            return Err(InterpreterResult(ErrCode(err), pexit_code));
        }
        Ok(GhostscriptStream(State::Running(instance)))
    }

    pub fn is_completed(&self) -> bool {
        match self.0 {
            State::Running(_) => false,
            State::Completed(_) => true,
            State::Closed => unreachable!("Bug! This variant should never be visible outside of drop/close!"),
        }
    }

    pub fn close(mut self) -> InterpreterResult {
        match ::std::mem::replace(&mut self.0, State::Closed) {
            State::Running(ref mut instance) => {
                let mut pexit_code: PostscriptExitCode = 0;
                let err = unsafe { gs_sys::ffi::gsapi_run_string_end(instance.as_raw_instance(), 0, &mut pexit_code) };

                InterpreterResult(ErrCode(err), pexit_code)
            },
            State::Completed(interpreter_result) => interpreter_result,
            State::Closed => unreachable!("Bug! This variant should never be visible outside of drop/close!"),
        }
    }
}

impl<'a, T: 'a> Drop for GhostscriptStream<'a, T> {
    fn drop(&mut self) {
        if let State::Closed = self.0 {
            return;
        };

        let e = Self::close(::std::mem::replace(self, GhostscriptStream(State::Closed)));

        debug!(
            "Dropped unclosed GhostscriptStream! Use explicit close() to collect errors. Close result: ({:?})",
            e
        );
    }
}
