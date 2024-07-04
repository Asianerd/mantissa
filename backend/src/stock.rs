use sqlx::prelude::FromRow;

#[derive(FromRow, Clone, Debug)]
pub struct Stock {
    pub id: i64,
    pub code: String,
    pub name: String
}

