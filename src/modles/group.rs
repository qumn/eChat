use chrono::NaiveDateTime;
use serde::Deserialize;


#[derive(Debug)]
pub struct Group {
    pub gid: u64,
    pub owner: u64,
    pub name: String,
    pub create_time: NaiveDateTime
}
#[derive(Deserialize)]
pub struct CreateGroup {
    pub name: String,
}

#[derive(Debug, sqlx::Type)]
#[repr(u8)]
pub enum GroupStatus {
    Pending = 0,
    Agree = 1,
    Refused = 2,
}

#[derive(Deserialize)]
pub struct JoinGroup{
    pub gid: u64
}

#[derive(Deserialize)]
pub struct AgreeGroup {
    pub gid: u64,
    pub uid: u64
}