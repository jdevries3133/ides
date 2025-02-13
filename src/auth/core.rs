use crate::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Token<'a>(pub &'a str);

#[derive(Debug, Eq, PartialEq)]
pub enum Role {
    Reader,
    Admin,
}

pub fn parse_from_headers(headers: &HeaderMap) -> Result<Token<'_>> {
    let cookie = headers.get("Cookie");
    if let Some(cookie) = cookie {
        let cookie_str = cookie.to_str().map_err(|e| {
            ErrStack::default()
                .wrap(ErrT::AuthNonUtf8Cookie)
                .ctx(format!("cannot stringify cookie: {e}"))
        })?;
        for item in cookie_str.split(";") {
            let item = item.trim();
            let mut key_val = item.split("=");
            let key = key_val.next();
            let val = key_val.next();
            if let (Some(key), Some(val)) = (key, val) {
                if key.trim() == "token" {
                    return Ok(Token(val.trim()));
                }
            };
        }
    }

    Err(ErrStack::default()
        .wrap(ErrT::AuthNotAuthenticated)
        .ctx("parse_from_headers could not find a token".into()))
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::http::HeaderValue;

    fn case(cookie: &str, expect: Result<Token>) {
        let mut h = HeaderMap::new();
        h.insert("Cookie", HeaderValue::from_str(cookie).unwrap());
        let result = parse_from_headers(&h);
        assert_eq!(result, expect);
    }

    #[test]
    fn test_parse_from_empty_headers() {
        let h = HeaderMap::new();
        let result = parse_from_headers(&h);
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
        let result = parse_from_headers(&h);
        if let Err(e) = result {
            assert_eq!(e.jenga().next(), Some(&ErrT::AuthNotAuthenticated));
        } else {
            panic!("expected result to be an error");
        }
    }

    #[test]
    fn test_parse_from_two_jar_cookie_first_spot() {
        case("token=foo ; bar=baz", Ok(Token("foo")));
    }

    #[test]
    fn test_parse_from_two_jar_cookie_second_spot() {
        case("bar=baz; token= foo;", Ok(Token("foo")));
    }

    #[test]
    fn test_parse_with_weird_whitespace() {
        case(
            "bar = baz ; token=boo bar ; other=value",
            Ok(Token("boo bar")),
        );
    }

    #[test]
    fn test_case_insensitive() {
        let mut h = HeaderMap::new();
        h.insert("cookie", HeaderValue::from_str("token=foo").unwrap());
        let result = parse_from_headers(&h);
        assert_eq!(result, Ok(Token("foo")));
    }
}
