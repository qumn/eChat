use std::sync::Arc;

use axum::Extension;
use dashmap::DashMap;
use eChat::err::Error;
use http::api_router;
use persistent::get_pool;
use sqlx::MySqlPool;
use tokio::sync::mpsc::Sender;
mod auth;
mod http;
mod persistent;
mod modles;

pub struct Msg{

}

#[derive(Clone)]
pub struct ApiContext {
    pub db: Arc<MySqlPool>,
    pub sender_map: DashMap<u64, Sender<Msg>>
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().unwrap(); // load .env file
    // Initialize the logger 
    tracing_subscriber::fmt::init();
    //tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();
    let pool = get_pool().await?;
    let ctx = ApiContext { db: Arc::new(pool), sender_map: DashMap::new() };
    let app = api_router(&ctx).layer(Extension(ctx));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("error running HTTP server");
    Ok(())
}
