use std::sync::Arc;

use eChat::err::Result;
use sqlx::{MySql, Pool};

use crate::modles::message::*;

#[derive(Clone, Debug)]
pub struct MessageManage {
    db: Arc<Pool<MySql>>,
}

impl MessageManage {
    pub fn new(db: Arc<Pool<MySql>>) -> Self {
        MessageManage { db }
    }
}

impl MessageManage {
    pub async fn create_message(&self, message: Message) -> Result<()> {
        sqlx::query!(
            r#" 
            insert into
                message(sender_uid,
                    receiver_type, 
                    receiver_id, 
                    content, 
                    create_time) 
            values (?, ?, ?,?, ?)
            "#,
            message.sender_uid,
            message.receiver_type,
            message.receiver_id,
            message.content,
            message.create_time
        )
        .execute(&*self.db)
        .await?;
        Ok(())
    }

    pub async fn get_message_by_receiver_id(
        &self,
        id: u64,
        mtype: ReceiverType,
    ) -> Result<Vec<Message>> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT
                mid,
                content, 
                sender_uid, 
                receiver_id, 
                create_time, 
                receiver_type as "receiver_type: ReceiverType" 
            from
                message
            where
                receiver_id = ? and receiver_type = ?
            "#,
            id,
            mtype
        )
        .fetch_all(&*self.db)
        .await?;
        Ok(messages)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::persistent::get_pool;

    #[tokio::test]
    async fn create_message_should_work() -> Result<()> {
        let pool = Arc::new(get_pool().await?);
        let message_manage = MessageManage::new(pool);
        message_manage
            .create_message(Message {
                mid: 0,
                content: "world".to_string(),
                sender_uid: 1,
                receiver_id: 1029,
                create_time: chrono::Utc::now().naive_utc(),
                receiver_type: ReceiverType::User,
            })
            .await?;
        Ok(())
    }
    #[tokio::test]
    async fn get_message_should_work() -> Result<()> {
        let pool = Arc::new(get_pool().await?);
        let message_manage = MessageManage::new(pool);
        let messages = message_manage
            .get_message_by_receiver_id(1029, ReceiverType::User)
            .await?;
        println!("get_message_should_work, messages: {:?}", messages);
        Ok(())
    }
}
