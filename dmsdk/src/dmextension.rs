//! Functions for creating and controlling engine native extension libraries.

use dmsdk_ffi::dmExtension;
use std::ffi::CString;

pub use ctor::ctor;

pub type AppParams = *mut dmExtension::AppParams;
pub type Params = *mut dmExtension::Params;
pub type Event = *const dmExtension::Event;
pub type AppCallback = unsafe extern "C" fn(params: AppParams) -> i32;
pub type Callback = unsafe extern "C" fn(params: Params) -> i32;
pub type EventCallback = unsafe extern "C" fn(params: Params, event: Event);
pub type Desc = dmExtension::Desc;

pub const RESULT_OK: i32 = 0;
pub const RESULT_INIT_ERROR: i32 = -1;

/// Equivalent to `DM_DECLARE_EXTENSION` in regular C++ extensions.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate dmsdk;
/// use dmsdk::{
///     dmlog,
///     dmextension::{self, AppParams, Params, Event},
/// };
///
/// #[no_mangle]
/// pub unsafe extern "C" fn ext_init(params: Params) -> i32 {
///     dmlog::info("MY_EXTENSION", "Registered extension MY_EXTENSION");
///     
///     dmextension::RESULT_OK
/// }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn ext_final(params: Params) -> i32 { dmextension::RESULT_OK }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn app_init(params: AppParams) -> i32 { dmextension::RESULT_OK }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn app_final(params: AppParams) -> i32 { dmextension::RESULT_OK }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn update(params: Params) -> i32 { dmextension::RESULT_OK }
///
/// #[no_mangle]
/// pub unsafe extern "C" fn on_event(params: Params, event: Event) { }
///
/// declare_extension!(MY_EXTENSION, Some(app_init), Some(app_final), Some(ext_init), Some(ext_final), Some(update), Some(on_event));
///
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! declare_extension {
    ($symbol:ident, $app_init:expr, $app_final:expr, $ext_init:expr, $ext_final:expr, $on_update:expr, $on_event:expr) => {
        paste! {
            static mut [<$symbol _DESC>]: dmextension::Desc = dmextension::Desc {
                _bindgen_opaque_blob: [0u64; 11],
            };

            #[no_mangle]
            #[dmextension::ctor]
            fn $symbol() {
                dmextension::_register(
                    stringify!($symbol),
                    &mut [<$symbol _DESC>],
                    $app_init,
                    $app_final,
                    $ext_init,
                    $ext_final,
                    $update,
                    $on_event,
                );
            }
        }
    };
}

#[allow(clippy::too_many_arguments)]
pub fn _register(
    name: &str,
    desc: &mut Desc,
    app_init: Option<AppCallback>,
    app_final: Option<AppCallback>,
    ext_init: Option<Callback>,
    ext_final: Option<Callback>,
    update: Option<Callback>,
    on_event: Option<EventCallback>,
) {
    let name = CString::new(name).unwrap();

    unsafe {
        dmExtension::Register(
            desc,
            11,
            name.as_ptr(),
            app_init,
            app_final,
            ext_init,
            ext_final,
            update,
            on_event,
        );
    }
}
