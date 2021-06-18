mod exception;
mod fileopration;

pub use exception::Error::{CannotCopyError, FileNotFoundError};
pub use fileopration::FileOperation;
