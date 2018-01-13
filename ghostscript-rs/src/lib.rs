#![deny(private_in_public)]
#![deny(missing_debug_implementations)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
//#![deny(missing_docs)]
//#![deny(trivial_casts)]
//#![deny(trivial_numeric_casts)]
//#![deny(unused_results)]

//#[macro_use]
//extern crate dbg;
#[macro_use]
extern crate bitflags;
extern crate boolinator;
#[cfg(feature = "synchronized")]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

extern crate ghostscript_sys;
use ghostscript_sys as gs_sys;

type DefaultEncoding = ::encoding::utf8::Utf8;

pub const GS_OK: error::ErrCode = error::consts::OK;

pub mod builder;
pub mod callback;
pub mod device_list;
mod encoding;
pub mod error;
pub mod instance;
pub mod interpreter;
