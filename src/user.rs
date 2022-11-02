use crate::{ApiContext, Auth::AuthUser};
use axum::{
    headers::{authorization::Bearer, Authorization},
    routing::post,
    Extension, Json, Router, TypedHeader,
};
use chrono::NaiveDateTime;
use eChat::err::Error;
use eChat::utils;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

pub fn router() -> Router {
    Router::new()
        .route("/api/users/login", post(login))
        .route("/api/users", post(create_user).get(get_current_user))
}

#[derive(Debug)]
pub struct User {
    pub uid: u64,
    pub username: String,
    pub mail: String,
    pub password: String,
    pub salt: String,
    pub create_time: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub mail: String,
    pub password: String,
}
#[derive(Serialize)]
pub struct ViewUser {
    pub uid: u64,
    pub username: String,
    pub mail: String,
    pub create_time: NaiveDateTime,
}

impl From<User> for ViewUser {
    fn from(user: User) -> Self {
        ViewUser {
            uid: user.uid,
            username: user.username,
            mail: user.mail,
            create_time: user.create_time,
        }
    }
}

async fn login(
    Json(login_user): Json<LoginUser>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<TypedHeader<Authorization<Bearer>>, Error> {
    let user = sqlx::query_as!(
        User,
        "select * from user where username = ?",
        login_user.username
    )
    .fetch_one(&ctx.db)
    .await?;
    if !utils::verify(&login_user.password, &user.password, &user.salt) {
        return Err(Error::unprocessable_entity([("msg", "用户名或者密码错误")]));
    }
    let auth_user = AuthUser::new(user.uid, user.username, user.mail);
    let token = AuthUser::encode(&auth_user);

    Ok(TypedHeader(Authorization::bearer(&token).unwrap()))
}
async fn create_user(
    Json(user): Json<CreateUser>,
    Extension(ctx): Extension<ApiContext>,
) -> Result<String, Error> {
    register(user, &ctx.db).await?;
    Ok("注册成功".to_string())
}

async fn register(user: CreateUser, pool: &MySqlPool) -> Result<u64, Error> {
    let (password, salt) = utils::encyption(&user.password).unwrap();
    // verify the username unique
    let exist = sqlx::query_scalar!("select uid from user where username = ?", user.username)
        .fetch_one(pool)
        .await
        .is_ok();
    if exist {
        return Err(Error::unprocessable_entity([("msg", "用户名已经存在")]));
    }
    let exist = sqlx::query_scalar!("select uid from user where mail = ?", user.mail)
        .fetch_one(pool)
        .await
        .is_ok();
    if exist {
        return Err(Error::unprocessable_entity([("msg", "邮箱已经存在")]));
    }
    let create_time = chrono::Local::now().naive_local();
    let id = sqlx::query!(
        "insert into user(username, password, mail, salt, create_time) values(?, ?, ?, ?, ?)",
        user.username,
        password,
        user.mail,
        salt,
        create_time
    )
    .execute(pool)
    .await?
    .last_insert_id();
    Ok(id)
}

async fn get_current_user(
    auth_user: AuthUser,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Json<ViewUser>, Error> {
    let user = sqlx::query_as!(
        ViewUser,
        "select username, uid, mail, create_time from user where uid = ?",
        auth_user.uid
    )
    .fetch_one(&ctx.db)
    .await?;
    Ok(Json(user))
}

async fn _delete(uid: u64, pool: &MySqlPool) {
    sqlx::query!("delete from user where uid = ?", uid)
        .execute(pool)
        .await
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    async fn register_should_work() -> Result<(), Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:root@localhost:3305/echat")
            .await?;
        let user = CreateUser {
            username: "qumn4".into(),
            password: "123".into(),
            mail: "1234@qq.com".into(),
        };
        let _uid = register(user.clone(), &pool).await?;
        Ok(())
    }
}
