//! All possible routes with their params are defined in a big enum.

use super::{auth, book, controllers, legal, middleware, models};
use axum::{
    middleware::from_fn,
    routing::{get, post, Router},
};

/// This enum contains all of the route strings in the application. This
/// solves several problems.
///
/// 1. Maintaining a single source of truth for route paths, even if it has
///    multiple controllers for various HTTP methods
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
    About,
    Auth,
    Book,
    Favicon,
    Htmx,
    Ping,
    PrivacyPolicy,
    RobotsTxt,
    Root,
    StaticAppleIcon,
    StaticLargeIcon,
    StaticManifest,
    StaticMaskableLargeIcon,
    StaticMaskableMediumIcon,
    StaticMaskableSmallIcon,
    StaticMediumIcon,
    StaticSmallIcon,
    StaticTinyIcon,
    TermsOfService,
    /// Route which will return an empty string. This is mainly an HTMX utility
    /// to allow a component to easily be swapped with nothing.
    Void,
}

impl Route {
    pub fn as_string(&self) -> String {
        match self {
            Self::About => "/about".into(),
            Self::Auth => "/auth".into(),
            Self::Book => "/book".into(),
            Self::Favicon => "/favicon.ico".into(),
            Self::Htmx => "/generated/htmx-2.0.2".into(),
            Self::Ping => "/ping".into(),
            Self::PrivacyPolicy => "/privacy".into(),
            Self::Root => "/".into(),
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
            Self::TermsOfService => "/terms".into(),
            Self::Void => "/void".into(),
        }
    }
}

impl std::fmt::Display for Route {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

/// In [crate::main], these routes are not protected by any authentication, so
/// any requester can access these routes.
fn get_public_routes() -> Router<models::AppState> {
    Router::new()
        .route(&Route::About.as_string(), get(controllers::about))
        .route(&Route::Auth.as_string(), get(auth::ui::render_form))
        .route(&Route::Auth.as_string(), post(auth::ui::handle_submit))
        .route(&Route::Book.as_string(), get(book::view_book))
        .route(&Route::Favicon.as_string(), get(controllers::get_favicon))
        .route(&Route::Htmx.as_string(), get(controllers::get_htmx_js))
        .route(
            &Route::StaticTinyIcon.as_string(),
            get(controllers::get_tiny_icon),
        )
        .route(&Route::Ping.as_string(), get(controllers::pong))
        .route(
            &Route::PrivacyPolicy.as_string(),
            get(legal::get_privacy_policy),
        )
        .route(
            &Route::RobotsTxt.as_string(),
            get(controllers::get_robots_txt),
        )
        .route(&Route::Root.as_string(), get(controllers::root))
        .route(
            &Route::StaticAppleIcon.as_string(),
            get(controllers::get_apple_icon),
        )
        .route(
            &Route::StaticLargeIcon.as_string(),
            get(controllers::get_large_icon),
        )
        .route(
            &Route::StaticManifest.as_string(),
            get(controllers::get_manifest),
        )
        .route(
            &Route::StaticMaskableLargeIcon.as_string(),
            get(controllers::get_maskable_large_icon),
        )
        .route(
            &Route::StaticMaskableMediumIcon.as_string(),
            get(controllers::get_maskable_medium_icon),
        )
        .route(
            &Route::StaticMaskableSmallIcon.as_string(),
            get(controllers::get_maskable_small_icon),
        )
        .route(
            &Route::StaticMediumIcon.as_string(),
            get(controllers::get_medium_icon),
        )
        .route(
            &Route::StaticSmallIcon.as_string(),
            get(controllers::get_small_icon),
        )
        .route(&Route::TermsOfService.as_string(), get(legal::get_tos))
        .route(&Route::Void.as_string(), get(controllers::void))
}

pub fn get_routes() -> Router<models::AppState> {
    let public_routes = get_public_routes()
        .layer(from_fn(middleware::html_headers))
        .layer(from_fn(middleware::log));

    Router::new().nest("/", public_routes)
}
