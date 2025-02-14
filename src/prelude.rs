//! Dedupe of common internal and external imports

pub use crate::{
    auth::{Auth, AuthResult},
    components::{Component, Page, PageContainer},
    error::{ErrStack, ErrT},
    models::AppState,
    routes::Route,
};
pub use ammonia::clean;
pub use axum::{
    extract::{Form, State},
    http::HeaderMap,
    response::IntoResponse,
};
pub use serde::Deserialize;
pub use sqlx::{postgres::PgExecutor, query, query_as};

pub type Result<T> = std::result::Result<T, ErrStack>;
