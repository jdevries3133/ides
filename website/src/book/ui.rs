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

    Ok(Page {
        title: "Ides of August",
        children: &PageContainer {
            children: &Reader {
                reader_name: &auth.name,
            },
        },
    }
    .render())
}

struct Reader<'a> {
    reader_name: &'a str,
}
impl Component for Reader<'_> {
    fn render(&self) -> String {
        let reader_name = self.reader_name;
        format!(
            r#"
            <div class="w-full h-full">
            <p>reading as {reader_name}</p>
            </div>
            "#
        )
    }
}
