use std::sync::Arc;

use crate::modles::group::*;
use eChat::err::{Error, Result, ResultExt};
use sqlx::{MySql, Pool};

#[derive(Clone)]
pub struct GroupManage {
    db: Arc<Pool<MySql>>,
}

impl GroupManage {
    pub fn new(db: Arc<Pool<MySql>>) -> Self {
        GroupManage { db }
    }
}

impl GroupManage {
    pub async fn create_group(&self, group: Group) -> Result<()> {
        sqlx::query!(
            "insert into `group` (owner,name, create_time) values (?, ?, ?)",
            group.owner,
            group.name,
            group.create_time
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }

    pub async fn join(&self, uid: u64, gid: u64) -> Result<()> {
        sqlx::query!(
            "insert into group_user (uid, gid, status) values (?, ?, ?)",
            uid,
            gid,
            GroupStatus::Pending
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }
    /// user_id the id of current user
    pub async fn agree(&self, user_id: u64,  uid: u64, gid: u64) -> Result<()> {
        let mut tx = self.db.begin().await?;
        // check owner
        let owner_id = sqlx::query_scalar!("select owner from `group` where gid = ?", gid)
            .fetch_optional(&mut tx)
            .await?;
        if owner_id.is_none() || owner_id.unwrap() != user_id {
            return Err(Error::unprocessable_entity([("msg", "没有权限同意")]));
        }

        sqlx::query!(
            r#"
            update
                group_user set status = ? 
            where 
                uid = ? and gid = ?
            "#,
            GroupStatus::Agree,
            uid,
            gid,
        )
        .execute(&mut tx)
        .await
        .on_duplicated("已经申请过了".into())?;

        tx.commit().await?;
        Ok(())
    }

}
