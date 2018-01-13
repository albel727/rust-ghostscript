use GS_OK;
use callback::panic::PanicCallback;
use error::ErrCode;

pub trait PollCallback: PanicCallback {
    fn poll(&mut self) -> ErrCode {
        GS_OK
    }
}

pub(crate) mod ffi_callbacks;
