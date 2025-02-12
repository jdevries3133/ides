//! Dedupe of common internal and external imports

pub use crate::{
    components::{Component, Page, PageContainer},
    error::{Err, ErrStack},
    routes::Route,
};
pub use ammonia::clean;
pub use axum::response::IntoResponse;

pub type Result<T> = std::result::Result<T, ErrStack>;
