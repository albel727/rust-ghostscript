pub mod consts;

use GS_OK;
use gs_sys::GsErrorType;
use std::error::Error;
use std::fmt;

#[derive(Copy, Clone, Ord, Eq, PartialOrd, PartialEq)]
pub struct ErrCode(pub GsErrorType);

impl ErrCode {
    pub fn raw_err(&self) -> GsErrorType {
        self.0
    }
}

impl ::std::default::Default for ErrCode {
    fn default() -> Self {
        GS_OK
    }
}

impl fmt::Display for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {} ({})",
            <ErrCode as Error>::description(self),
            self.0,
        )
    }
}

impl fmt::Debug for ErrCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Error for ErrCode {
    fn description(&self) -> &str {
        error_code_to_str_unwrap(*self)
    }
}

pub(crate) fn error_code_to_str_unwrap(e: ErrCode) -> &'static str {
    error_code_to_str(e).unwrap_or("Unrecognized error code")
}

pub fn error_code_to_str(e: ErrCode) -> Option<&'static str> {
    match e {
        consts::OK => Some("OK"),
        consts::UNKNOWN_ERROR => Some("UNKNOWN_ERROR"),
        consts::DICT_FULL => Some("DICT_FULL"),
        consts::DICT_STACK_OVERFLOW => Some("DICT_STACK_OVERFLOW"),
        consts::DICT_STACK_UNDERFLOW => Some("DICT_STACK_UNDERFLOW"),
        consts::EXEC_STACK_OVERFLOW => Some("EXEC_STACK_OVERFLOW"),
        consts::INTERRUPT => Some("INTERRUPT"),
        consts::INVALID_ACCESS => Some("INVALID_ACCESS"),
        consts::INVALID_EXIT => Some("INVALID_EXIT"),
        consts::INVALID_FILE_ACCESS => Some("INVALID_FILE_ACCESS"),
        consts::INVALID_FONT => Some("INVALID_FONT"),
        consts::INVALID_RESTORE => Some("INVALID_RESTORE"),
        consts::IO_ERROR => Some("IO_ERROR"),
        consts::LIMIT_CHECK => Some("LIMIT_CHECK"),
        consts::NO_CURRENT_POINT => Some("NO_CURRENT_POINT"),
        consts::RANGE_CHECK => Some("RANGE_CHECK"),
        consts::STACK_OVERFLOW => Some("STACK_OVERFLOW"),
        consts::STACK_UNDERFLOW => Some("STACK_UNDERFLOW"),
        consts::SYNTAX_ERROR => Some("SYNTAX_ERROR"),
        consts::TIMEOUT => Some("TIMEOUT"),
        consts::TYPECHECK => Some("TYPECHECK"),
        consts::UNDEFINED => Some("UNDEFINED"),
        consts::UNDEFINED_FILENAME => Some("UNDEFINED_FILENAME"),
        consts::UNDEFINED_RESULT => Some("UNDEFINED_RESULT"),
        consts::UNMATCHED_MARK => Some("UNMATCHED_MARK"),
        consts::VM_ERROR => Some("VM_ERROR"),
        consts::CONFIGURATION_ERROR => Some("CONFIGURATION_ERROR"),
        consts::UNDEFINED_RESOURCE => Some("UNDEFINED_RESOURCE"),
        consts::UNREGISTERED => Some("UNREGISTERED"),
        consts::INVALID_CONTEXT => Some("INVALID_CONTEXT"),
        consts::INVALID_ID => Some("INVALID_ID"),
        consts::HIT_DETECTED => Some("HIT_DETECTED"),
        consts::FATAL => Some("FATAL"),
        consts::QUIT => Some("QUIT"),
        consts::INTERPRETER_EXIT => Some("INTERPRETER_EXIT"),
        consts::REMAP_COLOR => Some("REMAP_COLOR"),
        consts::EXEC_STACK_UNDERFLOW => Some("EXEC_STACK_UNDERFLOW"),
        consts::VM_RECLAIM => Some("VM_RECLAIM"),
        consts::NEED_INPUT => Some("NEED_INPUT"),
        consts::INFO => Some("INFO"),
        consts::HANDLED => Some("HANDLED"),
        _ => None,
    }
}
