use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, sqlx::Type)]
pub struct Message {
    pub mid: u64,
    pub content: String,
    pub sender_uid: u64,
    pub receiver_id: u64,
    pub create_time: NaiveDateTime,
    pub receiver_type: ReceiverType,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Msg {
    pub receiver_type: ReceiverType,
    pub receiver_id: u64,
    pub content: String,
}
impl Msg {
    pub fn new(content: &str) -> Self {
        Msg {
            receiver_type: ReceiverType::User,
            receiver_id: 0,
            content: content.into(),
        }
    }
    pub fn to_message(&self, sender_uid: u64) -> Message {
        Message {
            mid: 0,
            content: self.content.clone(),
            receiver_id: self.receiver_id,
            receiver_type: self.receiver_type.clone(),
            sender_uid,
            create_time: Local::now().naive_local(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug, sqlx::Type)]
#[repr(i8)]
pub enum ReceiverType {
    User = 0,
    Group = 1,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn serialize_should_work() {
        let msg = Msg::new("hello");
        let msg = serde_json::to_string(&msg).unwrap();
        println!("{}", msg);
    }

    #[test]
    fn deserialize_should_work() {
        let msg = r#"{"receiver_type":0, "receiver_id":"1029", "content":"111"}"#;
        let msg: Msg = serde_json::from_str(msg).unwrap();
        println!("{:?}", msg);
    }
}
