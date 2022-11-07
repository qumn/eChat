use axum::routing::{post};
use axum::Router;
use axum::{Extension, Json};
use eChat::err::Result;

use crate::auth::AuthUser;
use crate::modles::friend::*;
use crate::modles::user::ViewUser;
use crate::persistent::FriendManage;
use crate::ApiContext;

pub fn router(ctx: &ApiContext) -> Router {
    Router::new()
        .route("/api/friends", post(add_friend).get(get_friend))
        .route("/api/friends/agree", post(agree_friend))
        .route("/api/friends/refuse", post(refuse_friend))
        .layer(Extension(FriendManage::new(ctx.db.clone())))
}

async fn add_friend(
    Json(friend): Json<AddFriend>,
    auth_user: AuthUser,
    Extension(friend_manage): Extension<FriendManage>,
) -> Result<Json<()>> {
    let uid = auth_user.uid;
    friend_manage.create_friend(uid, friend.friend_id).await?;
    Ok(Json(()))
}

async fn get_friend(
    auth_user: AuthUser,
    Extension(friend_manage): Extension<FriendManage>,
) -> Result<Json<Vec<ViewUser>>> {
    let uid = auth_user.uid;
    let friends: Vec<ViewUser> = friend_manage.get_friends(uid).await?;
    Ok(Json(friends))
}

async fn agree_friend(
    Json(friend): Json<AddFriend>,
    auth_user: AuthUser,
    Extension(friend_manage): Extension<FriendManage>,
) -> Result<Json<()>> {
    let uid = auth_user.uid;
    friend_manage.agree_friend(uid, friend.friend_id).await?;
    Ok(Json(()))
}

async fn refuse_friend(
    Json(friend): Json<AddFriend>,
    auth_user: AuthUser,
    Extension(friend_manage): Extension<FriendManage>,
) -> Result<Json<()>> {
    let uid = auth_user.uid;
    friend_manage.refuse_friend(uid, friend.friend_id).await?;
    Ok(Json(()))
}


