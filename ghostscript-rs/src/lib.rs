/*
#![deny(missing_debug_implementations,
       missing_docs,
       trivial_casts,
       trivial_numeric_casts,
       unused_extern_crates,
       unused_import_braces,
       unused_qualifications,
       unused_results)]
*/
//#[macro_use]
//extern crate dbg;
#[macro_use]
extern crate bitflags;
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
pub mod device_list;
pub mod display;
mod encoding;
pub mod error;
pub mod instance;
pub mod interpreter;
pub mod panic;
pub mod poll;
pub mod stdio;
