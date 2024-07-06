use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};
use strum_macros::{Display, EnumString, IntoStaticStr};

use crate::utils;

const LISTING_ID_MAX: i64 = 281474976710656i64; // 16^12

#[derive(FromRow, Clone, Debug, Serialize, Deserialize)]
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

    pub async fn generate_listing_id(db: &Pool<Sqlite>) -> i64 {
        // might be expensive?
        let existing_ids = sqlx::query_as::<_, utils::ValueInt>("select id from listing;")
            .fetch_all(db)
            .await
            .unwrap()
            .iter()
            .map(|x| x.0)
            .collect::<Vec<i64>>();

        for _ in 0..1000 {
            let candidate = utils::async_rng_int_large(LISTING_ID_MAX);

            if existing_ids.contains(&candidate) {
                continue;
            }
            return candidate;
        }

        existing_ids.iter().max().unwrap() + 1
    }

    pub async fn create_listing(db: &Pool<Sqlite>, value: f64, volume: f64, listing_type: ListingType, user_id: i64, stock_id: i64) -> ListingError {
        // take from user
        // 4 sql calls to create a listing :skull:

        if volume <= 0f64 {
            return ListingError::NoVolume;
        }

        if listing_type == ListingType::Sell {
            let current: Option<utils::Value> = sqlx::query_as("select volume from inventory where user_id = $1 and stock_id = $2;").bind(user_id).bind(stock_id)
                .fetch_optional(db)
                .await.unwrap();
            println!("current : {current:?}\tvolume : {volume}");
            if current.is_none() {
                return ListingError::NotEnoughOwned;
            }
            let current = current.unwrap().0;
            if current < volume {
                return ListingError::NotEnoughOwned;
            }
            let balance = utils::slightly_round_floats(current - volume, None);
            sqlx::query("update inventory set volume = $1 where user_id = $2 and stock_id = $3;").bind(balance).bind(user_id).bind(stock_id)
                .execute(db)
                .await.unwrap();
        }
        // add listing
        sqlx::query("insert into listing values ($1, $2, $3, $4, $5, $6, $7, $8, $9);").bind(Listing::generate_listing_id(db).await).bind(value).bind(volume).bind(utils::get_time() as i64).bind(0).bind(format!("{}", Status::Pending)).bind(format!("{}", listing_type)).bind(user_id).bind(stock_id)
            .execute(db)
            .await.unwrap();
        ListingError::Success
    }
}

#[derive(EnumString, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Display, IntoStaticStr)]
pub enum Status {
    Pending,
    Resolved
}

#[derive(EnumString, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Display, IntoStaticStr)]
// #[strum(serialize_all="lowercase")]
pub enum ListingType {
    Buy, // looking for someone to buy this listing
    Sell // looking for someone to sell this listing
}

pub enum ListingError {
    Success,

    NoVolume, // when volume is 0.0

    NotEnoughOwned
}
