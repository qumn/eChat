use axum::{routing::get, Extension, Json, Router};
use eChat::err::Error;
use serde::Serialize;
use sqlx::MySqlPool;

use crate::{auth::AuthUser, ApiContext};

#[derive(Serialize, Debug)]
struct Firend {
    fid: u64,
    user_id: u64,
    firend_id: u64,
    state: FirendState,
}

#[derive(Clone, Debug, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
enum FirendState {
    Pending,
    Agree,
    Refused,
}

pub fn router() -> Router {
    Router::new().route("api/firend", get(get_firends))
}

pub async fn get_firends(
    auth_user: AuthUser,
    Extension(ctx): Extension<ApiContext>,
) -> Result<Json<Vec<Firend>>, Error> {
    let firends = sqlx::query_as!(
        Firend,
        "select fid, user_id, firend_id, state as 'state!: FirendState'  from firend where user_id = ?",
        auth_user.uid
    )
    .fetch_all(&ctx.db)
    .await?;
    Ok(Json(firends))
}

pub async fn add_firends(auth_user: AuthUser, fid: u64, Extension(ctx): Extension<ApiContext>) -> Result<(), Error> {
    // add firend
    sqlx::query!(
        "insert into firend (user_id, firend_id, state) values (?, ?, ?)",
        auth_user.uid,
        fid,
        FirendState::Pending
    )
    .fetch_one(&ctx.db)
    .await?;
    sqlx::query!(

        fid,
        auth_user.uid,
        FirendState::Pending
    )
    .fetch_one(&ctx.db)
    .await?;
    Ok(())
}

pub async fn get_firends2(uid: u64, pool: &MySqlPool) -> Result<Json<Vec<Firend>>, Error> {
    let firends = sqlx::query_as!(
        Firend,
        "select fid, user_id, firend_id, state as 'state!: FirendState'  from firend where user_id = ?",
        uid
    )
    .fetch_all(pool)
    .await?;
    Ok(Json(firends))
}

#[cfg(test)]
mod test {
    use super::*;
    use eChat::err::Error;
    use sqlx::mysql::MySqlPoolOptions;

    #[tokio::test]
    async fn get_fireds_should_work() -> Result<(), Error> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect("mysql://root:root@localhost:3305/echat")
            .await?;
        let r = get_firends2(1, &pool).await?;
        println!("{:?}", r);
        Ok(())
    }
}
