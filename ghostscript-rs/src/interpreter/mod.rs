mod gs;

use DefaultEncoding as Encoding;
use encoding::StringEncoding;
use error::ErrCode;
use gs_sys;

pub type PostscriptExitCode = gs_sys::GsPExitCode;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct InterpreterResult(pub ErrCode, pub PostscriptExitCode);

pub trait InterpreterStream<'a>: ::std::io::Write {
    fn is_completed(&self) -> bool;
    fn close(self) -> InterpreterResult;
}

pub trait Interpreter<'a> {
    type Stream: InterpreterStream<'a>;

    fn open_interpreter_stream(&'a mut self) -> Result<Self::Stream, InterpreterResult>;
    fn interpret_buffer(&mut self, buffer: &[u8]) -> InterpreterResult;
    fn interpret_file(&mut self, file_name: &<Encoding as StringEncoding>::RustType) -> InterpreterResult;
}
