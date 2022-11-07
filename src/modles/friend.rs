use serde::{Serialize, Deserialize};

#[derive(Serialize, Debug)]
pub struct Friend {
    fid: u64,
    uid: u64,
    friend_id: u64,
    status: FriendStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, sqlx::Type)]
#[repr(i8)]
pub enum FriendStatus {
    Pending = 0,
    Agree = 1,
    Refused = 2,
}

#[derive(Deserialize)]
pub struct AddFriend {
    pub friend_id: u64
}