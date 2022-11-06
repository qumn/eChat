use axum::routing::{get, post};
use axum::Router;
use axum::{
    headers::{authorization::Bearer, Authorization},
    Extension, Json, TypedHeader,
};
use axum_macros::debug_handler;
use eChat::err::Error;
use eChat::utils;

use crate::auth::AuthUser;
use crate::modles::*;
use crate::persistent::UserManage;
use crate::ApiContext;

pub fn router(ctx: &ApiContext) -> Router {
    Router::new()
        .route("/api/users/login", post(login))
        .route("/api/users", get(get_current_user).post(create_user))
        .layer(Extension(UserManage::new(ctx.db.clone())))
}

#[debug_handler]
async fn login(
    Json(login_user): Json<LoginUser>,
    Extension(user_manage): Extension<UserManage>,
) -> Result<TypedHeader<Authorization<Bearer>>, Error> {
    let user = user_manage
        .get_user_by_username(&login_user.username)
        .await?;

    if !utils::verify(&login_user.password, &user.password, &user.salt) {
        return Err(Error::unprocessable_entity([("msg", "用户名或者密码错误")]));
    }

    let auth_user = AuthUser::new(user.uid, user.username, user.mail);
    let token = AuthUser::encode(&auth_user);

    Ok(TypedHeader(Authorization::bearer(&token).unwrap()))
}

async fn get_current_user(
    auth_user: AuthUser,
    Extension(user_manage): Extension<UserManage>,
) -> Result<Json<ViewUser>, Error> {
    let user = user_manage.get_user(auth_user.uid).await?;
    Ok(Json(user.into()))
}

async fn create_user(
    Json(user): Json<CreateUser>,
    Extension(user_manage): Extension<UserManage>,
) -> Result<String, Error> {
    user_manage.create_user(user.into()).await?;
    Ok("注册成功".to_string())
}
