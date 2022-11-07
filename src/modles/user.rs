use chrono::NaiveDateTime;
use eChat::utils;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct User {
    pub uid: u64,
    pub username: String,
    pub mail: String,
    pub password: String,
    pub salt: String,
    pub create_time: NaiveDateTime,
}

#[derive(Debug)]
pub struct UpdateUser {
    pub uid: u64,
    pub username: Option<String>,
    pub mail: Option<String>,
    pub password: Option<String>,
    pub salt: Option<String>,
    pub create_time: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct CreateUser {
    pub username: String,
    pub mail: String,
    pub password: String,
}

impl Into<User> for CreateUser {
    fn into(self) -> User {
        let (password, salt) = utils::encyption(&self.password).unwrap();
        User {
            uid: 0,
            username: self.username,
            password,
            mail: self.mail,
            salt,
            create_time: chrono::Local::now().naive_local(),
        }
    }
}


#[derive(Serialize)]
pub struct ViewUser {
    pub uid: u64,
    pub username: String,
    pub mail: String,
    pub create_time: NaiveDateTime,
}

impl From<User> for ViewUser {
    fn from(user: User) -> Self {
        ViewUser {
            uid: user.uid,
            username: user.username,
            mail: user.mail,
            create_time: user.create_time,
        }
    }
}
