//! Logging functions. **Use these instead of the macros listed on the main page!**

use std::ffi::CString;

/// Log message severity.
#[allow(missing_docs)]
pub enum Severity {
    Debug,
    UserDebug,
    Info,
    Warning,
    Error,
    Fatal,
}

impl From<Severity> for u32 {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Debug => 0,
            Severity::UserDebug => 1,
            Severity::Info => 2,
            Severity::Warning => 3,
            Severity::Error => 4,
            Severity::Fatal => 5,
        }
    }
}

impl From<Severity> for i32 {
    fn from(severity: Severity) -> Self {
        u32::from(severity) as i32
    }
}

/// Logs a message with the given severity.
pub fn log(severity: Severity, domain: &str, message: &str) {
    let message = CString::new(message).unwrap();
    let domain = CString::new(domain).unwrap();

    unsafe {
        dmsdk_ffi::LogInternal(severity.into(), domain.as_ptr(), message.as_ptr());
    }
}

/// Logs a message with severity [`Severity::Debug`].
///
/// The arguments are the same as [`format!`].
///
/// # Examples
/// ```
/// dmlog::debug!("This is a debug message!");
///
/// let lucky_number = 7;
/// dmlog::debug!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_debug {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Debug, LOG_DOMAIN, &format!($($arg)*));
    };
}

/// Logs a message with severity [`Severity::UserDebug`].
///
/// Its usage is the same as [`println!`].
///
/// # Examples
/// ```
/// dmlog::user_debug!("This is a user debug message!");
///
/// let lucky_number = 7;
/// dmlog::user_debug!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_user_debug {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::UserDebug, LOG_DOMAIN, &format!($($arg)*));
    };
}

/// Logs a message with severity [`Severity::Info`].
///
/// Its usage is the same as [`println!`].
///
/// # Examples
/// ```
/// dmlog::info!("This is an informative message!");
///
/// let lucky_number = 7;
/// dmlog::info!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_info {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Info, LOG_DOMAIN, &format!($($arg)*));
    };
}

/// Logs a message with severity [`Severity::Warning`].
///
/// Its usage is the same as [`println!`].
///
/// # Examples
/// ```
/// dmlog::warning!("This is a warning message!");
///
/// let lucky_number = 7;
/// dmlog::warning!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_warning {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Warning, LOG_DOMAIN, &format!($($arg)*));
    };
}

/// Logs a message with severity [`Severity::Error`].
///
/// Its usage is the same as [`println!`].
///
/// # Examples
/// ```
/// dmlog::error!("This is an error message!");
///
/// let lucky_number = 7;
/// dmlog::error!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_error {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Error, LOG_DOMAIN, &format!($($arg)*));
    };
}

/// Logs a message with severity [`Severity::Fatal`].
///
/// Its usage is the same as [`println!`].
///
/// # Examples
/// ```
/// dmlog::fatal!("Something has gone very, very wrong!");
///
/// let lucky_number = 7;
/// dmlog::fatal!("Lucky number: {lucky_number}");
/// ```
#[macro_export]
macro_rules! __internal_fatal {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Fatal, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(inline)]
pub use crate::{
    __internal_debug as debug, __internal_error as error, __internal_fatal as fatal,
    __internal_info as info, __internal_user_debug as user_debug, __internal_warning as warning,
};
