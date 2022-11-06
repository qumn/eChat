use axum::Router;

use crate::ApiContext;
mod user;


pub fn api_router(ctx: &ApiContext) -> Router {
    // This is the order that the modules were authored in.
    user::router(ctx)
}