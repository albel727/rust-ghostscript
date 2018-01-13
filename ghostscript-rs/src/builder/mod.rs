use DefaultEncoding as Encoding;
use device_list;
use display;
use encoding::StringEncoding;
use error::ErrCode;
use gs_sys;
use instance;
use poll;
use std::os::raw::{c_char, c_void};
use stdio;

use std::sync::Arc;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BuilderErrorKind {
    Creation,
    ArgumentEncoding,
    DefaultDeviceList,
    DisplayCallback,
    PollCallback,
    StdioCallback,
    Initialization,
}

#[derive(Debug)]
pub struct BuilderError<T> {
    pub kind: BuilderErrorKind,
    pub code: ErrCode,
    pub user_data: T,
}

impl<T> BuilderError<T> {
    pub fn new(kind: BuilderErrorKind, code: ErrCode, user_data: T) -> Self {
        BuilderError {
            kind,
            code,
            user_data,
        }
    }

    pub fn kind_and_code(&self) -> (BuilderErrorKind, ErrCode) {
        (self.kind, self.code)
    }
}

#[derive(Debug)]
pub enum BuilderResult<T> {
    Running(::instance::Ghostscript<T>),
    Quit(T),
    Failed(BuilderError<T>),
}

impl<T> BuilderResult<T> {
    pub fn running(self) -> Result<::instance::Ghostscript<T>, BuilderError<T>> {
        match self {
            BuilderResult::Running(instance) => Ok(instance),
            BuilderResult::Quit(user_data) => Err(BuilderError::new(
                BuilderErrorKind::Initialization,
                ::error::consts::QUIT,
                user_data,
            )),
            BuilderResult::Failed(be) => Err(be),
        }
    }

    pub fn has_quit(self) -> Result<T, Result<::instance::Ghostscript<T>, BuilderError<T>>> {
        match self {
            BuilderResult::Quit(user_data) => Ok(user_data),
            BuilderResult::Running(instance) => Err(Ok(instance)),
            BuilderResult::Failed(be) => Err(Err(be)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GhostscriptBuilder<T> {
    default_device_list: Option<device_list::DeviceList>,
    display_callback: Option<Arc<gs_sys::display::DisplayCallback>>,
    poll_callback: Option<::poll::callbacks::Poll>,
    stdin_callback: Option<::stdio::callbacks::Input>,
    stdout_callback: Option<::stdio::callbacks::Output>,
    stderr_callback: Option<::stdio::callbacks::Output>,
    init_params: Vec<<Encoding as StringEncoding>::FfiType>,
    _pd: ::std::marker::PhantomData<T>,
}

impl<T> ::std::default::Default for GhostscriptBuilder<T> {
    fn default() -> Self {
        GhostscriptBuilder {
            default_device_list: None,
            display_callback: None,
            poll_callback: None,
            stdin_callback: None,
            stdout_callback: None,
            stderr_callback: None,
            init_params: Vec::new(),
            _pd: ::std::marker::PhantomData::<T>,
        }
    }
}

impl<T> GhostscriptBuilder<T> {
    pub fn new() -> Self {
        GhostscriptBuilder::default()
    }

    pub fn with_display(&mut self, do_it: bool) -> &mut Self
    where
        T: display::DisplayCallback,
    {
        if !do_it {
            self.display_callback = None;
        } else if self.display_callback.is_none() {
            self.display_callback = Some(Arc::new(display::callbacks::new_display_callback::<T>()))
        }
        self
    }

    pub fn with_display_update(&mut self, do_it: bool) -> &mut Self
    where
        T: display::DisplayUpdateCallback,
    {
        if do_it {
            self.with_display(true);
        }
        self.display_callback
            .as_mut()
            .map(|dc| display::callbacks::display_callback_set_update::<T>(Arc::make_mut(dc), do_it));
        self
    }

    pub fn with_display_alloc(&mut self, do_it: bool) -> &mut Self
    where
        T: display::DisplayAllocCallback,
    {
        if do_it {
            self.with_display(true);
        }
        self.display_callback
            .as_mut()
            .map(|dc| display::callbacks::display_callback_set_alloc::<T>(Arc::make_mut(dc), do_it));
        self
    }

    pub fn with_display_separation(&mut self, do_it: bool) -> &mut Self
    where
        T: display::DisplaySeparationCallback,
    {
        if do_it {
            self.with_display(true);
        }
        self.display_callback
            .as_mut()
            .map(|dc| display::callbacks::display_callback_set_separation::<T>(Arc::make_mut(dc), do_it));
        self
    }

    pub fn with_default_device_list<S: Into<device_list::DeviceList>>(&mut self, device_list: Option<S>) -> &mut Self {
        self.default_device_list = device_list.map(Into::into);
        self
    }

    pub fn with_poll(&mut self, do_it: bool) -> &mut Self
    where
        T: poll::PollCallback,
    {
        if do_it {
            self.poll_callback = Some(::poll::callbacks::poll_callback::<T>);
        } else {
            self.poll_callback = None;
        }
        self
    }

    pub fn with_stdin(&mut self, do_it: bool) -> &mut Self
    where
        T: stdio::StdioCallback,
    {
        if do_it {
            self.stdin_callback = Some(::stdio::callbacks::stdin_callback::<T>);
        } else {
            self.stdin_callback = None;
        }
        self
    }

    pub fn with_stdout(&mut self, do_it: bool) -> &mut Self
    where
        T: stdio::StdioCallback,
    {
        if do_it {
            self.stdout_callback = Some(::stdio::callbacks::stdout_callback::<T>);
        } else {
            self.stdout_callback = None;
        }
        self
    }

    pub fn with_stderr(&mut self, do_it: bool) -> &mut Self
    where
        T: stdio::StdioCallback,
    {
        if do_it {
            self.stderr_callback = Some(::stdio::callbacks::stderr_callback::<T>);
        } else {
            self.stderr_callback = None;
        }
        self
    }

    pub fn with_init_params<Q: AsRef<str>, I: IntoIterator<Item = Q>>(&mut self, params: I) -> &mut Self {
        self.init_params = params
            .into_iter()
            .map(|s| Encoding::from_rust_to_ffi(s.as_ref()))
            .collect();
        self
    }

    #[cfg_attr(feature = "cargo-clippy", allow(let_unit_value))]
    pub fn build<Q: ::callback::CallbackSafe<Target = T>>(&self, mut user_data: Q) -> BuilderResult<Q> {
        let lock = ::instance::lock::get_lock();

        let mut instance = ::std::ptr::null_mut();

        unsafe {
            let err = {
                let data_ptr: *mut T = user_data.as_stable_mut();
                gs_sys::ffi::gsapi_new_instance(&mut instance, data_ptr as *mut c_void)
            };
            if err != gs_sys::GS_OK {
                return BuilderResult::Failed(BuilderError::new(
                    BuilderErrorKind::Creation,
                    ErrCode(err),
                    user_data,
                ));
            }
        }

        // Wrap the instance early, so that it will be
        // properly destroyed by drop() in case of error.
        let mut instance = instance::Ghostscript {
            lock,
            instance,
            initialized: false,
            user_data: Some(user_data),
            display_callback: None,
        };

        unsafe {
            let err = gs_sys::ffi::gsapi_set_arg_encoding(instance.instance, Encoding::GHOSTSCRIPT_ENCODING);
            if err != gs_sys::GS_OK {
                return BuilderResult::Failed(BuilderError::new(
                    BuilderErrorKind::ArgumentEncoding,
                    ErrCode(err),
                    instance.into_inner(),
                ));
            }
        }

        if let Some(default_device_list) = self.default_device_list.as_ref() {
            let default_device_list = default_device_list.as_str();
            unsafe {
                let err = gs_sys::ffi::gsapi_set_default_device_list(
                    instance.instance,
                    default_device_list.as_ptr() as _,
                    default_device_list.len() as _,
                );
                if err != gs_sys::GS_OK {
                    return BuilderResult::Failed(BuilderError::new(
                        BuilderErrorKind::DefaultDeviceList,
                        ErrCode(err),
                        instance.into_inner(),
                    ));
                }
            }
        }

        if let Some(display_callback) = self.display_callback.clone() {
            unsafe {
                let err = gs_sys::ffi::gsapi_set_display_callback(
                    instance.instance,
                    display_callback.as_ref() as *const gs_sys::display::DisplayCallback as *mut _,
                );
                if err != gs_sys::GS_OK {
                    return BuilderResult::Failed(BuilderError::new(
                        BuilderErrorKind::DisplayCallback,
                        ErrCode(err),
                        instance.into_inner(),
                    ));
                }
                instance.display_callback = Some(display_callback);
            }
        }

        if self.poll_callback.is_some() {
            unsafe {
                let err = gs_sys::ffi::gsapi_set_poll(instance.instance as *mut c_void, self.poll_callback);
                if err != gs_sys::GS_OK {
                    return BuilderResult::Failed(BuilderError::new(
                        BuilderErrorKind::PollCallback,
                        ErrCode(err),
                        instance.into_inner(),
                    ));
                }
            }
        }

        if self.stdin_callback.is_some() || self.stdout_callback.is_some() || self.stderr_callback.is_some() {
            unsafe {
                let err = gs_sys::ffi::gsapi_set_stdio(
                    instance.instance,
                    self.stdin_callback,
                    self.stdout_callback,
                    self.stderr_callback,
                );
                if err != gs_sys::GS_OK {
                    return BuilderResult::Failed(BuilderError::new(
                        BuilderErrorKind::StdioCallback,
                        ErrCode(err),
                        instance.into_inner(),
                    ));
                }
            }
        }

        let mut init_ptrs: Vec<*const c_char> = Vec::new();

        // First parameter is always ignored, fill it with an empty one.
        let empty_arg = Encoding::from_rust_to_ffi("");
        init_ptrs.push(empty_arg.as_ptr());

        // Make second parameter be display handle, if we need one.
        // Making it last doesn't work, if args contain file names.
        let display_handle_arg = {
            self.display_callback.as_ref().map(|_| {
                Encoding::from_rust_to_ffi(&Self::format_display_handle_string(
                    instance
                        .user_data
                        .as_mut()
                        .expect("user_data isn't None")
                        .as_stable_mut(),
                ))
            })
        };

        if let Some(display_handle_arg) = display_handle_arg.as_ref() {
            // Set our display handle for display callbacks to use.
            init_ptrs.push(display_handle_arg.as_ptr());
        }

        // Fill the rest of user arguments.
        init_ptrs.extend(self.init_params.iter().map(|s| s.as_ptr()));

        unsafe {
            let err = gs_sys::ffi::gsapi_init_with_args(
                instance.instance,
                init_ptrs.len() as _,
                init_ptrs.as_ptr() as *mut *mut _,
            );
            match err {
                gs_sys::GS_OK => {
                    // Success. gsapi_exit() shall be called on instance on drop.
                    instance.initialized = true;
                },
                gs_sys::error::QUIT => {
                    // Regular quit during init argument processing.
                    // The instance can't be used further,
                    // and has to be de-initialized with gsapi_exit().
                    instance.initialized = true;
                    return BuilderResult::Quit(instance.into_inner());
                },
                _ => {
                    return BuilderResult::Failed(BuilderError::new(
                        BuilderErrorKind::Initialization,
                        ErrCode(err),
                        instance.into_inner(),
                    ))
                },
            }
        }

        BuilderResult::Running(instance)
    }

    fn format_display_handle_string(handle: *const T) -> String {
        format!("-sDisplayHandle=16#{:x}", handle as u64)
    }
}

impl GhostscriptBuilder<()> {
    pub fn build_simple(&self) -> BuilderResult<::callback::NoCallback> {
        self.build(::callback::NoCallback)
    }
}
