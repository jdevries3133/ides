use super::core::{parse_from_headers, Auth, AuthResult, Token};
use crate::{components::Saved, htmx, prelude::*};
use axum::http::HeaderValue;

#[derive(Default)]
struct TokenForm<'a> {
    token: Option<Token<'a>>,
    indicate_token_is_invalid: bool,
}
impl Component for TokenForm<'_> {
    fn render(&self) -> String {
        let token_form_route = Route::Auth;
        let token = if let Some(token) = &self.token {
            token.0
        } else {
            ""
        };
        let validation_msg = if self.indicate_token_is_invalid {
            r#"<p class="text-red-500">token is not valid</p>"#
        } else {
            ""
        };
        format!(
            r#"
            <div class="flex items-center justify-center h-[80vh]">
                <form class="flex flex-col" hx-post="{token_form_route}">
                    <label for="token">Token</label>
                    <input id="token" name="token" type="text" value="{token}" />
                    {validation_msg}
                    <button>save</button>
                </form>
            </div>
            "#
        )
    }
}

#[derive(Deserialize)]
pub struct Payload {
    token: String,
}

pub async fn handle_submit(
    State(AppState { db }): State<AppState>,
    Form(Payload { token }): Form<Payload>,
) -> Result<impl IntoResponse> {
    let mut headers = HeaderMap::new();
    let val =
        HeaderValue::from_str(&format!("token={token}")).map_err(|e| {
            ErrStack::default()
                .wrap(ErrT::AuthNonUtf8Cookie)
                .ctx(format!("submitted auth cookie is not utf-8: {e}"))
        })?;
    headers.insert("Set-Cookie", val);

    match Auth::get(&db, &Token(&token)).await {
        AuthResult::Authenticated(_) => {
            Ok(htmx::redirect(headers, &Route::Book.as_string())
                .into_response())
        }
        AuthResult::NotAuthenticated => Ok((
            headers,
            [
                Saved {
                    message: "token updated",
                }
                .render(),
                TokenForm {
                    token: Some(Token(&token)),
                    indicate_token_is_invalid: !token.is_empty(),
                }
                .render(),
            ]
            .join(""),
        )
            .into_response()),
        AuthResult::Err(e) => Err(e),
    }
}

pub async fn render_form(headers: HeaderMap) -> Result<impl IntoResponse> {
    let token = parse_from_headers(&headers)?;
    Ok(Page {
        title: "Configure token",
        children: &PageContainer {
            children: &TokenForm {
                token: Some(token),
                indicate_token_is_invalid: false,
            },
        },
    }
    .render())
}
