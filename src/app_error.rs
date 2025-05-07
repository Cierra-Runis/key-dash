use config::ConfigError;
use std::io;
use strum::Display;
use tokio::sync::mpsc::error::SendError;

/// This is a custom error type for the application.
///
/// It is used to wrap different error types that can occur in the application.
#[derive(Debug, Display)]
pub enum AppError<T> {
    /// It is a wrapper for the [`std::io::Error`].
    Io(io::Error),
    /// It is a wrapper for the [`tokio::sync::mpsc::error::SendError`].
    Send(SendError<T>),
    /// It is a wrapper for the [`config::ConfigError`].
    Config(ConfigError),
    /// It is an invalid action.
    InvalidAction(String),
    /// It is an invalid event.
    InvalidEvent(String),
    /// It is used when a key is already bound.
    AlreadyBound,
    /// It is an invalid color.
    InvalidColor(String),
}

impl<T> From<io::Error> for AppError<T> {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl<T> From<SendError<T>> for AppError<T> {
    fn from(error: SendError<T>) -> Self {
        Self::Send(error)
    }
}

impl<T> From<ConfigError> for AppError<T> {
    fn from(error: ConfigError) -> Self {
        Self::Config(error)
    }
}
