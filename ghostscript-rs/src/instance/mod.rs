use device_list::DeviceList;
use error::ErrCode;
use gs_sys;
use std::ops::Drop;
use std::sync::Arc;

#[cfg(not(feature = "synchronized"))]
pub(crate) mod lock_none;
#[cfg(not(feature = "synchronized"))]
pub(crate) use self::lock_none as lock;

#[cfg(feature = "synchronized")]
pub(crate) mod lock_mutex;
#[cfg(feature = "synchronized")]
pub(crate) use self::lock_mutex as lock;

#[derive(Debug)]
pub struct Ghostscript<T> {
    #[allow(unused)] pub(crate) lock: lock::LockType,

    pub(crate) instance: *mut gs_sys::GsRawInstance,
    pub(crate) initialized: bool,
    pub(crate) user_data: Option<T>,
    pub(crate) display_callback: Option<Arc<gs_sys::display::DisplayCallback>>,
}

impl<T> Ghostscript<T> {
    pub fn into_inner(mut self) -> T {
        self.user_data.take().expect("Bug! user_data is missing")
    }

    pub unsafe fn as_raw_instance(&mut self) -> *mut gs_sys::GsRawInstance {
        self.instance
    }

    pub fn get_default_device_list(&self) -> Result<DeviceList, ErrCode> {
        let mut ptr: *mut u8 = ::std::ptr::null_mut();
        let mut size = 0;

        unsafe {
            let err = gs_sys::ffi::gsapi_get_default_device_list(
                self.instance,
                &mut ptr as *mut *mut _ as *mut *mut ::std::os::raw::c_char,
                &mut size,
            );
            if err != gs_sys::GS_OK {
                return Err(ErrCode(err));
            }
        }

        let s = unsafe {
            let s = ::std::slice::from_raw_parts(ptr, size as _);
            ::std::str::from_utf8(s).expect("Device names aren't ASCII (or even UTF-8)")
        };

        Ok(DeviceList::new(s))
    }
}

impl<T> Drop for Ghostscript<T> {
    fn drop(&mut self) {
        if self.initialized {
            unsafe { gs_sys::ffi::gsapi_exit(self.instance) };
        }
        unsafe { gs_sys::ffi::gsapi_delete_instance(self.instance) }
    }
}
