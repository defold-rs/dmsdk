//! Functions for creating and controlling engine native extension libraries.

use crate::*;
use dmsdk_ffi::dmExtension;
use libc::c_void;
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

#[doc(hidden)]
pub const DESC_BUFFER_SIZE: usize = 128;

/// Result of a callback function.
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum Result {
    Ok,
    InitError,
}

/// Event to be handled by [`Extension::on_event`].
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
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

/// Params passed to [`Extension::app_init`] and [`Extension::app_final`].
#[derive(Clone, Copy)]
pub struct AppParams {
    /// Project config file.
    pub config: dmconfigfile::ConfigFile,
    #[doc(hidden)]
    pub ptr: RawAppParams,
}

/// Params passed to [`Extension::ext_init`], [`Extension::ext_final`], [`Extension::on_update`], and [`Extension::on_event`].
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
            config: (*params).m_ConfigFile.into(),
            ptr: params,
        }
    }
}

impl Params {
    #[allow(clippy::missing_safety_doc)]
    #[doc(hidden)]
    pub unsafe fn from(params: RawParams) -> Self {
        Self {
            config: (*params).m_ConfigFile.into(),
            l: lua::State::new((*params).m_L),
            ptr: params,
        }
    }
}

pub trait Extension {
    fn app_init(&mut self, params: AppParams) -> Result {
        Result::Ok
    }
    fn app_final(&mut self, params: AppParams) -> Result {
        Result::Ok
    }
    fn ext_init(&mut self, params: Params) -> Result {
        Result::Ok
    }
    fn ext_final(&mut self, params: Params) -> Result {
        Result::Ok
    }
    fn on_update(&mut self, params: Params) -> Result {
        Result::Ok
    }
    fn on_event(&mut self, params: Params, event: Event) {}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __app_callback {
    ($ext_name:ident, $fn_name:ident) => {
		dmsdk::paste! {
			#[no_mangle]
			unsafe extern "C" fn [<$ext_name:snake:lower _ $fn_name>](params: dmsdk::dmextension::RawAppParams) -> i32 {
				[<$ext_name:snake:upper>].lock().unwrap_or_else(|err| {
					panic!(
						"failed to lock mutex for {}: {}",
						stringify!($ext_name),
						err
					)
				})
				.$fn_name(dmsdk::dmextension::AppParams::from(params)).into()
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __ext_callback {
    ($ext_name:ident, $fn_name:ident) => {
		dmsdk::paste! {
			#[no_mangle]
			unsafe extern "C" fn [<$ext_name:snake:lower _ $fn_name>](params: dmsdk::dmextension::RawParams) -> i32 {
				[<$ext_name:snake:upper>].lock().unwrap_or_else(|err| {
					panic!(
						"failed to lock mutex for {}: {}",
						stringify!($ext_name),
						err
					)
				})
				.$fn_name(dmsdk::dmextension::Params::from(params)).into()
			}
		}
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __event_callback {
    ($ext_name:ident) => {
		dmsdk::paste! {
			#[no_mangle]
			unsafe extern "C" fn [<$ext_name:snake:lower _on_event>](params: dmsdk::dmextension::RawParams, event: dmsdk::dmextension::RawEvent) {
				[<$ext_name:snake:upper>].lock().unwrap_or_else(|err| {
					panic!(
						"failed to lock mutex for {}: {}",
						stringify!($ext_name),
						err
					)
				})
				.on_event(dmsdk::dmextension::Params::from(params), dmsdk::dmextension::Event::from((*event).m_Event));
			}
		}
	};
}

/// Equivalent to `DM_DECLARE_EXTENSION` in regular C++ extensions.
///
/// # Examples
/// ```
/// use dmsdk::*;
/// use dmextension::{Params, Event, Extension}
///
/// #[derive(Default)]
/// struct MyExtension;
///
/// // See the Extension trait documentation for all available functions
/// impl Extension for MyExtension {
///     fn ext_init(&mut self, params: Params) -> dmextension::Result {
///         dmlog::info!("Registered extension MyExtension");
///
///         dmextension::Result::Ok
///     }
///
///     fn on_event(&mut self, params: Params, event: Event) {
///         dmlong::info!("Received event: {:?}", event);
///     }
/// }
///
/// declare_extension!(MyExtension);
/// ```
#[macro_export]
macro_rules! declare_extension {
    ($name:ident) => {
        dmsdk::paste! {
			const LOG_DOMAIN: &str = stringify!([<$name:upper>]);
			mod __init_extension {
				use super::$name as [<$name Struct>];
				use dmsdk::dmextension::Extension;

				static mut [<$name:snake:upper _DESC>]: dmsdk::dmextension::Desc = [0u8; dmsdk::dmextension::DESC_BUFFER_SIZE];

				dmsdk::lazy_static! {
					static ref [<$name:snake:upper>]: std::sync::Mutex<[<$name Struct>]> = std::sync::Mutex::new([<$name Struct>]::default());
				}

				dmsdk::__app_callback!($name, app_init);
				dmsdk::__app_callback!($name, app_final);
				dmsdk::__ext_callback!($name, ext_init);
				dmsdk::__ext_callback!($name, ext_final);
				dmsdk::__ext_callback!($name, on_update);
				dmsdk::__event_callback!($name);

				#[no_mangle]
				#[dmsdk::ctor]
				unsafe fn $name() {
					dmsdk::dmextension::__register(
						stringify!($name),
						&mut [<$name:snake:upper _DESC>],
						[<$name:snake:lower _app_init>],
						[<$name:snake:lower _app_final>],
						[<$name:snake:lower _ext_init>],
						[<$name:snake:lower _ext_final>],
						[<$name:snake:lower _on_update>],
						[<$name:snake:lower _on_event>],
					);
				}
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
        dmsdk_ffi::ExtensionRegister(
            desc.as_mut_ptr() as *mut c_void,
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
