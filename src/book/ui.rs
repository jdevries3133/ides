use super::access::log_access;
use crate::prelude::*;

pub async fn view(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    log_access(&db, 1, &headers)
        .await
        .map_err(|e| e.wrap(ErrT::BookUi))?;

    Ok("nice book")
}
