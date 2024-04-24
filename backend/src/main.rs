use std::sync::Mutex;

use data_instance::DataInstance;
// use std::time::Instant;
#[macro_use] extern crate rocket;
use rocket::State;

// use rand::prelude::*;

mod utils;
// mod account_handler;
// mod name_generator;

mod stock;
mod user;
// mod transaction;
mod article;
mod listing;

mod data_instance;
mod cors;
// mod stock;

#[get("/")]
fn debug_data_instance(instance: &State<Mutex<DataInstance>>) -> String {
    let d = instance.lock().unwrap();
    d.to_string()
}

#[get("/")]
fn get_stocks(instance: &State<Mutex<DataInstance>>) -> String {
    let d = instance.lock().unwrap();
    serde_json::to_string_pretty(&d.stocks).unwrap()
}

#[get("/")]
fn get_users(instance: &State<Mutex<DataInstance>>) -> String {
    let d = instance.lock().unwrap();
    serde_json::to_string_pretty(&d.account_handler.users).unwrap()
}

#[get("/")]
fn simulate(instance: &State<Mutex<DataInstance>>) -> String {
    let mut d = instance.lock().unwrap();
    d.simulate(&mut rand::thread_rng());
    serde_json::to_string_pretty(&d.stocks.values().filter(|x| x.content.len() > 1).collect::<Vec<&stock::Stock>>()).unwrap()
}

#[get("/")]
fn resolve_listings(instance: &State<Mutex<DataInstance>>) -> String {
    let mut d = instance.lock().unwrap();
    d.resolve_listings();
    serde_json::to_string_pretty(&d.stocks).unwrap()
}

#[get("/")]
fn simulate_and_resolve(instance: &State<Mutex<DataInstance>>) -> String {
    let mut d = instance.lock().unwrap();
    d.simulate(&mut rand::thread_rng());
    d.resolve_listings();
    serde_json::to_string_pretty(&d.stocks).unwrap()
}

#[get("/")]
fn index() -> String {
    "can you understand me?".to_string()
}

#[get("/<username>/<password>")]
fn debug_login(instance: &State<Mutex<DataInstance>>, username: String, password: String) -> String {
    let d = instance.lock().unwrap();
    format!("{:?}", d.account_handler.login(&username, &password))
}

#[get("/")]
fn save(instance: &State<Mutex<DataInstance>>) -> &str {
    let d = instance.lock().unwrap();
    // d.account_handler.save();
    d.save();
    "success(?)"
}

#[get("/")]
fn load(instance: &State<Mutex<DataInstance>>) -> &str {
    let mut d = instance.lock().unwrap();
    // d.account_handler = user::AccountHandler::load();
    *d = data_instance::DataInstance::load().clone();
    "success(?)"
}

#[launch]
fn rocket() -> _ {
    // read from json every time?
    //   - backend becomes near useless
    //   - boring
    //
    //       or
    //
    // save every 5 mins? <- current implementation
    //   - load everything in memory
    //      - expensive
    //   - more fun

    // TODO:revisit in future
    rocket::build()
        // .manage(Mutex::new(rand::thread_rng()))
        .manage(Mutex::new(data_instance::DataInstance::new()))
        .mount("/", routes![index])
        .mount("/debug", routes![debug_data_instance])
        .mount("/users", routes![get_users])
        .mount("/stocks", routes![get_stocks])
        .mount("/simulate", routes![simulate])
        .mount("/resolve", routes![resolve_listings])
        .mount("/iterate", routes![simulate_and_resolve])

        .mount("/login", routes![debug_login])
        .mount("/save", routes![save])
        .mount("/load", routes![load])

        .attach(cors::CORS)
}
