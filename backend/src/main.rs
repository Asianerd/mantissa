#[macro_use] extern crate rocket;
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};

mod utils;
mod cors;
mod soterius;

mod database;

mod login_info;
mod user;
mod stock;
mod inventory;
mod listing;

#[launch]
async fn rocket() -> _ {
    rocket::custom(rocket::config::Config::figment().merge(("port", 8003)))
        .manage(SqlitePool::connect_with(SqliteConnectOptions::new()
            .filename("db")
        ).await.unwrap())
        .attach(cors::CORS)
        .mount("/debug", routes![database::debug_users])
        .mount("/test", routes![user::test])
        .mount("/sim_test", routes![user::sim])


        .mount("/shares/highest", routes![stock::fetch_highest_valued])
        .mount("/shares/pricing", routes![stock::pricing_history])
        .mount("/shares/svg", routes![stock::generate_svg])

        .mount("/listing/by_id", routes![listing::fetch_by_id])
        .mount("/listing/by_user", routes![listing::fetch_by_user])
}
