use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};
use strum_macros::EnumString;

use crate::utils::{self, generate_name};

#[derive(FromRow, Clone)]
pub struct Listing {
    pub id: i64,
    pub value: f64,
    pub volume: f64,
    pub start_time: i64,
    pub end_time: i64,
    status: String,
    listing_type: String,
    pub user_id: i64,
    pub stock_id: i64
}
impl Listing {
    pub fn status(&self) -> Status {
        Status::from_str(&self.status).unwrap()
    }

    pub fn listing_type(&self) -> ListingType {
        ListingType::from_str(&self.listing_type).unwrap()
    }

    pub async fn find_listings(stock_id: i64, db: &Pool<Sqlite>) -> Vec<Listing> {
        sqlx::query_as::<_, Listing>("select * from listing where stock_id = $1")
            .bind(stock_id)
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn standard_deviation(stock_id: i64, db: &Pool<Sqlite>) -> (Option<f64>, Option<f64>) { // (buy price, sell price)
        let listings = Listing::find_listings(stock_id, db).await;
        (
            utils::std_deviation(&listings.iter().filter(|x| x.listing_type() == ListingType::Buy).map(|x| x.value).collect::<Vec<f64>>()),
            utils::std_deviation(&listings.iter().filter(|x| x.listing_type() == ListingType::Sell).map(|x| x.value).collect::<Vec<f64>>())
        )
    }

    pub async fn generate_listing_id() -> i64 {
        0
    }

    //(id int primary key, value decimal, volume decimal, start_time int, end_time int, status varchar, listing_type varchar, user_id int, stock_id int)
    pub async fn create_listing(db: &Pool<Sqlite>, value: f64, volume: f64, listing_type: ListingType, user_id: i64, stock_id: i64) {
        sqlx::query("insert into listing values ($1, $2, $3, $4, $5, $6, $7, $8, $9);")
            .bind(Listing::generate_listing_id().await)
            .bind(value)
            .bind(volume)
            .bind(utils::get_time() as i64)
            .bind(0)
            .bind(serde_json::to_string(&Status::Pending).unwrap())
            .bind(serde_json::to_string(&listing_type).unwrap())
            .bind(user_id)
            .bind(stock_id)
            .execute(db)
            .await
            .unwrap();
    }
}

#[derive(EnumString, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Status {
    Pending,
    Resolved
}

#[derive(EnumString, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
// #[strum(serialize_all="lowercase")]
pub enum ListingType {
    Buy, // looking for someone to buy this listing
    Sell // looking for someone to sell this listing
}
