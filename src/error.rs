use thiserror::Error;

#[derive(Error, Debug)]
pub enum SleepToolError {
    #[error("config error: {0}")]
    Config(String),
    #[error("platform error: {0}")]
    Platform(String),
    #[error("monitor error: {0}")]
    Monitor(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SleepToolError>;
