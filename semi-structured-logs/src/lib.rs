// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let log_message = match level {
        LogLevel::Info => "INFO",
        LogLevel::Warning => "WARNING",
        LogLevel::Error => "ERROR",
        LogLevel::Debug => "DEBUG"
    };
    return format!("[{}]: {}", log_message, message);
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
