use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to read save file: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid or unsupported save format: {0}")]
    InvalidFormat(String),
}
