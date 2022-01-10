use std::io;
use thiserror::Error;

pub type ConnectResult<T> = Result<T, ConnectError>;

#[derive(Debug, Error)] // Наследую трейты
pub enum ConnectError {
    #[error("Unexpected handshake response: {0}")] // реализует трейт Display
    BadHandshake(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error), // Атрибут from генерирует трейт From
}

pub type SendResult = Result<(), SendError>;

#[derive(Debug, Error)]
pub enum SendError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

pub type RecvResult = Result<String, RecvError>;

#[derive(Debug, Error)]
pub enum RecvError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("bad encoding")]
    BadEncoding,
}
