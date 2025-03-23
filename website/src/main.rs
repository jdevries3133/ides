#![feature(let_chains)]

use dotenvy::dotenv;
use ides::prelude::*;
use std::net::SocketAddr;

mod about;
mod admin;
mod auth;
mod book;
mod components;
mod htmx;
mod middleware;
mod models;
mod prelude;
mod routes;
mod r#static;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let db = ides::db::create_pg_pool().await?;
    sqlx::migrate!().run(&db).await.map_err(|e| {
        ErrStack::new(ErrT::DbMigrationFailure)
            .ctx(format!("migrations failed: {e}"))
    })?;
    let state = models::AppState { db };

    let app = routes::get_routes().with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    axum::serve(
        tokio::net::TcpListener::bind(&addr)
            .await
            .inspect(|_| {
                println!("listening on {}", addr);
            })
            .unwrap_or_else(|e| panic!("Can bind to address {addr} ({e})")),
        app.into_make_service(),
    )
    .await
    .unwrap();

    Ok(())
}
