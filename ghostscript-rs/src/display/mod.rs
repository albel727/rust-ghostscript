pub(crate) mod callbacks;

pub mod consts;
pub use self::consts::DisplayFormat;
use GS_OK;
use error::ErrCode;
use panic::PanicCallback;
use std::ffi::CStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum RawDisplayDevice {}

pub trait DisplayCallback: PanicCallback {
    fn display_open(&mut self, _device: *mut RawDisplayDevice) -> ErrCode {
        GS_OK
    }

    fn display_preclose(&mut self, _device: *mut RawDisplayDevice) -> ErrCode {
        GS_OK
    }

    fn display_close(&mut self, _device: *mut RawDisplayDevice) -> ErrCode {
        GS_OK
    }

    fn display_presize(
        &mut self,
        _device: *mut RawDisplayDevice,
        _width: usize,
        _height: usize,
        _raster: usize,
        _format: DisplayFormat,
    ) -> ErrCode {
        GS_OK
    }

    fn display_size(
        &mut self,
        _device: *mut RawDisplayDevice,
        _width: usize,
        _height: usize,
        _raster: usize,
        _format: DisplayFormat,
        _pimage: *mut u8,
    ) -> ErrCode {
        GS_OK
    }

    fn display_sync(&mut self, _device: *mut RawDisplayDevice) -> ErrCode {
        GS_OK
    }

    fn display_page(&mut self, _device: *mut RawDisplayDevice, _copies: u32, _flush: bool) -> ErrCode {
        GS_OK
    }
}

pub trait DisplayUpdateCallback: DisplayCallback {
    fn display_update(&mut self, _device: *mut RawDisplayDevice, _x: usize, _y: usize, _w: usize, _h: usize) -> ErrCode {
        GS_OK
    }
}

pub trait DisplayAllocCallback: DisplayCallback {
    unsafe fn display_memalloc(&mut self, _device: *mut RawDisplayDevice, size: usize) -> *mut ::std::os::raw::c_void {
        use std::mem::size_of;
        let size = (size + size_of::<usize>() - 1) / size_of::<usize>();
        let mut v: Vec<usize> = Vec::with_capacity(size + 1);
        let ptr = v.as_mut_ptr();
        *ptr = v.capacity();
        ::std::mem::forget(v);
        ptr.offset(1) as *mut ::std::os::raw::c_void
    }

    unsafe fn display_memfree(&mut self, _device: *mut RawDisplayDevice, mem: *mut ::std::os::raw::c_void) -> ErrCode {
        let ptr = mem as *mut usize;
        let ptr = ptr.offset(-1);
        let capacity = *ptr;
        let v = Vec::from_raw_parts(ptr, 0, capacity);
        ::std::mem::drop(v);
        GS_OK
    }
}

pub trait DisplaySeparationCallback: DisplayCallback {
    fn display_separation(
        &mut self,
        _device: *mut RawDisplayDevice,
        _component: u32,
        _component_name: &CStr,
        _cmyk: (u16, u16, u16, u16),
    ) -> ErrCode {
        GS_OK
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
fn components_for_alpha(format: DisplayFormat) -> Option<u8> {
    use self::consts::DisplayFormat as DF;
    match format & DF::MASK_ALPHA {
        DF::ALPHA_NONE => Some(0),
        DF::ALPHA_FIRST | DF::ALPHA_LAST => None, //Some(1) //Isn't actually supported.
        DF::UNUSED_FIRST | DF::UNUSED_LAST => Some(1),
        _ => None,
    }
}

pub fn components_per_pixel(format: DisplayFormat) -> Option<u8> {
    use self::consts::DisplayFormat as DF;
    let alpha = components_for_alpha(format)?;
    match format & DF::MASK_COLORS {
        DF::COLORS_NATIVE if alpha == 0 => Some(1),
        DF::COLORS_GRAY if alpha == 0 => Some(1),
        DF::COLORS_RGB => Some(3 + alpha),
        DF::COLORS_CMYK if alpha == 0 => Some(4),
        DF::COLORS_SEPARATION if alpha == 0 && depth_bits(format)? == 8 => {
            // Separation format uses fixed number of bytes per pixel
            // equal to the size of a pointer, regardless of component depth
            // (which is also presently restricted to DEPTH_8).
            // So there are as many components as bytes in a pointer.
            Some(::std::mem::size_of::<usize>() as u8)
        },
        _ => None,
    }
}

pub fn depth_bits(format: DisplayFormat) -> Option<u8> {
    use self::consts::DisplayFormat as DF;
    match format & DF::MASK_DEPTH {
        DF::DEPTH_1 => Some(1),
        DF::DEPTH_2 => Some(2),
        DF::DEPTH_4 => Some(4),
        DF::DEPTH_8 => Some(8),
        DF::DEPTH_12 => Some(12),
        DF::DEPTH_16 => Some(16),
        _ => None,
    }
}

pub fn bits_per_pixel(format: DisplayFormat) -> Option<u8> {
    let components = components_per_pixel(format)?;
    let bits = depth_bits(format)?;
    Some(components * bits)
}
