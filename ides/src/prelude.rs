pub use crate::error::{ErrStack, ErrT};
pub type Result<T> = std::result::Result<T, ErrStack>;
