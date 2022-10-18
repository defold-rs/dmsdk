//! Logging functions.

use std::ffi::CString;

fn log(severity: u32, domain: &str, message: &str) {
    let message = CString::new(message).unwrap();
    let domain = CString::new(domain).unwrap();

    unsafe {
        dmsdk_ffi::LogInternal(severity, domain.as_ptr(), message.as_ptr());
    }
}

pub fn debug(domain: &str, message: &str) {
    log(dmsdk_ffi::LogSeverity_LOG_SEVERITY_DEBUG, domain, message);
}

pub fn user_debug(domain: &str, message: &str) {
    log(
        dmsdk_ffi::LogSeverity_LOG_SEVERITY_USER_DEBUG,
        domain,
        message,
    );
}

pub fn info(domain: &str, message: &str) {
    log(dmsdk_ffi::LogSeverity_LOG_SEVERITY_INFO, domain, message);
}

pub fn warning(domain: &str, message: &str) {
    log(dmsdk_ffi::LogSeverity_LOG_SEVERITY_WARNING, domain, message);
}

pub fn error(domain: &str, message: &str) {
    log(dmsdk_ffi::LogSeverity_LOG_SEVERITY_ERROR, domain, message);
}

pub fn fatal(domain: &str, message: &str) {
    log(dmsdk_ffi::LogSeverity_LOG_SEVERITY_FATAL, domain, message);
}
