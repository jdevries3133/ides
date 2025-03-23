//! Core data-models for the application.

use serde::Deserialize;
use sqlx::PgPool;

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
}

pub struct SqlPagination {
    pub limit: i64,
    pub offset: i64,
}

/// Uses a default page size of 100 items.
impl From<PaginationParams> for SqlPagination {
    fn from(value: PaginationParams) -> Self {
        SqlPagination {
            limit: 100,
            offset: value.page.unwrap_or_default() * 100,
        }
    }
}
