//! Functions for creating and controlling engine native extension libraries.

use crate::*;
use dmsdk_ffi::dmExtension;
use std::ffi::CString;

pub use ctor::ctor;

pub type RawParams = *mut dmExtension::Params;
pub type RawEvent = *const dmExtension::Event;
pub type RawAppParams = *mut dmExtension::AppParams;
type RawAppCallback = unsafe extern "C" fn(RawAppParams) -> i32;
type RawCallback = unsafe extern "C" fn(RawParams) -> i32;
type RawEventCallback = unsafe extern "C" fn(RawParams, RawEvent);
pub type Desc = [u8; DESC_BUFFER_SIZE];
pub type AppCallback = fn(AppParams) -> Result;
pub type Callback = fn(Params) -> Result;
pub type EventCallback = fn(Params, Event);

pub const DESC_BUFFER_SIZE: usize = 128;

pub enum Result {
    Ok,
    InitError,
}

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

impl Event {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from(event: *const dmExtension::Event) -> Self {
        let id = (*event).m_Event;
        match id {
            0 => Self::ActivateApp,
            1 => Self::DeactivateApp,
            2 => Self::IconifyApp,
            3 => Self::DeiconifyApp,
            _ => Self::Unknown,
        }
    }
}

pub struct AppParams {
    pub config_file: dmconfigfile::ConfigFile,
    pub ptr: RawAppParams,
}

pub struct Params {
    pub config_file: dmconfigfile::ConfigFile,
    pub l: lua::State,
    pub ptr: RawParams,
}

impl AppParams {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from(params: RawAppParams) -> Self {
        Self {
            config_file: (*params).m_ConfigFile,
            ptr: params,
        }
    }
}

impl Params {
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from(params: RawParams) -> Self {
        Self {
            config_file: (*params).m_ConfigFile,
            l: (*params).m_L,
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
                Some(func) => func(
                    dmextension::Params::from(params),
                    dmextension::Event::from(event),
                ),
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
