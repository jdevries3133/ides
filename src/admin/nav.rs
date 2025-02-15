use crate::{
    auth::{AuthResult, Role},
    htmx,
    prelude::*,
};
use axum::response::Response;

pub enum AdminNav {
    IsAdmin,
    GetOuttaHere(Response),
    Err(ErrStack),
}

pub fn nav_helper(auth_result: AuthResult) -> AdminNav {
    match auth_result {
        AuthResult::Authenticated(auth) => match auth.role {
            Role::Admin => AdminNav::IsAdmin,
            Role::Reader => AdminNav::GetOuttaHere(
                htmx::redirect(HeaderMap::new(), &Route::Book.as_string())
                    .into_response(),
            ),
        },
        AuthResult::NotAuthenticated => AdminNav::GetOuttaHere(
            htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response(),
        ),
        AuthResult::Err(e) => AdminNav::Err(e),
    }
}
