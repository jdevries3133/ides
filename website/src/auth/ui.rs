use crate::{components::Saved, prelude::*};
use axum::{http::HeaderValue, response::Redirect};
use chrono::{Days, Utc};
use ides::auth::{Auth, AuthResult, Token};

#[derive(Default)]
struct TokenForm<'a> {
    token: Option<&'a Token>,
    indicate_token_is_invalid: bool,
}
impl Component for TokenForm<'_> {
    fn render(&self) -> String {
        let token_form_route = Route::Auth;
        let token = if let Some(token) = &self.token {
            token.display_secret_value()
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
                <form class="flex flex-col" method="POST" action="{token_form_route}">
                    <label for="token">Token</label>
                    <input id="token" name="token" type="password" value="{token}" />
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

pub async fn post_handler(
    State(AppState { db }): State<AppState>,
    Form(Payload { token }): Form<Payload>,
) -> Result<impl IntoResponse> {
    let mut headers = HeaderMap::new();
    let expiry_date = Utc::now()
        .checked_add_days(Days::new(365))
        .expect("heat death of the universe has not happened yet")
        .format("%a, %d %b %Y %H:%M:%S %Z");
    let val = HeaderValue::from_str(&format!(
        "token={token}; Path=/; HttpOnly; Expires={expiry_date}"
    ))
    .map_err(|e| {
        ErrStack::new(ErrT::AuthNonUtf8Cookie)
            .ctx(format!("submitted auth cookie is not utf-8: {e}"))
    })?;
    headers.insert("Set-Cookie", val);

    let token = Token::new(token);
    match Auth::get(&db, &token).await {
        AuthResult::Authenticated(_) => {
            Ok((headers, Redirect::to(&Route::Book.as_string()))
                .into_response())
        }
        AuthResult::NotAuthenticated => Ok((
            headers,
            [
                Saved {
                    message: "token updated",
                }
                .render(),
                render_token_form(Some(&token), !token.is_empty()),
            ]
            .join(""),
        )
            .into_response()),
        AuthResult::Err(e) => Err(e),
    }
}

pub async fn get_handler(headers: HeaderMap) -> Result<impl IntoResponse> {
    match Token::parse_from_headers(&headers) {
        Ok(token) => Ok(render_token_form(Some(&token), false)),
        Err(e) => match e.peek() {
            ErrT::AuthNotAuthenticated => Ok(render_token_form(None, false)),
            _ => Err(e),
        },
    }
}

fn render_token_form(
    token: Option<&Token>,
    indicate_token_is_invalid: bool,
) -> String {
    Page {
        title: "Configure token",
        children: &PageContainer {
            children: &TokenForm {
                token,
                indicate_token_is_invalid,
            },
        },
    }
    .render()
}
