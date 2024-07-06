use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{listing::Listing, utils};

pub struct DetailedStock {
    // object that gets sent to frontend
    pub stock: Stock,
    pub listings: Vec<Listing>,
}
impl DetailedStock {
    pub async fn convert(db: &Pool<Sqlite>, stock: &Stock) -> DetailedStock {
        DetailedStock {
            stock: stock.clone(),
            listings: Listing::find_listings(stock.id, db).await
        }
    }
}

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
pub struct Stock {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub std_dev: f64,

    pub recent: f64, // most recent buy is the most recent sell too (duh)
}
impl Stock {
    pub async fn get_all_stocks(db: &Pool<Sqlite>) -> Vec<Stock> {
        vec![]
    }

    pub async fn get_stock(db: &Pool<Sqlite>, stock_id: i64) -> Option<Stock> {
        sqlx::query_as("select * from stock where id = $1")
            .bind(stock_id)
            .fetch_optional(db)
            .await
            .unwrap()
    }
}

#[get("/")]
pub async fn fetch_highest_valued(db: &State<Pool<Sqlite>>) -> String {
    let mut result = Stock::get_all_stocks(db.inner()).await;
    result.sort_by(|a, b| a.recent.partial_cmp(&b.recent).unwrap());
    utils::parse_response(Ok(serde_json::to_string(&result.iter().rev().take(5).collect::<Vec<&Stock>>()).unwrap()))
}

// #[get("/")]
// pub async fn fetch_highest_valued(db: &State<Pool<Sqlite>>) -> String {
//     let mut result = Stock::get_all_stocks(db.inner()).await;
//     result.sort_by(|a, b| a.recent.partial_cmp(&b.recent).unwrap());
//     utils::parse_response(Ok(serde_json::to_string(&result.iter().rev().take(5).collect::<Vec<&Stock>>()).unwrap()))
// }
