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

#[macro_export]
macro_rules! _log_internal {
    ($severity:ident, $($arg:tt)*) => {};
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        _log_internal!(Debug, $($arg)+)
    };
}

#[macro_export]
macro_rules! user_debug {
    ($($arg:tt)+) => {
        _log_internal!(UserDebug, $($arg)+)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        dmlog::log(dmlog::Severity::Info, LOG_DOMAIN, &format!($($arg)*));
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)+) => {
        _log_internal!(Warning, $($arg)+)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        _log_internal!(Error, $($arg)+)
    };
}

#[macro_export]
macro_rules! fatal {
    ($($arg:tt)+) => {
        _log_internal!(Fatal, $($arg)+)
    };
}
