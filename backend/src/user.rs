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

    pub async fn simulate(db: &Pool<Sqlite>) {
        // simulate create listings,
        // resolve all listings

        for user in sqlx::query_as::<_, User>("select * from user where bot;").fetch_all(db).await.unwrap() {
            for _ in 0..5 {
                if utils::async_rng_bool(0.8) {
                    // pass
                    continue;
                }

                if utils::async_rng_bool(0.5) {
                    // sell
                    let inventory = Inventory::fetch_inventory(&user, db).await;
                    if inventory.is_empty() {
                        continue;
                    }

                    let item = utils::async_rng_item(&inventory).clone();
                    if item.volume <= 0f64 {
                        continue;
                    }

                    if utils::async_rng_bool(0.7) {
                        // target existing listing
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
                            continue;
                        }

                        let available_listings = available_listings
                            .iter()
                            .filter(|x| x.volume <= item.volume)
                            .map(|x| x.clone())
                            .collect::<Vec<Listing>>();

                        let targeted_listing = utils::async_rng_item(&available_listings).clone();

                        Listing::create_listing(db, targeted_listing.value, targeted_listing.volume, listing::ListingType::Sell, user.id, targeted_listing.stock_id).await;
                    } else {
                        // make new listing
                        let mut available_listings = listing::Listing::find_listings(item.stock_id, db)
                            .await
                            .iter()
                            .filter(|x|
                                x.status() == listing::Status::Pending &&
                                x.listing_type() == listing::ListingType::Sell
                            )
                            .map(|x| x.clone())
                            .collect::<Vec<listing::Listing>>();
                        available_listings.sort_by_key(|x| x.start_time);
                        
                        let median = utils::median(available_listings
                            .iter()
                            .rev()
                            .take(10)
                            .map(|x| x.value)
                            .collect::<Vec<f64>>()
                        );

                        if median.is_none() {
                            // no listings are available, create a new one at custom price
                            // TODO: base this custom price on previous price history
                            Listing::create_listing(
                                db,
                                utils::round(utils::async_rng_range(1f64, 1000f64), 2),
                                utils::round((utils::async_rng_range_int(800, 1_000) as f64 / 1_000.0) * item.volume, 2),
                                listing::ListingType::Sell,
                                user.id,
                                item.stock_id
                            ).await;
                            continue;
                        }
                        let median = median.unwrap();
                        let std_dev = stock::Stock::get_stock(db, item.stock_id).await.unwrap().std_dev;

                        Listing::create_listing(
                            db,
                            utils::round(median + (utils::async_rng_range(0.995, 1.005) * std_dev), 2),
                            utils::round((utils::async_rng_range_int(100, 1_000) as f64 / 1_000.0) * item.volume, 2),
                            listing::ListingType::Sell,
                            user.id,
                            item.stock_id
                        ).await;
                    }
                } else {
                    // buy
                }
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

#[get("/")]
pub async fn sim(db: &State<Pool<Sqlite>>) -> String {
    User::simulate(db.inner()).await;
    "huh".to_string()
}
