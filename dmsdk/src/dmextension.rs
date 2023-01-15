//! Functions for creating and controlling engine native extension libraries.

use crate::*;
use dmsdk_ffi::dmExtension;
use std::ffi::CString;

#[doc(hidden)]
pub use ctor::ctor;

#[doc(hidden)]
pub type RawParams = *mut dmExtension::Params;
#[doc(hidden)]
pub type RawEvent = *const dmExtension::Event;
#[doc(hidden)]
pub type RawAppParams = *mut dmExtension::AppParams;
type RawAppCallback = unsafe extern "C" fn(RawAppParams) -> i32;
type RawCallback = unsafe extern "C" fn(RawParams) -> i32;
type RawEventCallback = unsafe extern "C" fn(RawParams, RawEvent);
#[doc(hidden)]
pub type Desc = [u8; DESC_BUFFER_SIZE];
/// Callback function called during the application lifecycle.
pub type AppCallback = fn(AppParams) -> Result;
/// Callback function called during the extension lifecycle.
pub type Callback = fn(Params) -> Result;
/// Callback for handling various events.
pub type EventCallback = fn(Params, Event);

#[doc(hidden)]
pub const DESC_BUFFER_SIZE: usize = 128;

/// Result of a callback function.
#[allow(missing_docs)]
pub enum Result {
    Ok,
    InitError,
}

/// Possible events that an [`EventCallback`] can respond to.
#[allow(missing_docs)]
pub enum Event {
    ActivateApp,
    DeactivateApp,
    IconifyApp,
    DeiconifyApp,
    Unknown,
}

impl From<Result> for i32 {
    fn from(result: Result) -> Self {
        match result {
            Result::Ok => 0,
            Result::InitError => -1,
        }
    }
}

impl From<u32> for Event {
    fn from(id: u32) -> Self {
        match id {
            0 => Self::ActivateApp,
            1 => Self::DeactivateApp,
            2 => Self::IconifyApp,
            3 => Self::DeiconifyApp,
            _ => Self::Unknown,
        }
    }
}

impl From<i32> for Event {
    fn from(id: i32) -> Self {
        (id as u32).into()
    }
}

/// Params passed to an [`AppCallback`].
#[derive(Clone, Copy)]
pub struct AppParams {
    /// Project config file.
    pub config: dmconfigfile::ConfigFile,
    #[doc(hidden)]
    pub ptr: RawAppParams,
}

/// Params passed to a [`Callback`] or [`EventCallback`].
#[derive(Clone, Copy)]
pub struct Params {
    /// Project config file.
    pub config: dmconfigfile::ConfigFile,
    /// Lua state.
    pub l: lua::State,
    #[doc(hidden)]
    pub ptr: RawParams,
}

impl AppParams {
    #[allow(clippy::missing_safety_doc)]
    #[doc(hidden)]
    pub unsafe fn from(params: RawAppParams) -> Self {
        Self {
            config: (*params).m_ConfigFile,
            ptr: params,
        }
    }
}

impl Params {
    #[allow(clippy::missing_safety_doc)]
    #[doc(hidden)]
    pub unsafe fn from(params: RawParams) -> Self {
        Self {
            config: (*params).m_ConfigFile,
            l: lua::State::new((*params).m_L),
            ptr: params,
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_app_callback {
    ($symbol:ident, $option:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $symbol(params: dmextension::RawAppParams) -> i32 {
            let func: Option<dmextension::AppCallback> = $option;
            match func {
                Some(func) => func(dmextension::AppParams::from(params)).into(),
                None => 0,
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_callback {
    ($symbol:ident, $option:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $symbol(params: dmextension::RawParams) -> i32 {
            let func: Option<dmextension::Callback> = $option;
            match func {
                Some(func) => func(dmextension::Params::from(params)).into(),
                None => 0,
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __declare_event_callback {
    ($symbol:ident, $option:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $symbol(params: dmextension::RawParams, event: dmextension::RawEvent) {
            let func: Option<dmextension::EventCallback> = $option;
            match func {
                Some(func) => func(dmextension::Params::from(params), (*event).m_Event.into()),
                None => {}
            }
        }
    };
}

/// Equivalent to `DM_DECLARE_EXTENSION` in regular C++ extensions.
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn app_init(params: dmextension::AppParams) -> dmextension::Result { dmextension::Result::Ok }
/// fn app_final(params: dmextension::AppParams) -> dmextension::Result { dmextension::Result::Ok }
/// fn ext_init(params: dmextension::Params) -> dmextension::Result {
///     dmlog::info!("Registered extension MY_EXTENSION");
///     
///     dmextension::Result::Ok
/// }
/// fn ext_final(params: dmextension::Params) -> dmextension::Result { dmextension::Result::Ok }
/// fn on_update(params: dmextension::Params) -> dmextension::Result { dmextension::Result::Ok }
/// fn on_event(params: dmextension::Params, event: dmextension::Event) { }
///
/// declare_extension!(MY_EXTENSION, Some(app_init), Some(app_final), Some(ext_init), Some(ext_final), Some(on_update), Some(on_event));
/// ```
#[macro_export]
macro_rules! declare_extension {
    ($symbol:ident, $app_init:expr, $app_final:expr, $ext_init:expr, $ext_final:expr, $on_update:expr, $on_event:expr) => {
        paste! {
            static mut [<$symbol _DESC>]: dmextension::Desc = [0u8; dmextension::DESC_BUFFER_SIZE];

            const LOG_DOMAIN: &str = stringify!($symbol);

            __declare_app_callback!([<$symbol _app_init>], $app_init);
            __declare_app_callback!([<$symbol _app_final>], $app_final);
            __declare_callback!([<$symbol _ext_init>], $ext_init);
            __declare_callback!([<$symbol _ext_final>], $ext_final);
            __declare_callback!([<$symbol _on_update>], $on_update);
            __declare_event_callback!([<$symbol _on_event>], $on_event);

            #[no_mangle]
            #[dmextension::ctor]
            unsafe fn $symbol() {
                dmextension::__register(
                    stringify!($symbol),
                    &mut [<$symbol _DESC>],
                    [<$symbol _app_init>],
                    [<$symbol _app_final>],
                    [<$symbol _ext_init>],
                    [<$symbol _ext_final>],
                    [<$symbol _on_update>],
                    [<$symbol _on_event>],
                );
            }
        }
    };
}

#[allow(clippy::too_many_arguments)]
#[doc(hidden)]
pub fn __register(
    name: &str,
    desc: &mut Desc,
    app_init: RawAppCallback,
    app_final: RawAppCallback,
    ext_init: RawCallback,
    ext_final: RawCallback,
    update: RawCallback,
    on_event: RawEventCallback,
) {
    let name = CString::new(name).unwrap();

    unsafe {
        dmExtension::Register(
            desc.as_mut_ptr() as *mut dmExtension::Desc,
            11,
            name.as_ptr(),
            Some(app_init),
            Some(app_final),
            Some(ext_init),
            Some(ext_final),
            Some(update),
            Some(on_event),
        );
    }
}

#[doc(inline)]
pub use crate::declare_extension;
