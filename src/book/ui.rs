use super::access::log_access;
use crate::{htmx, prelude::*};

pub async fn book_controller(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(auth) => {
            Ok(book_view(&auth, &db).await?.into_response())
        }
        AuthResult::NotAuthenticated => {
            Ok(htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response())
        }
        AuthResult::Err(e) => Err(e),
    }
}

async fn book_view(
    auth: &Auth,
    db: impl PgExecutor<'_> + Copy,
) -> Result<impl IntoResponse> {
    log_access(auth, db, 1)
        .await
        .map_err(|e| e.wrap(ErrT::BookUi))?;

    Ok("nice book".into_response())
}
