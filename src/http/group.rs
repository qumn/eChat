use axum::{Router, Extension, routing::post, Json};

use crate::auth::{AuthUser, self};
use crate::{persistent::GroupManage, ApiContext};
use crate::modles::group::*;
use eChat::err::Result;

pub fn router(ctx: &ApiContext) -> Router{
    let group_manage = GroupManage::new(ctx.db.clone());
    Router::new()
        .route("/api/groups", post(create_group))
        .layer(Extension(group_manage))
}

pub async fn create_group(
    auth_user: AuthUser,
    create_group: Json<CreateGroup>,
    Extension(group_manage): Extension<GroupManage>,
) -> Result<()> {
    let group = Group {
        gid: 0,
        owner: auth_user.uid,
        name: create_group.name.clone(),
        create_time: chrono::Local::now().naive_local(),
    };
    group_manage.create_group(group).await?;
    Ok(())
}