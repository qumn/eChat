use std::sync::Arc;

use axum::Extension;
use eChat::err::Error;
use http::api_router;
use persistent::get_pool;
use sqlx::MySqlPool;
mod auth;
// mod firend;
mod http;
mod persistent;
mod modles;

#[derive(Clone)]
pub struct ApiContext {
    pub db: Arc<MySqlPool>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let pool = get_pool().await?;
    let ctx = ApiContext { db: Arc::new(pool) };
    let app = api_router(&ctx).layer(Extension(ctx));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("error running HTTP server");
    Ok(())
}
