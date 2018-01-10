use GS_OK;
use error::ErrCode;
use panic::PanicCallback;

pub trait PollCallback: PanicCallback {
    fn poll(&mut self) -> ErrCode {
        GS_OK
    }
}

pub(crate) mod callbacks;
