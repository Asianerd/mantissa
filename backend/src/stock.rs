use std::{collections::HashMap, fs};

use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Sqlite};

use crate::{listing::{self, Listing}, utils};

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
        sqlx::query_as("select * from stock;")
            .fetch_all(db)
            .await
            .unwrap()
    }

    pub async fn get_stock(db: &Pool<Sqlite>, stock_id: i64) -> Option<Stock> {
        sqlx::query_as("select * from stock where id = $1;")
            .bind(stock_id)
            .fetch_optional(db)
            .await
            .unwrap()
    }

    pub async fn get_pricing_history(db: &Pool<Sqlite>, stock_id: i64, start_period: i64) -> Vec<(f64, i64)> { // (price, time)
        sqlx::query_as("select value, end_time from listing where (end_time >= $1) and (stock_id = $2) and (listing_type = $3) and (status = $4) order by end_time asc;")
            .bind(start_period)
            .bind(stock_id)
            .bind(listing::ListingType::Buy.to_string())
            .bind(listing::Status::Resolved.to_string())
            .fetch_all(db)
            .await
            .unwrap()
    }

    // #region svg generation
    pub async fn generate_svg_thumbnail(db: &Pool<Sqlite>, stock_id: i64) {
        let size = (250, 160);
        let length = 14; // take last 14 days worth of trading data
        let mut result = "<Svg>".to_string();

        let date_segmented_data = Stock::segment_by_date(Stock::get_pricing_history(db, stock_id, utils::get_time() - (14 * 86400)).await);
        let start_date = utils::epoch_to_date(utils::get_time() - (14 * 86400));
        let mut candlestick_data: Vec<Option<((f64, f64), (f64, f64))>> = vec![];

        for offset in 0..length {
            let target_date = start_date + offset;
            candlestick_data.push(
                match date_segmented_data.get(&target_date) {
                    Some(d) => {
                        Stock::generate_candlestick(d.to_vec())
                    },
                    None => None
                }
            );
        }

        println!("{candlestick_data:?}");

        let bounds = ( // (min, max)
            candlestick_data.clone().iter().filter(|x| x.is_some()).map(|x| x.unwrap().1.0).min_by(|a, b| a.total_cmp(b)).unwrap_or(0f64),
            candlestick_data.clone().iter().filter(|x| x.is_some()).map(|x| x.unwrap().1.1).max_by(|a, b| a.total_cmp(b)).unwrap_or(0f64)
        );

        let increment = (0.8 * size.1 as f64) / (bounds.1 - bounds.0);
        let padding = 0.2 * size.1 as f64;

        for (index, data) in candlestick_data.iter().enumerate() {
            println!("data : {data:?}");
            if data.is_none() {
                continue;
            }
            let data = data.unwrap();
            println!("increment : {increment}");
            println!("padding: {padding}");
            println!("bounds : {bounds:?}");
            result += format!("<Rect x='{}' y='{}' width='15' height='{}' />", index * 15, padding + ((data.1.1 - bounds.1) * increment), (data.1.1 - data.1.0) * increment).as_str();
        }

        result += "</Svg>";

        fs::write("stock_thumbnail.svg", result).unwrap();
    }


    fn generate_candlestick(d: Vec<(f64, i64)>) -> Option<((f64, f64), (f64, f64))> { // ((open, close), (low, high))
        if d.is_empty() {
            return None;
        }
        let mut d = d;
        d.sort_by_key(|x| x.1.clone());
        let a = d.clone();
        d.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        Some(
            (
                (
                    a.iter().rev().last().unwrap().0,
                    a.last().unwrap().0
                ),
                (
                    d.iter().rev().last().unwrap().0,
                    d.last().unwrap().0
                )
            )
        )
    }

    fn segment_by_date(collection: Vec<(f64, i64)>) -> HashMap<i64, Vec<(f64, i64)>> {
        let mut result: HashMap<i64, Vec<(f64, i64)>> = HashMap::new();

        for d in collection {
            let date = utils::epoch_to_date(d.1);

            if !result.contains_key(&date) {
                result.insert(date, vec![]);
            }

            result.get_mut(&date).unwrap().push(d.clone());
        }

        result
    }
    // #endregion
}

#[get("/<amount>")]
pub async fn fetch_highest_valued(db: &State<Pool<Sqlite>>, amount: usize) -> String {
    let mut result = Stock::get_all_stocks(db.inner()).await;
    result.sort_by(|a, b| a.recent.partial_cmp(&b.recent).unwrap());
    utils::parse_response(Ok(serde_json::to_string(&result.iter().rev().take(amount).collect::<Vec<&Stock>>()).unwrap()))
}

#[get("/<stock_id>/<start_period>")]
pub async fn pricing_history(db: &State<Pool<Sqlite>>, stock_id: i64, start_period: i64) -> String {
    utils::parse_response(Ok(Stock::get_pricing_history(db.inner(), stock_id, start_period).await))
}

#[get("/<stock_id>")]
pub async fn generate_svg(db: &State<Pool<Sqlite>>, stock_id: i64) {
    Stock::generate_svg_thumbnail(db, stock_id).await;
}
