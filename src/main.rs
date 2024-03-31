mod contacts;
mod handler;
mod state;

use axum::routing::{delete, get, post, put};
use axum::Router;
use dotenv::dotenv;
use fred::prelude::*;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use crate::state::StateInternal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let pg_url = env::var("DATABASE_URL")?;
    let redis_url = env::var("REDIS_URL")?;
    let server_url = "0.0.0.0:3000";

    let dbpool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&pg_url)
        .await
        .expect("Database connection. Fail to connect");

    let redis_pool_size = 8;
    let redis_config = RedisConfig::from_url(&redis_url)?;

    sqlx::migrate!().run(&dbpool).await?;

    let redis_pool = Builder::from_config(redis_config)
        .with_performance_config(|config| {
            config.auto_pipeline = true;
        })
        .set_policy(ReconnectPolicy::new_exponential(0, 100, 30_000, 2))
        .build_pool(redis_pool_size)
        .expect("Redis connection. Error creating pool.");

    redis_pool
        .init()
        .await
        .expect("Redis connection. Fail to connect");

    let app_state = Arc::new(StateInternal::new(dbpool, redis_pool));

    let app = Router::new()
        .route("/v1/contacts", get(handler::contacts_list))
        .route("/v1/contacts", post(handler::contacts_create))
        .route("/v1/contacts/:id", get(handler::contacts_read))
        .route("/v1/contacts/:id", put(handler::contacts_update))
        .route("/v1/contacts/:id", delete(handler::contacts_delete))
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&server_url).await?;
    println!("Server running at {}", &server_url);

    axum::serve(listener, app).await?;

    Ok(())
}
