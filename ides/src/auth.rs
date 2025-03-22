use crate::{bytes::Bytes, prelude::*};
use aws_lc_rs::{
    digest::{Context, SHA256},
    rand::fill,
};

pub struct Auth {
    #[allow(unused)]
    pub name: String,
    #[allow(unused)]
    pub role: Role,
    pub token_id: i32,
}

pub enum AuthResult {
    Authenticated(Auth),
    NotAuthenticated,
    Err(ErrStack),
}

impl Auth {
    pub async fn from_headers(
        db: impl PgExecutor<'_>,
        headers: &HeaderMap,
    ) -> AuthResult {
        match Token::parse_from_headers(headers) {
            Ok(token) => Self::get(db, &token).await,
            Err(e) => AuthResult::Err(e),
        }
    }
    pub async fn get(db: impl PgExecutor<'_>, token: &Token) -> AuthResult {
        #[derive(Debug)]
        struct Qres {
            name: String,
            role: String,
            token_id: i32,
        }
        let result = query_as!(
            Qres,
            "select t.id token_id, t.name, r.name as role
            from token t
            join role r on r.id = t.role_id
            where t.token_digest = $1",
            token.sha256_hex()
        )
        .fetch_optional(db)
        .await
        .map_err(|e| {
            ErrStack::new(ErrT::SqlxError).ctx(format!("Auth::get: {e}"))
        });
        match result {
            Ok(Some(Qres {
                name,
                role,
                token_id,
            })) => {
                match role.try_into().map_err(|e: ErrStack| {
                    e.wrap(ErrT::SqlxError).ctx("during Auth::get".into())
                }) {
                    Ok(role) => AuthResult::Authenticated(Auth {
                        name,
                        role,
                        token_id,
                    }),
                    Err(e) => AuthResult::Err(e),
                }
            }
            Ok(None) => AuthResult::NotAuthenticated,
            Err(e) => AuthResult::Err(e),
        }
    }
}

#[derive(Eq, PartialEq)]
pub struct Token(String);

impl Token {
    pub fn new(token: String) -> Self {
        Self(token)
    }
    pub fn create() -> Result<Self> {
        let mut buffer = [0u8; 66];
        fill(&mut buffer).map_err(|e| {
            ErrStack::new(ErrT::Invariant)
                .ctx(format!("aws says no random bytes for you: {e}"))
        })?;
        Ok(Self::new(buffer.to_base64()))
    }
    pub fn sha256_hex(&self) -> String {
        let mut ctx = Context::new(&SHA256);
        ctx.update(self.0.as_bytes());
        let digest = ctx.finish();
        digest.as_ref().to_hex()
    }
    pub fn parse_from_headers(headers: &HeaderMap) -> Result<Self> {
        let cookie = headers.get("Cookie");
        if let Some(cookie) = cookie {
            let cookie_str = cookie.to_str().map_err(|e| {
                ErrStack::new(ErrT::AuthNonUtf8Cookie)
                    .ctx(format!("cannot stringify cookie: {e}"))
            })?;
            for item in cookie_str.split(";") {
                let item = item.trim();
                let mut key_val = item.split("=");
                let key = key_val.next();
                let val = key_val.next();
                if let (Some(key), Some(val)) = (key, val) {
                    if key.trim() == "token" {
                        return Ok(Self(val.trim().to_string()));
                    }
                };
            }
        }

        Err(ErrStack::new(ErrT::AuthNotAuthenticated)
            .ctx("parse_from_headers could not find a token".into()))
    }
    pub fn display_secret_value(&self) -> &str {
        &self.0
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Token([sensitive value omitted])")
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Role {
    Reader,
    Admin,
}

impl TryInto<Role> for String {
    type Error = ErrStack;
    fn try_into(self) -> Result<Role> {
        match self.as_str() {
            "reader" => Ok(Role::Reader),
            "admin" => Ok(Role::Admin),
            _ => Err(ErrStack::new(ErrT::DbReturnedErronoeousRole)
                .ctx(format!("role {self} does not match an expected type"))),
        }
    }
}

impl From<Role> for i32 {
    fn from(val: Role) -> Self {
        match val {
            Role::Reader => 1,
            Role::Admin => 2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::HeaderValue;

    fn case(cookie: &str, expect: Result<Token>) {
        let mut h = HeaderMap::new();
        h.insert("Cookie", HeaderValue::from_str(cookie).unwrap());
        let result = Token::parse_from_headers(&h);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_parse_from_empty_headers() {
        let h = HeaderMap::new();
        let result = Token::parse_from_headers(&h);
        if let Err(e) = result {
            assert_eq!(e.jenga().next(), Some(&ErrT::AuthNotAuthenticated));
        } else {
            panic!("expected result to be an error");
        }
    }

    #[test]
    fn test_parse_from_empty_cookie() {
        let mut h = HeaderMap::new();
        h.insert("Cookie", HeaderValue::from_str("").unwrap());
        let result = Token::parse_from_headers(&h);
        if let Err(e) = result {
            assert_eq!(e.peek(), &ErrT::AuthNotAuthenticated);
        } else {
            panic!("expected result to be an error");
        }
    }

    #[test]
    fn test_parse_from_two_jar_cookie_first_spot() {
        case("token=foo ; bar=baz", Ok(Token("foo".into())));
    }

    #[test]
    fn test_parse_from_two_jar_cookie_second_spot() {
        case("bar=baz; token= foo;", Ok(Token("foo".into())));
    }

    #[test]
    fn test_parse_with_weird_whitespace() {
        case(
            "bar = baz ; token=boo bar ; other=value",
            Ok(Token("boo bar".into())),
        );
    }

    #[test]
    fn test_case_insensitive() {
        let mut h = HeaderMap::new();
        h.insert("cookie", HeaderValue::from_str("token=foo").unwrap());
        let result = Token::parse_from_headers(&h);
        assert_eq!(result, Ok(Token("foo".into())));
    }
}
