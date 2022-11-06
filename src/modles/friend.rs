use serde::Serialize;

#[derive(Serialize, Debug)]
struct Friend {
    fid: u64,
    user_id: u64,
    friend_id: u64,
    status: FriendStatus,
}

#[derive(Clone, Debug, PartialEq, Serialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
enum FriendStatus {
    Pending,
    Agree,
    Refused,
}