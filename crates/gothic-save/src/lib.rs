mod error;
mod save;

pub use error::Error;
pub use save::Save;

pub type Result<T> = std::result::Result<T, Error>;
