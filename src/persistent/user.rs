use std::sync::Arc;

use eChat::err::{Result, ResultExt};
use sqlx::{MySql, Pool};

use crate::modles::{UpdateUser, User};

#[derive(Clone)]
pub struct UserManage {
    db: Arc<Pool<MySql>>,
}

impl UserManage {
    pub fn new(db: Arc<Pool<MySql>>) -> UserManage {
        UserManage { db }
    }
}

impl UserManage {
    pub async fn create_user(&self, user: User) -> Result<u64> {
        // verify username and email is unique
        let mut tx = self.db.begin().await?;
        let id = sqlx::query!(
            "insert into user (username, mail, password, salt, create_time) values (?, ?, ?, ?, ?)",
            user.username,
            user.mail,
            user.password,
            user.salt,
            user.create_time
        )
        .execute(&mut tx)
        .await
        .on_duplicated(format!("用户名或者邮箱已经存在"))?
        .last_insert_id();
        Ok(id)
    }

    pub async fn update_user(&self, user: UpdateUser) -> Result<()> {
        sqlx::query!(
            "update user set 
                username = coalesce(?, user.username), 
                mail = coalesce(?, user.mail),
                password = coalesce(?, user.password),
                salt = coalesce(?, user.salt),
                create_time = coalesce(?, user.create_time)
                where uid = ?",
            user.username,
            user.mail,
            user.password,
            user.salt,
            user.create_time,
            user.uid
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }

    pub async fn delete_user(&self, id: u64) -> Result<()> {
        sqlx::query!("delete from user where uid = ?", id)
            .execute(&*self.db)
            .await?;
        Ok(())
    }

    pub async fn get_user(&self, id: u64) -> Result<User> {
        let user = sqlx::query_as!(User, "select * from user where uid = ?", id)
            .fetch_one(&*self.db)
            .await?;
        Ok(user)
    }

    pub async fn get_user_by_username(&self, name: &str) -> Result<User> {
        let user = sqlx::query_as!(User, "select * from user where username = ?", name)
            .fetch_one(&*self.db)
            .await?;
        Ok(user)
    }
}

#[cfg(test)]
mod test {
    use crate::persistent::get_pool;

    use super::*;

    #[tokio::test]
    async fn create_user_should_work() {
        let pool = get_pool().await.unwrap();
        let user_manage = UserManage::new(Arc::new(pool));
        let user = User {
            uid: 0,
            username: "test1".to_string(),
            mail: "test".to_string(),
            password: "test".to_string(),
            salt: "test".to_string(),
            create_time: chrono::Local::now().naive_local(),
        };
        user_manage.create_user(user).await.unwrap();
    }
}
