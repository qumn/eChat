mod auth;
mod http;
mod modles;
mod persistent;
mod err;
mod utils;

use std::collections::HashMap;
use std::sync::Arc;

use axum::Extension;
use dashmap::DashMap;
use eChat::err::Error;
use http::api_router;
use modles::message::Msg;
use persistent::get_pool;
use sqlx::MySqlPool;
use tokio::sync::Mutex;
use tokio::sync::mpsc::Sender;
use tower_http::cors::{CorsLayer};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct ApiContext {
    pub db: Arc<MySqlPool>,
    pub active_users: Arc<Mutex<HashMap<u64, Sender<Msg>>>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().unwrap(); // load .env file
                                // Initialize the logger
    tracing_subscriber::fmt::init();
    //tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let pool = get_pool().await?;
    let ctx = ApiContext {
        db: Arc::new(pool),
        active_users: Arc::new(Mutex::new(HashMap::default()))
    };
    let app = api_router(&ctx)
        .layer(Extension(ctx))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("error running HTTP server");
    Ok(())
}
