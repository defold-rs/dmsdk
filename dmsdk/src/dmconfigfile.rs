//! Configuration file access functions. The configuration file is the compiled version of game.project.

use dmsdk_ffi::dmConfigFile;
use std::ffi::{CStr, CString};

pub type ConfigFile = dmConfigFile::HConfig;

pub struct Config {}

/// Gets the corresponding config value as a String.
///
/// `default_value` will be returned if the key isn't found.
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn app_init(config: dmconfigfile::ConfigFile) {
///     let title = dmconfigfile::get_string(config, "project.title", "Untitled");
///     dmlog::info!("Project title is: {title}");
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
    CStr::from_ptr(ptr).to_string_lossy().into_owned()
}

/// Gets the corresponding config value as an i32.
///
/// `default_value` will be returned if the key isn't found or if the value found isn't a valid integer.
///
/// # Examples
/// ```
/// use dmsdk::*;
///
/// fn app_init(config: dmconfigfile::ConfigFile) {
///     let display_width = dmconfigfile::get_int(config, "display.width", 960);
///     dmlog::info!("Window width is: {display_width}");
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
/// use dmsdk::*;
///
/// fn app_init(config: dmconfigfile::ConfigFile) {
///     let gravity = dmconfigfile::get_float(config, "physics.gravity_y", -9.8);
///     dmlog::info!("Gravity is: {gravity}");
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
