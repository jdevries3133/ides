//! A GPT-powered calorie counter.

#![feature(try_trait_v2)]

use anyhow::Result;
use dotenvy::dotenv;
use std::net::SocketAddr;

mod auth;
mod book;
mod components;
mod db_ops;
mod error;
mod htmx;
mod middleware;
mod models;
mod prelude;
mod routes;
mod r#static;

#[tokio::main]
async fn main() -> Result<()> {
    println!("bool: {}", bool::default());
    dotenv().ok();

    let db = db_ops::create_pg_pool().await?;
    sqlx::migrate!().run(&db).await?;
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
