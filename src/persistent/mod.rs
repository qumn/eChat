use eChat::err::Ok;
use eChat::err::Result;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
mod user;
mod friend;

pub use user::UserManage;


// get connect pool
pub async fn get_pool() -> Result<Pool<MySql>> {
    let url = get_url();
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&url)
        .await?;
    Ok(pool)
}

pub fn get_url() -> String {
    dotenvy::var("DATABASE_URL").expect("please set the env variable of DATABASE_URL")
}
