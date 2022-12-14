//! Configuration file access functions. The configuration file is the compiled version of game.project.

use dmsdk_ffi::dmConfigFile;
use std::ffi::{c_char, CStr, CString};

/// Pointer to a project config file.
pub type ConfigFile = dmConfigFile::HConfig;

///
pub struct Config {}

/// Gets the corresponding config value as a String.
///
/// `default_value` will be returned if the key isn't found.
///
/// # Examples
/// ```
/// # const LOG_DOMAIN: &str = "DOCTEST";
/// use dmsdk::*;
///
/// fn app_init(params: dmextension::AppParams) -> dmextension::Result {
///     let title = unsafe { dmconfigfile::get_string(params.config, "project.title", "Untitled") };
///     dmlog::info!("Project title is: {title}");
///
///     dmextension::Result::Ok
/// }
/// ```
///
/// # Safety
///
/// This function is safe as long as `config` points to a valid config file.
pub unsafe fn get_string(config: ConfigFile, key: &str, default_value: &str) -> String {
    let key = CString::new(key).unwrap();
    let default_value = CString::new(default_value).unwrap();

    let ptr = dmConfigFile::GetString(config, key.as_ptr(), default_value.as_ptr());
    let cstr = CStr::from_ptr(ptr);
    String::from_utf8_lossy(cstr.to_bytes()).into_owned()
}

/// Gets the corresponding config value as an i32.
///
/// `default_value` will be returned if the key isn't found or if the value found isn't a valid integer.
///
/// # Examples
/// ```
/// # const LOG_DOMAIN: &str = "DOCTEST";
/// use dmsdk::*;
///
/// fn app_init(params: dmextension::AppParams) -> dmextension::Result {
///     let display_width = unsafe { dmconfigfile::get_int(params.config, "display.width", 960) };
///     dmlog::info!("Window width is: {display_width}");
///
///     dmextension::Result::Ok
/// }
/// ```
///
/// # Safety
///
/// This function is safe as long as `config` points to a valid config file.
pub unsafe fn get_int(config: ConfigFile, key: &str, default_value: i32) -> i32 {
    let key = CString::new(key).unwrap();
    dmConfigFile::GetInt(config, key.as_ptr(), default_value)
}

/// Gets the corresponding config value as an f32.
///
/// `default_value` will be returned instead if the key isn't found or if the value found isn't a valid float.
///
/// # Examples
/// ```
/// # const LOG_DOMAIN: &str = "DOCTEST";
/// use dmsdk::*;
///
/// fn app_init(params: dmextension::AppParams) -> dmextension::Result {
///     let gravity = unsafe { dmconfigfile::get_float(params.config, "physics.gravity_y", -9.8) };
///     dmlog::info!("Gravity is: {gravity}");
///
///     dmextension::Result::Ok
/// }
/// ```
///
/// # Safety
///
/// This function is safe as long as `config` points to a valid config file.
pub unsafe fn get_float(config: ConfigFile, key: &str, default_value: f32) -> f32 {
    let key = CString::new(key).unwrap();
    dmConfigFile::GetFloat(config, key.as_ptr(), default_value)
}

pub type PluginLifecycle = fn(ConfigFile);
pub type PluginGetter<T> = fn(ConfigFile, &str, T) -> Option<T>;
#[doc(hidden)]
pub type RawPluginLifecycle = unsafe extern "C" fn(ConfigFile);
#[doc(hidden)]
pub type RawPluginGetter<T> = unsafe extern "C" fn(ConfigFile, *const c_char, T, *mut T) -> bool;
#[doc(hidden)]
pub type Desc = [u8; DESC_BUFFER_SIZE as usize];

#[doc(hidden)]
pub const DESC_BUFFER_SIZE: u32 = 64;

#[doc(hidden)]
#[macro_export]
macro_rules! declare_plugin_lifecycle {
    ($symbol:ident, $option:expr) => {
        #[no_mangle]
        unsafe extern "C" fn $symbol(config: dmconfigfile::ConfigFile) {
            let func: Option<dmconfigfile::PluginLifecycle> = $option;
            if let Some(func) = func {
                func(config);
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! declare_plugin_getter {
    ($symbol:ident, $option:expr, $type:ident) => {
        #[no_mangle]
        unsafe extern "C" fn $symbol(
            config: dmconfigfile::ConfigFile,
            key: *const core::ffi::c_char,
            default_value: $type,
            out: *mut $type,
        ) -> bool {
            let func: Option<dmconfigfile::PluginGetter<$type>> = $option;
            if let Some(func) = func {
                let key = core::ffi::CStr::from_ptr(key)
                    .to_str()
                    .expect("Invalid UTF-8 sequence in key!");
                if let Some(value) = func(config, key, default_value) {
                    out.write(value);
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
    };
}

#[macro_export]
macro_rules! declare_configfile_extension {
    ($symbol:ident, $create:expr, $destroy:expr, $get_string:expr, $get_int:expr, $get_float:expr) => {
        paste! {
            static mut [<$symbol _PLUGIN_DESC>]: dmconfigfile::Desc = [0u8; dmconfigfile::DESC_BUFFER_SIZE as usize];

            declare_plugin_lifecycle!([<$symbol _plugin_create>], $create);
            declare_plugin_lifecycle!([<$symbol _plugin_destroy>], $destroy);
            declare_plugin_getter!([<$symbol _plugin_get_int>], $get_int, i32);
            declare_plugin_getter!([<$symbol _plugin_get_float>], $get_float, f32);

            #[no_mangle]
            unsafe extern "C" fn [<$symbol _plugin_get_string>](
                config: dmconfigfile::ConfigFile,
                key: *const core::ffi::c_char,
                default_value: *const core::ffi::c_char,
                out: *mut *const core::ffi::c_char,
            ) -> bool {
                let func: Option<dmconfigfile::PluginGetter<&str>> = $get_string;
                if let Some(func) = func {
                    let key = core::ffi::CStr::from_ptr(key)
                        .to_str();
                    if key.is_err() {
                        dmlog::error!("Invalid UTF-8 sequence in key!");
                        return false;
                    }

                    let default_value = core::ffi::CStr::from_ptr(default_value)
                        .to_str();
                    if default_value.is_err() {
                        dmlog::error!("Invalid UTF-8 sequence in default_value!");
                        return false;
                    }

                    if let Some(value) = func(config, key.unwrap(), default_value.unwrap()) {
                        let cstr = std::ffi::CString::new(value).expect("Unexpected null in return value!");
                        out.write(cstr.as_ptr());
                        true
                    } else{
                        false
                    }
                } else {
                    false
                }
            }

            #[no_mangle]
            #[dmextension::ctor]
            unsafe fn $symbol() {
                dmconfigfile::register(
                    &mut [<$symbol _PLUGIN_DESC>],
                    stringify!($symbol),
                    [<$symbol _plugin_create>],
                    [<$symbol _plugin_destroy>],
                    [<$symbol _plugin_get_string>],
                    [<$symbol _plugin_get_int>],
                    [<$symbol _plugin_get_float>],
                );
            }
        }
    };
}

#[doc(hidden)]
pub fn register(
    desc: &mut Desc,
    name: &str,
    create: RawPluginLifecycle,
    destroy: RawPluginLifecycle,
    get_string: RawPluginGetter<*const c_char>,
    get_int: RawPluginGetter<i32>,
    get_float: RawPluginGetter<f32>,
) {
    let name = CString::new(name).unwrap();
    unsafe {
        dmConfigFile::Register(
            desc.as_mut_ptr() as *mut dmConfigFile::PluginDesc,
            DESC_BUFFER_SIZE,
            name.as_ptr(),
            Some(create),
            Some(destroy),
            Some(get_string),
            Some(get_int),
            Some(get_float),
        );
    }
}
