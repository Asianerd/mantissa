use std::collections::HashMap;

use rand::prelude::*;

use rocket::State;
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{inventory::Inventory, listing::{self, Listing}, login_info::{LoginInformation, LoginResult}, stock, utils};

#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub balance: f64,
    pub aurum_id: i64,
    pub bot: bool
}
impl User {
    pub async fn lookup_user_id(id: &i64, db: &Pool<Sqlite>) -> i64 {
        let result: (i64,) = sqlx::query_as("select count(*) from user where id = $1;")
            .bind(id)
            .fetch_one(db)
            .await
            .unwrap();
        result.0
    }

    async fn simulate_sell_existing(&self, db: &Pool<Sqlite>, item: &Inventory) {
        let available_listings = listing::Listing::find_listings(item.stock_id, db)
            .await
            .iter()
            .filter(|x|
                x.status() == listing::Status::Pending &&
                x.listing_type() == listing::ListingType::Buy
            )
            .map(|x| x.clone())
            .collect::<Vec<listing::Listing>>();
        
        if available_listings.is_empty() {
            return;
        }

        let available_listings = available_listings
            .iter()
            .filter(|x| x.volume <= item.volume)
            .map(|x| x.clone())
            .collect::<Vec<Listing>>();

        let targeted_listing = utils::async_rng_item(&available_listings).clone();

        Listing::create_listing(db, targeted_listing.value, targeted_listing.volume, listing::ListingType::Sell, self.id, targeted_listing.stock_id).await;
    }

    async fn simulate_sell_new(&self, db: &Pool<Sqlite>, item: &Inventory) {
        let mut available_listings: Vec<listing::Listing> = listing::Listing::find_listings(item.stock_id, db).await.iter()
            .filter(|x|
                x.status() == listing::Status::Pending &&
                x.listing_type() == listing::ListingType::Sell
            ).map(|x| x.clone()).collect();
        available_listings.sort_by_key(|x| x.start_time);
        
        let median = utils::median(available_listings.iter().rev().take(10).map(|x| x.value).collect());

        if median.is_none() {
            // no listings are available, create a new one at custom price
            // TODO: base this custom price on previous price history
            Listing::create_listing(
                db,
                utils::round(utils::async_rng_range(1f64, 1000f64), 2),
                utils::round(utils::round(utils::async_rng_range(0.8, 1.0), 2) * item.volume, 2),
                listing::ListingType::Sell,
                self.id,
                item.stock_id
            ).await;
            return;
        }
        let median = median.unwrap();
        let std_dev = stock::Stock::get_stock(db, item.stock_id).await.unwrap().std_dev;

        Listing::create_listing(
            db,
            utils::round(median + (utils::async_rng_range(0.995, 1.005) * std_dev), 2),
            utils::round(utils::round(utils::async_rng_range(0.8, 1.0), 2) * item.volume, 2),
            listing::ListingType::Sell,
            self.id,
            item.stock_id
        ).await;
    }

    async fn simulate_buy_existing() {
        
    }

    async fn simulate_buy_new() {

    }

    async fn simulate(&self, db: &Pool<Sqlite>) {
        if utils::async_rng_bool(0.8) { // 80% to do nothing
            return;
        }

        if utils::async_rng_bool(0.5) { // 50/50 buy/sell
            // sell
            let inventory = Inventory::fetch_inventory(&self, db).await;
            if inventory.is_empty() {
                return;
            }

            let item = utils::async_rng_item(&inventory).clone(); // item to be sold
            if item.volume <= 0f64 {
                return;
            }

            if utils::async_rng_bool(0.7) { // 70% to target existing listing
                self.simulate_sell_existing(db, &item).await; // target existing listing
            } else {
                self.simulate_sell_new(db, &item).await; // make new listing
            }
        } else {
            // buy
        }
    }

    pub async fn simulate_all(db: &Pool<Sqlite>, iterations: usize) {
        // simulate create listings,
        // resolve all listings

        for user in sqlx::query_as::<_, User>("select * from user where bot;").fetch_all(db).await.unwrap() {
            for _ in 0..iterations {
                user.simulate(db).await;
            }
        }
    }
}



#[post("/", data="<login>")]
pub async fn test(db: &State<Pool<Sqlite>>, login: LoginInformation) -> String {
    match login.login(db.inner()).await {
        LoginResult::Success(user_id) => {
            utils::parse_response(Ok(user_id))
        },
        _ => utils::parse_response(Err("huh"))
    }
}

#[get("/<iterations>")]
pub async fn sim(db: &State<Pool<Sqlite>>, iterations: usize) -> String {
    User::simulate_all(db.inner(), iterations).await;
    "huh".to_string()
}
