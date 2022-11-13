//! Logging functions.

use std::ffi::CString;

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

pub fn log(severity: Severity, domain: &str, message: &str) {
    let message = CString::new(message).unwrap();
    let domain = CString::new(domain).unwrap();

    unsafe {
        dmsdk_ffi::LogInternal(severity.into(), domain.as_ptr(), message.as_ptr());
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! _debug {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Debug, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _user_debug {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::UserDebug, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _info {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Info, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _warning {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Warning, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _error {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Error, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! _fatal {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Fatal, LOG_DOMAIN, &format!($($arg)*));
    };
}

pub use crate::{
    _debug as debug, _error as error, _fatal as fatal, _info as info, _user_debug as user_debug,
    _warning as warning,
};
