use std::collections::HashMap;

use rand::prelude::*;

use rocket::State;
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{inventory::Inventory, listing, login_info::{LoginInformation, LoginResult}, utils};

#[derive(FromRow, Clone, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
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

        let mut rng = rand::thread_rng();

        for user in sqlx::query_as::<_, User>("select * from user where bot;").fetch_all(db).await.unwrap() {
            let inventory = Inventory::fetch_inventory(&user, db).await;
            for item in inventory {
                if rng.gen_bool(0.8) {
                    // pass
                    continue;
                }

                if rng.gen_bool(0.7) {
                    // target existing listing
                    let available_listings = listing::Listing::find_listings(item.stock_id, db)
                        .await
                        .iter()
                        .filter(|x| x.status() == listing::Status::Pending)
                        .collect::<Vec<&listing::Listing>>();

                    
                } else {
                    // make new listing
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
