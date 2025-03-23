//! All possible routes with their params are defined in a big enum.

use super::{about, admin, auth, book, middleware, models, r#static};
use axum::{
    middleware::from_fn,
    routing::{delete, get, post, Router},
};

/// This enum contains all of the route strings in the application. This
/// solves several problems.
///
/// 1. Maintaining a single source of truth for route paths, even if it has
///    multiple r#static for various HTTP methods
/// 2. Making it easier to refactor routing without needing to keep the axum
///    router and paths referenced in routers in sync.
/// 3. Making it easier to jump from a component to the handlers in a route it
///    references and visa versa.
/// 4. Further decoupling the app from the underlying HTTP.
/// 5. Allowing documentation on a route, which is super useful for quick
///    reference when authoring components.
///
/// For each route, the parameters are inside an Option<T>. If no parameters
/// are provided, we'll construct the route with the `:id` template in it
/// for the Axum router.
pub enum Route {
    /// Route which will return an empty string. This is mainly an HTMX utility
    /// to allow a component to easily be swapped with nothing.
    AdminHome,
    AdminImportBook,
    AdminChangeRevision,
    AdminToken,
    AdminRevokeToken {
        token_id: Option<i32>,
    },
    Auth,
    About,
    Book,
    BookComment {
        block_id: Option<i32>,
    },
    BookNextPage,
    BookPrevPage,
    Favicon,
    Htmx,
    Ping,
    RobotsTxt,
    StaticAppleIcon,
    StaticLargeIcon,
    StaticManifest,
    StaticMaskableLargeIcon,
    StaticMaskableMediumIcon,
    StaticMaskableSmallIcon,
    StaticMediumIcon,
    StaticSmallIcon,
    StaticTinyIcon,
    Void,
}

impl Route {
    pub fn as_string(&self) -> String {
        match self {
            Self::AdminHome => "/admin".into(),
            Self::AdminImportBook => "/admin/import-book".into(),
            Self::AdminChangeRevision => "/admin/change-revision".into(),
            Self::AdminToken => "/admin/manage-tokens".into(),
            Self::AdminRevokeToken { token_id } => match token_id {
                Some(id) => format!("/admin/manage-tokens/{id}"),
                None => "/admin/manage-tokens/:token_id".into(),
            },
            Self::Auth => "/".into(),
            Self::About => "/about".into(),
            Self::Book => "/book".into(),
            Self::BookComment { block_id } => match block_id {
                Some(id) => format!("/block/{id}/comment"),
                None => "/block/:block_id/comment".into(),
            },
            Self::BookNextPage => "/book/next-page".into(),
            Self::BookPrevPage => "/book/prev-page".into(),
            Self::Favicon => "/favicon.ico".into(),
            Self::Htmx => "/generated/htmx-2.0.2-mod3".into(),
            Self::Ping => "/ping".into(),
            Self::RobotsTxt => "/robots.txt".into(),
            Self::StaticAppleIcon => "/static/apple_icon".into(),
            Self::StaticLargeIcon => "/static/large-icon".into(),
            Self::StaticManifest => "/static/manifest".into(),
            Self::StaticMaskableLargeIcon => {
                "/static/maskable-large-icon".into()
            }
            Self::StaticMaskableMediumIcon => {
                "/static/maskable-medium-icon".into()
            }
            Self::StaticMaskableSmallIcon => {
                "/static/maskable-small-icon".into()
            }
            Self::StaticMediumIcon => "/static/icon".into(),
            Self::StaticSmallIcon => "/static/xs-icon".into(),
            Self::StaticTinyIcon => "/static/xxs-icon".into(),
            Self::Void => "/void".into(),
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

pub fn get_routes() -> Router<models::AppState> {
    Router::new()
        .route(&Route::About.as_string(), get(about::about))
        .route(&Route::AdminHome.as_string(), get(admin::home))
        .route(
            &Route::AdminImportBook.as_string(),
            get(admin::import_book_ui),
        )
        .route(
            &Route::AdminImportBook.as_string(),
            post(admin::handle_import_book),
        )
        .route(
            &Route::AdminChangeRevision.as_string(),
            get(admin::change_revision),
        )
        .route(
            &Route::AdminChangeRevision.as_string(),
            post(admin::handle_revision_change),
        )
        .route(&Route::AdminToken.as_string(), get(admin::manage_tokens))
        .route(
            &Route::AdminToken.as_string(),
            post(admin::handle_create_token),
        )
        .route(
            &Route::AdminRevokeToken { token_id: None }.as_string(),
            delete(admin::handle_revoke_token),
        )
        .route(&Route::Auth.as_string(), get(auth::ui::get_handler))
        .route(&Route::Auth.as_string(), post(auth::ui::post_handler))
        .route(&Route::Book.as_string(), get(book::ui))
        .route(
            &Route::BookComment { block_id: None }.as_string(),
            get(book::comment),
        )
        .route(
            &Route::BookComment { block_id: None }.as_string(),
            post(book::handle_comment),
        )
        .route(&Route::BookNextPage.as_string(), get(book::next_page))
        .route(&Route::BookPrevPage.as_string(), get(book::prev_page))
        .route(&Route::Favicon.as_string(), get(r#static::get_favicon))
        .route(&Route::Htmx.as_string(), get(r#static::get_htmx_js))
        .route(
            &Route::StaticTinyIcon.as_string(),
            get(r#static::get_tiny_icon),
        )
        .route(&Route::Ping.as_string(), get(r#static::pong))
        .route(&Route::RobotsTxt.as_string(), get(r#static::get_robots_txt))
        .route(
            &Route::StaticAppleIcon.as_string(),
            get(r#static::get_apple_icon),
        )
        .route(
            &Route::StaticLargeIcon.as_string(),
            get(r#static::get_large_icon),
        )
        .route(
            &Route::StaticManifest.as_string(),
            get(r#static::get_manifest),
        )
        .route(
            &Route::StaticMaskableLargeIcon.as_string(),
            get(r#static::get_maskable_large_icon),
        )
        .route(
            &Route::StaticMaskableMediumIcon.as_string(),
            get(r#static::get_maskable_medium_icon),
        )
        .route(
            &Route::StaticMaskableSmallIcon.as_string(),
            get(r#static::get_maskable_small_icon),
        )
        .route(
            &Route::StaticMediumIcon.as_string(),
            get(r#static::get_medium_icon),
        )
        .route(
            &Route::StaticSmallIcon.as_string(),
            get(r#static::get_small_icon),
        )
        .route(&Route::Void.as_string(), get(r#static::void))
        .layer(from_fn(middleware::html_headers))
        .layer(from_fn(middleware::log))
}
