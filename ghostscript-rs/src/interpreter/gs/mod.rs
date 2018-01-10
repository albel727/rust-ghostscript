mod stream;

use super::*;
use DefaultEncoding as Encoding;
use gs_sys;

impl<'a, T> InterpreterStream<'a> for stream::GhostscriptStream<'a, T> {
    fn is_completed(&self) -> bool {
        self.is_completed()
    }

    fn close(self) -> InterpreterResult {
        self.close()
    }
}

impl<'a, T: 'a> Interpreter<'a> for ::instance::Ghostscript<T> {
    type Stream = stream::GhostscriptStream<'a, T>;

    fn open_interpreter_stream(&'a mut self) -> Result<Self::Stream, InterpreterResult> {
        stream::GhostscriptStream::new(self)
    }

    fn interpret_buffer(&mut self, mut buffer: &[u8]) -> InterpreterResult {
        let mut os = match self.open_interpreter_stream() {
            Ok(os) => os,
            Err(r @ InterpreterResult(_, _)) => return r,
        };

        if let Err(e) = ::std::io::copy(&mut buffer, &mut os) {
            if e.kind() != ::std::io::ErrorKind::Other {
                // Interpreter must error with ErrorKind::Other only, so this
                // isn't an interpreter error. Something inside copy() must have failed.
                // For byte buffer copying there should be no such place, but let's panic just in case.
                panic!("Bug! Unexpected kind of error on copying to interpreter stream!");
            }
            // Ignore. Interpreter errors will be visible in close() result.
        };
        os.close()
    }

    fn interpret_file(&mut self, file_name: &<Encoding as StringEncoding>::RustType) -> InterpreterResult {
        let file_name = Encoding::from_rust_to_ffi(file_name);
        let mut pexit_code: PostscriptExitCode = 0;
        let err = unsafe {
            gs_sys::ffi::gsapi_run_file(
                self.as_raw_instance() as *mut _,
                file_name.as_ptr(),
                0,
                &mut pexit_code,
            )
        };
        InterpreterResult(ErrCode(err), pexit_code)
    }
}
