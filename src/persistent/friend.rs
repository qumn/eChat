use std::sync::Arc;

use eChat::err::{Result, ResultExt};
use sqlx::{MySql, Pool};
use tracing::instrument;

use crate::modles::friend::*;
use crate::modles::user::ViewUser;

#[derive(Clone, Debug)]
pub struct FriendManage {
    db: Arc<Pool<MySql>>,
}

impl FriendManage {
    pub fn new(db: Arc<Pool<MySql>>) -> FriendManage {
        FriendManage { db }
    }
}

impl FriendManage {
    #[instrument]
    pub async fn create_friend(&self, uid: u64, friend_id: u64) -> Result<()> {
        let mut tx = self.db.begin().await?; // sqlx how to rollback

        sqlx::query!(
            "insert into friend (uid, friend_id, status) values (?, ?, ?)",
            uid,
            friend_id,
            FriendStatus::Agree
        )
        .execute(&mut tx)
        .await
        .on_duplicated(format!("已经申请添加好友了"))?;

        sqlx::query!(
            "insert into friend (uid, friend_id, status) values (?, ?, ?)",
            friend_id,
            uid,
            FriendStatus::Pending
        )
        .execute(&mut tx)
        .await
        .on_duplicated(format!("已经申请添加好友了"))?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn agree_friend(&self, uid: u64, friend_id: u64) -> Result<()> {
        self.change_status(uid, friend_id, FriendStatus::Agree)
            .await
    }

    pub async fn refuse_friend(&self, uid: u64, friend_id: u64) -> Result<()> {
        self.change_status(uid, friend_id, FriendStatus::Refused)
            .await
    }

    /// status only be changed from Pending to Agree or Refused
    #[instrument]
    async fn change_status(&self, uid: u64, friend_id: u64, status: FriendStatus) -> Result<()> {
        let mut tx = self.db.begin().await?;
        sqlx::query!(
            r#"update 
                friend 
            set
                status = ? 
            where 
                uid = ? and friend_id = ? and status = ?
            "#,
            status,
            uid,
            friend_id,
            FriendStatus::Pending
        )
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(())
    }

    #[instrument]
    pub async fn get_friends(&self, uid: u64) -> Result<Vec<ViewUser>> {
        let friends = sqlx::query_as!(
            ViewUser,
            r#"
            with friends as 
            (
            SELECT 
                f1.uid, f1.friend_id
            FROM 
                friend f1 join friend f2 on f1.uid = f2.friend_id and f1.friend_id = f2.uid
            where f1.status = ? and f2.status = ?
            )
            
            select 
                u.uid, u.mail, u.username, u.create_time 
            from
                friends  f
            join 
                user u on u.uid = f.friend_id
            where 
                f.uid = ?;
            "#,
            FriendStatus::Agree,
            FriendStatus::Agree,
            uid
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(friends)
    }
}
