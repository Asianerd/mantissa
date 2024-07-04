use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::user::User;

#[derive(FromRow, Clone, Debug)]
pub struct Inventory {
    // misleading name, actually refers to a singular item in the inventory
    pub id: i64,
    pub volume: i64,
    pub user_id: i64,
    pub stock_id: i64
}
impl Inventory {
    pub async fn fetch_inventory(user: &User, db: &Pool<Sqlite>) -> Vec<Inventory> {
        sqlx::query_as::<_, Inventory>("select * from inventory where user_id = $1")
            .bind(user.id)
            .fetch_all(db)
            .await
            .unwrap()
    }
}