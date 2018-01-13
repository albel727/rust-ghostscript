use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};

pub const DISPLAY_VERSION_MAJOR: c_int = DISPLAY_VERSION_MAJOR_V2;
pub const DISPLAY_VERSION_MINOR: c_int = DISPLAY_VERSION_MINOR_V2;

pub const DISPLAY_VERSION_MAJOR_V2: c_int = 2;
pub const DISPLAY_VERSION_MINOR_V2: c_int = 0;
pub const DISPLAY_VERSION_MAJOR_V1: c_int = 1;
pub const DISPLAY_VERSION_MINOR_V1: c_int = 0;

pub type DisplayFormat = c_uint;

pub const DISPLAY_COLORS_MASK: DisplayFormat = 0x0008_000f;
pub const DISPLAY_ALPHA_MASK: DisplayFormat = 0x0000_00f0;
pub const DISPLAY_DEPTH_MASK: DisplayFormat = 0x0000_ff00;
pub const DISPLAY_ENDIAN_MASK: DisplayFormat = 0x0001_0000;
pub const DISPLAY_FIRSTROW_MASK: DisplayFormat = 0x0002_0000;
pub const DISPLAY_555_MASK: DisplayFormat = 0x0004_0000;
pub const DISPLAY_ROW_ALIGN_MASK: DisplayFormat = 0x0070_0000;

#[cfg_attr(feature = "cargo-clippy", allow(identity_op))]
pub const DISPLAY_COLORS_NATIVE: DisplayFormat = 1 << 0;
pub const DISPLAY_COLORS_GRAY: DisplayFormat = 1 << 1;
pub const DISPLAY_COLORS_RGB: DisplayFormat = 1 << 2;
pub const DISPLAY_COLORS_CMYK: DisplayFormat = 1 << 3;
pub const DISPLAY_COLORS_SEPARATION: DisplayFormat = 1 << 19;

pub const DISPLAY_ALPHA_NONE: DisplayFormat = 0 << 4;
pub const DISPLAY_ALPHA_FIRST: DisplayFormat = 1 << 4;
pub const DISPLAY_ALPHA_LAST: DisplayFormat = 1 << 5;
pub const DISPLAY_UNUSED_FIRST: DisplayFormat = 1 << 6;
pub const DISPLAY_UNUSED_LAST: DisplayFormat = 1 << 7;

pub const DISPLAY_DEPTH_1: DisplayFormat = 1 << 8;
pub const DISPLAY_DEPTH_2: DisplayFormat = 1 << 9;
pub const DISPLAY_DEPTH_4: DisplayFormat = 1 << 10;
pub const DISPLAY_DEPTH_8: DisplayFormat = 1 << 11;
pub const DISPLAY_DEPTH_12: DisplayFormat = 1 << 12;
pub const DISPLAY_DEPTH_16: DisplayFormat = 1 << 13;

pub const DISPLAY_BIGENDIAN: DisplayFormat = 0 << 16;
pub const DISPLAY_LITTLEENDIAN: DisplayFormat = 1 << 16;

pub const DISPLAY_TOPFIRST: DisplayFormat = 0 << 17;
pub const DISPLAY_BOTTOMFIRST: DisplayFormat = 1 << 17;

pub const DISPLAY_NATIVE_555: DisplayFormat = 0 << 18;
pub const DISPLAY_NATIVE_565: DisplayFormat = 1 << 18;

pub const DISPLAY_ROW_ALIGN_DEFAULT: DisplayFormat = 0 << 20;
pub const DISPLAY_ROW_ALIGN_4: DisplayFormat = 3 << 20;
pub const DISPLAY_ROW_ALIGN_8: DisplayFormat = 4 << 20;
pub const DISPLAY_ROW_ALIGN_16: DisplayFormat = 5 << 20;
pub const DISPLAY_ROW_ALIGN_32: DisplayFormat = 6 << 20;
pub const DISPLAY_ROW_ALIGN_64: DisplayFormat = 7 << 20;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DisplayRawDevice {}

pub type DisplayCallbackOpen = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int;
pub type DisplayCallbackPreClose = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int;
pub type DisplayCallbackClose = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int;

pub type DisplayCallbackPreSize =
    unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice, width: c_int, height: c_int, raster: c_int, format: c_uint) -> c_int;
pub type DisplayCallbackSize = unsafe extern "C" fn(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    width: c_int,
    height: c_int,
    raster: c_int,
    format: c_uint,
    pimage: *mut c_uchar,
) -> c_int;

pub type DisplayCallbackSync = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int;
pub type DisplayCallbackPage = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice, copies: c_int, flush: c_int) -> c_int;

pub type DisplayCallbackUpdate =
    unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice, x: c_int, y: c_int, w: c_int, h: c_int) -> c_int;

pub type DisplayCallbackMemAlloc = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice, size: c_ulong) -> *mut c_void;
pub type DisplayCallbackMemFree = unsafe extern "C" fn(handle: *mut c_void, device: *mut DisplayRawDevice, mem: *mut c_void) -> c_int;

pub type DisplayCallbackSeparation = unsafe extern "C" fn(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    component: c_int,
    component_name: *const c_char,
    c: c_ushort,
    m: c_ushort,
    y: c_ushort,
    k: c_ushort,
) -> c_int;

pub type DisplayCallback = DisplayCallbackV2;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DisplayCallbackV2 {
    pub size: c_int,
    pub version_major: c_int,
    pub version_minor: c_int,
    pub display_open: Option<DisplayCallbackOpen>,
    pub display_preclose: Option<DisplayCallbackPreClose>,
    pub display_close: Option<DisplayCallbackClose>,
    pub display_presize: Option<DisplayCallbackPreSize>,
    pub display_size: Option<DisplayCallbackSize>,
    pub display_sync: Option<DisplayCallbackSync>,
    pub display_page: Option<DisplayCallbackPage>,
    pub display_update: Option<DisplayCallbackUpdate>,
    pub display_memalloc: Option<DisplayCallbackMemAlloc>,
    pub display_memfree: Option<DisplayCallbackMemFree>,
    pub display_separation: Option<DisplayCallbackSeparation>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DisplayCallbackV1 {
    pub size: c_int,
    pub version_major: c_int,
    pub version_minor: c_int,
    pub display_open: Option<DisplayCallbackOpen>,
    pub display_preclose: Option<DisplayCallbackPreClose>,
    pub display_close: Option<DisplayCallbackClose>,
    pub display_presize: Option<DisplayCallbackPreSize>,
    pub display_size: Option<DisplayCallbackSize>,
    pub display_sync: Option<DisplayCallbackSync>,
    pub display_page: Option<DisplayCallbackPage>,
    pub display_update: Option<DisplayCallbackUpdate>,
    pub display_memalloc: Option<DisplayCallbackMemAlloc>,
    pub display_memfree: Option<DisplayCallbackMemFree>,
}
