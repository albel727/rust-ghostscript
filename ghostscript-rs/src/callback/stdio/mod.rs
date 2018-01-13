pub(crate) mod callbacks;

use callback::panic::PanicCallback;

pub trait StdioCallback: PanicCallback {
    fn read_stdin(&mut self, _buf: &mut [u8]) -> Option<usize> {
        Some(0) // Nothing to read. EOF.
    }

    fn write_stdout(&mut self, buf: &[u8]) -> usize {
        buf.len() // Silently discard everything.
    }

    fn write_stderr(&mut self, buf: &[u8]) -> usize {
        self.write_stdout(buf)
    }
}
