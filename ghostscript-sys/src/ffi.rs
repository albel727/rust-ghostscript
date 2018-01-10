use GsArgEncoding;
use GsErrorType;
use GsPExitCode;
use GsRawInstance;

use std::os::raw::{c_char, c_int, c_uint, c_void};

pub type StdioInputCallback = unsafe extern "C" fn(caller_handle: *mut c_void, buf: *mut c_char, len: c_int) -> c_int;
pub type StdioOutputCallback = unsafe extern "C" fn(caller_handle: *mut c_void, str: *const c_char, len: c_int) -> c_int;
pub type PollCallback = unsafe extern "C" fn(caller_handle: *mut c_void) -> GsErrorType;

extern "C" {
    pub fn gsapi_revision(pr: *mut ::revision::GsApiRevision, len: c_int) -> c_int;

    pub fn gsapi_new_instance(pinstance: *mut *mut GsRawInstance, caller_handle: *mut c_void) -> GsErrorType;

    pub fn gsapi_delete_instance(instance: *mut GsRawInstance);

    pub fn gsapi_set_stdio(
        instance: *mut GsRawInstance,
        stdin_fn: Option<StdioInputCallback>,
        stdout_fn: Option<StdioOutputCallback>,
        stderr_fn: Option<StdioOutputCallback>,
    ) -> GsErrorType;

    pub fn gsapi_set_poll(instance: *mut c_void, poll_fn: Option<PollCallback>) -> GsErrorType;

    pub fn gsapi_set_display_callback(instance: *mut GsRawInstance, callback: *mut ::display::DisplayCallback) -> GsErrorType;

    pub fn gsapi_set_default_device_list(instance: *mut GsRawInstance, list: *mut c_char, listlen: c_int) -> GsErrorType;

    pub fn gsapi_get_default_device_list(instance: *mut GsRawInstance, list: *mut *mut c_char, listlen: *mut c_int) -> GsErrorType;

    pub fn gsapi_set_arg_encoding(instance: *mut GsRawInstance, encoding: GsArgEncoding) -> GsErrorType;

    pub fn gsapi_init_with_args(instance: *mut GsRawInstance, argc: c_int, argv: *mut *mut c_char) -> GsErrorType;

    pub fn gsapi_run_string_begin(instance: *mut GsRawInstance, user_errors: c_int, pexit_code: *mut GsPExitCode) -> GsErrorType;

    pub fn gsapi_run_string_continue(
        instance: *mut GsRawInstance,
        str: *const c_char,
        length: c_uint,
        user_errors: c_int,
        pexit_code: *mut GsPExitCode,
    ) -> GsErrorType;

    pub fn gsapi_run_string_end(instance: *mut GsRawInstance, user_errors: c_int, pexit_code: *mut GsPExitCode) -> GsErrorType;

    pub fn gsapi_run_string_with_length(
        instance: *mut GsRawInstance,
        str: *const c_char,
        length: c_uint,
        user_errors: c_int,
        pexit_code: *mut GsPExitCode,
    ) -> GsErrorType;

    pub fn gsapi_run_string(
        instance: *mut GsRawInstance,
        str: *const c_char,
        user_errors: c_int,
        pexit_code: *mut GsPExitCode,
    ) -> GsErrorType;

    pub fn gsapi_run_file(
        instance: *mut GsRawInstance,
        file_name: *const c_char,
        user_errors: c_int,
        pexit_code: *mut GsPExitCode,
    ) -> GsErrorType;

    pub fn gsapi_exit(instance: *mut GsRawInstance) -> GsErrorType;
}
