//! Reader UI

use super::access::log_access;
use crate::{htmx, prelude::*};

#[derive(Deserialize)]
pub struct Params {
    page: Option<i32>,
    #[allow(dead_code)]
    /// On the client, this is set to the product of `window.innerHeight` &
    /// `window.innerWidth`.
    screen_area: i32,
}

pub async fn ui(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
    params: Query<Params>,
) -> Result<impl IntoResponse> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(auth) => {
            log_access(&auth, &db, params.page.unwrap_or_default())
                .await
                .map_err(|e| {
                    e.wrap(ErrT::BookUi).ctx("while accessing book UI".into())
                })?;

            Ok(Page {
                title: "Ides of August",
                children: &PageContainer {
                    children: &Reader {
                        reader_name: &auth.name,
                    },
                },
            }
            .render()
            .into_response())
        }
        AuthResult::NotAuthenticated => {
            Ok(htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response())
        }
        AuthResult::Err(e) => Err(e),
    }
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
