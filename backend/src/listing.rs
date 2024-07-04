use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};
use strum_macros::EnumString;

use crate::utils;

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
