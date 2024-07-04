use rocket::State;
use sqlx::{Pool, Sqlite};

use crate::user;

#[get("/")]
pub async fn debug_users(pool: &State<Pool<Sqlite>>) -> String {
    format!("{:?}", sqlx::query_as::<_, user::User>("select * from user;")
        .fetch_all(pool.inner())
        .await
        .unwrap()
    )
}
