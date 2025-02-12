//! Core data-models for the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[allow(unused)]
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}
