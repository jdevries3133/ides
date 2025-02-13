//! Dedupe of common internal and external imports

pub use crate::{
    components::{Component, Page, PageContainer},
    error::{ErrStack, ErrT},
    routes::Route,
};
pub use ammonia::clean;
pub use axum::{
    extract::Form,
    http::HeaderMap,
    response::IntoResponse,
};
pub use serde::Deserialize;

pub type Result<T> = std::result::Result<T, ErrStack>;
