use super::core::{parse_from_headers, Role, Token};
use crate::{components::Saved, prelude::*};
use axum::http::HeaderValue;

impl TryInto<Role> for String {
    type Error = ErrStack;
    fn try_into(self) -> Result<Role> {
        match self.as_str() {
            "reader" => Ok(Role::Reader),
            "admin" => Ok(Role::Admin),
            _ => Err(ErrStack::default()
                .wrap(ErrT::DbReturnedErronoeousRole)
                .ctx(format!("role {self} does not match an expected type"))),
        }
    }
}

struct TokenForm<'a> {
    token: Option<Token<'a>>,
}
impl Component for TokenForm<'_> {
    fn render(&self) -> String {
        let token_form_route = Route::Auth;
        let token = if let Some(token) = &self.token {
            token.0
        } else {
            ""
        };
        format!(
            r#"
            <form hx-post="{token_form_route}">
                <label for="token">Token</label>
                <input id="token" name="token" type="text" value="{token}" />
                <button>save</button>
            </form>
            "#
        )
    }
}

#[derive(Deserialize)]
pub struct Payload {
    token: String,
}

pub async fn handle_submit(
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
    Ok((
        headers,
        [
            Saved {
                message: "token updated",
            }
            .render(),
            TokenForm {
                token: Some(Token(&token)),
            }
            .render(),
        ]
        .join(""),
    ))
}

pub async fn render_form(headers: HeaderMap) -> Result<impl IntoResponse> {
    let token = parse_from_headers(&headers)?;
    Ok(Page {
        title: "Configure token",
        children: &PageContainer {
            children: &TokenForm { token: Some(token) },
        },
    }
    .render())
}
