//! HTMX utils

use axum::{
    http::{HeaderMap, HeaderValue},
    response::{IntoResponse, Redirect},
};

pub const fn get_client_script() -> &'static str {
    concat!(
        include_str!("./htmx-2.0.2.vendor.js"),
        include_str!("./static/htmx_extras.js")
    )
}

pub fn redirect(mut headers: HeaderMap, to: &str) -> impl IntoResponse {
    headers.insert(
        "Hx-Redirect",
        HeaderValue::from_str(to)
            .unwrap_or(HeaderValue::from_str("/").unwrap()),
    );
    let response = Redirect::to(to);
    (headers, response)
}
