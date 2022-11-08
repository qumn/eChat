 mod user;
mod friend;
mod msg;
mod group;
mod message;

use axum::Router;

use crate::ApiContext;


pub fn api_router(ctx: &ApiContext) -> Router {
    // This is the order that the modules were authored in.
    user::router(ctx)
        .merge(friend::router(ctx))
        .merge(group::router(ctx))
        .merge(message::router(ctx))
}