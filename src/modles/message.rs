use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, Serialize, sqlx::Type)]
pub struct Message {
    pub mid: u64,
    pub content: String,
    pub sender_uid: u64,
    pub receiver_id: u64,
    pub create_time: NaiveDateTime,
    pub receiver_type: ReceiverType,
}

#[derive(Serialize, Clone, PartialEq, Debug, sqlx::Type)]
#[repr(i8)]
pub enum ReceiverType {
    User = 0,
    Group = 1
}
