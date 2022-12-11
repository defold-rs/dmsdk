//! Configuration file access functions. The configuration file is the compiled version of game.project.

use dmsdk_ffi::dmConfigFile;
use std::ffi::{CStr, CString};

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
