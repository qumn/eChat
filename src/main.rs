use axum::{Extension, Router};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
mod user;
mod Auth;

#[derive(Clone)]
pub struct ApiContext {
    pub db: MySqlPool,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect("mysql://root:root@localhost:3305/echat?serverTimezone=Asia/Shanghai")
        .await?;
    let ctx = ApiContext { db: pool };
    let app = api_router().layer(Extension(ctx));
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("error running HTTP server");
    Ok(())
}

fn api_router() -> Router {
    // This is the order that the modules were authored in.
    user::router()
}
