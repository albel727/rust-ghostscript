use error::ErrCode;

pub trait PanicCallback {
    fn on_callback_panic(callback_ptr: *mut Self, callback_name: &'static str, _error: Box<::std::any::Any + Send + 'static>) -> ErrCode {
        error!(
            "Panic in ghostscript {} callback ({:p}), aborting!",
            callback_name, callback_ptr
        );
        ::std::process::abort()
    }
}
