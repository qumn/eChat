use axum::{routing::post, Extension, Json, Router};

use crate::auth::AuthUser;
use crate::modles::group::*;
use crate::{persistent::GroupManage, ApiContext};
use eChat::err::Result;

pub fn router(ctx: &ApiContext) -> Router {
    let group_manage = GroupManage::new(ctx.db.clone());
    Router::new()
        .route("/api/groups", post(create_group))
        .route("/api/groups/join", post(join_group))
        .route("/api/groups/agree", post(agree))
        .layer(Extension(group_manage))
}

pub async fn create_group(
    auth_user: AuthUser,
    create_group: Json<CreateGroup>,
    Extension(group_manage): Extension<GroupManage>,
) -> Result<()> {
    let group = Group {
        gid: 0, // a placeholder, the real gid will be automatically generated
        owner: auth_user.uid,
        name: create_group.name.clone(),
        create_time: chrono::Local::now().naive_local(),
    };
    group_manage.create_group(group).await?;
    Ok(())
}

pub async fn join_group(
    auth_user: AuthUser,
    Json(join_group): Json<JoinGroup>,
    Extension(group_manage): Extension<GroupManage>,
) -> Result<()> {
    group_manage.join(auth_user.uid, join_group.gid).await?;
    Ok(())
}

pub async fn agree(
    auth_user: AuthUser,
    Json(agree_group): Json<AgreeGroup>,
    Extension(group_manage): Extension<GroupManage>,
) -> Result<()> {
    group_manage
        .agree(auth_user.uid, agree_group.uid, agree_group.gid)
        .await?;
    Ok(())
}
