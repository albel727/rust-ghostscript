use super::*;
use boolinator::Boolinator;
use gs_sys::display as disp;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use std::panic::catch_unwind;

unsafe extern "C" fn display_open<T: DisplayCallback>(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int {
    catch_unwind(|| {
        debug!("display_open! Handle: {:p}, Device: {:p}", handle, device);
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_open(device)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_open", e))
        .raw_err()
}

unsafe extern "C" fn display_preclose<T: DisplayCallback>(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_preclose! Handle: {:p}, Device: {:p}",
            handle, device
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_preclose(device)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_preclose", e))
        .raw_err()
}

unsafe extern "C" fn display_close<T: DisplayCallback>(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int {
    catch_unwind(|| {
        debug!("display_close! Handle: {:p}, Device: {:p}", handle, device);
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_close(device)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_close", e))
        .raw_err()
}

unsafe extern "C" fn display_presize<T: DisplayCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    width: c_int,
    height: c_int,
    raster: c_int,
    format: c_uint,
) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_presize! Handle: {:p}, Device: {:p}, W: {}, H: {}, Raster: {}, Format: {:x}",
            handle, device, width, height, raster, format
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");

        let fmt = super::DisplayFormat::from_bits_truncate(format);
        debug_assert_eq!(
            fmt.bits(),
            format,
            "Warning, DisplayFormat contains unknown bits! Got: {:b}; Known: {:b}",
            format,
            fmt.bits()
        );

        cb.display_presize(
            device,
            width as usize,
            height as usize,
            raster as usize,
            fmt,
        )
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_presize", e))
        .raw_err()
}

unsafe extern "C" fn display_size<T: DisplayCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    width: c_int,
    height: c_int,
    raster: c_int,
    format: c_uint,
    pimage: *mut c_uchar,
) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_size! Handle: {:p}, Device: {:p}, W: {}, H: {}, Raster: {}, Format: {:x}, Pimage: {:p}",
            handle, device, width, height, raster, format, pimage
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");

        let fmt = super::DisplayFormat::from_bits_truncate(format);
        debug_assert_eq!(
            fmt.bits(),
            format,
            "Warning, DisplayFormat contains unknown bits! Got: {:b}; Known: {:b}",
            format,
            fmt.bits()
        );

        cb.display_size(
            device,
            width as usize,
            height as usize,
            raster as usize,
            fmt,
            pimage as *mut u8,
        )
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_size", e))
        .raw_err()
}

unsafe extern "C" fn display_sync<T: DisplayCallback>(handle: *mut c_void, device: *mut DisplayRawDevice) -> c_int {
    catch_unwind(|| {
        debug!("display_sync! Handle: {:p}, Device: {:p}", handle, device);
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_sync(device)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_sync", e))
        .raw_err()
}

unsafe extern "C" fn display_page<T: DisplayCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    copies: c_int,
    flush: c_int,
) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_page! Handle: {:p}, Device: {:p}, Copies: {}, Flush: {}",
            handle, device, copies, flush
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_page(device, copies as _, flush != 0)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_page", e))
        .raw_err()
}

unsafe extern "C" fn display_update<T: DisplayUpdateCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    x: c_int,
    y: c_int,
    w: c_int,
    h: c_int,
) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_update! Handle: {:p}, Device: {:p}, X: {}, Y: {}, W: {}, H: {}",
            handle, device, x, y, w, h
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_update(device, x as usize, y as usize, w as usize, h as usize)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_update", e))
        .raw_err()
}

unsafe extern "C" fn display_memalloc<T: DisplayAllocCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    size: c_ulong,
) -> *mut c_void {
    catch_unwind(|| {
        debug!(
            "display_memalloc! Handle: {:p}, Device: {:p}, Size: {}",
            handle, device, size
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_memalloc(device, size as usize)
    }).unwrap_or_else(|e| {
        T::on_callback_panic(handle as *mut T, "display_memalloc", e);
        ::std::ptr::null_mut()
    })
}

unsafe extern "C" fn display_memfree<T: DisplayAllocCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    mem: *mut c_void,
) -> c_int {
    catch_unwind(|| {
        debug!(
            "display_memfree! Handle: {:p}, Device: {:p}, Mem: {:p}",
            handle, device, mem
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_memfree(device, mem)
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_memfree", e))
        .raw_err()
}

unsafe extern "C" fn display_separation<T: DisplaySeparationCallback>(
    handle: *mut c_void,
    device: *mut DisplayRawDevice,
    component: c_int,
    component_name: *const c_char,
    c: c_ushort,
    m: c_ushort,
    y: c_ushort,
    k: c_ushort,
) -> c_int {
    use std::ffi::CStr;
    catch_unwind(|| {
        let component_name = CStr::from_ptr(component_name);
        debug!(
            "display_separation! Handle: {:p}, Device: {:p}, Comp: {}, CName {}, C: {}, M: {}, Y: {}, K: {}",
            handle,
            device,
            component,
            component_name
                .to_str()
                .unwrap_or("<ghostscript-rs component name decoding failure>"),
            c,
            m,
            y,
            k
        );
        let cb = (handle as *mut T)
            .as_mut()
            .expect("Ghostscript callback handle is not null");
        cb.display_separation(
            device,
            component as u32,
            component_name,
            (c as u16, m as u16, y as u16, k as u16),
        )
    }).unwrap_or_else(|e| T::on_callback_panic(handle as *mut T, "display_separation", e))
        .raw_err()
}

pub fn new_display_callback<T: DisplayCallback>() -> disp::DisplayCallback {
    disp::DisplayCallbackV2 {
        // Init with V1 by default
        size: ::std::mem::size_of::<disp::DisplayCallbackV1>() as _,
        version_major: disp::DISPLAY_VERSION_MAJOR_V1,
        version_minor: disp::DISPLAY_VERSION_MINOR_V1,

        display_open: Some(display_open::<T>),
        display_preclose: Some(display_preclose::<T>),
        display_close: Some(display_close::<T>),
        display_presize: Some(display_presize::<T>),
        display_size: Some(display_size::<T>),
        display_sync: Some(display_sync::<T>),
        display_page: Some(display_page::<T>),
        display_update: None,
        display_memalloc: None,
        display_memfree: None,
        display_separation: None,
    }
}

pub fn display_callback_set_update<T: DisplayUpdateCallback>(cb: &mut disp::DisplayCallback, do_it: bool) {
    cb.display_update = do_it.as_some(display_update::<T> as disp::DisplayCallbackUpdate);
}

pub fn display_callback_set_alloc<T: DisplayAllocCallback>(cb: &mut disp::DisplayCallback, do_it: bool) {
    cb.display_memalloc = do_it.as_some(display_memalloc::<T> as disp::DisplayCallbackMemAlloc);
    cb.display_memfree = do_it.as_some(display_memfree::<T> as disp::DisplayCallbackMemFree);
}

pub fn display_callback_set_separation<T: DisplaySeparationCallback>(cb: &mut disp::DisplayCallback, do_it: bool) {
    if do_it {
        // If we use separation callback, require interface version 2 with ghostscript lib.
        cb.size = ::std::mem::size_of::<disp::DisplayCallbackV2>() as _;
        cb.version_major = disp::DISPLAY_VERSION_MAJOR_V2;
        cb.version_minor = disp::DISPLAY_VERSION_MINOR_V2;
        cb.display_separation = Some(display_separation::<T> as disp::DisplayCallbackSeparation);
    } else {
        // If we don't use separation callback, v1 interface is enough.
        cb.size = ::std::mem::size_of::<disp::DisplayCallbackV1>() as _;
        cb.version_major = disp::DISPLAY_VERSION_MAJOR_V1;
        cb.version_minor = disp::DISPLAY_VERSION_MINOR_V1;
        cb.display_separation = None;
    };
}
