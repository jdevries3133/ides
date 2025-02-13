//! Core data-models for the application.

use sqlx::PgPool;

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
