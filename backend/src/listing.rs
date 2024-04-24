use serde::{Deserialize, Serialize};

use crate::stock::OwnedStock;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Listing {
    // pub id: u128,
    pub user_id: u128,
    pub stock_id: u128, // redundant?
    // TODO:remove if redundant

    pub value: f64,
    pub amount: u128,
    pub time: u64, // time in epoch unix

    pub status: Status,
    pub listing_type: ListingType,
}
impl Listing {
    pub fn cost(&self) -> f64{
        (self.amount as f64) * self.value
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Done,
    Pending,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ListingType {
    Buy,
    Sell
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ListingReward {
    // used to give money/stocks when a listing is resolved
    Buy {
        // when user's buy listing is resolved
        // give stocks

        // listing_id: u128,
        // stock_id: u128,
        // volume: u128,
        // value: f64,
        user_id: u128,
        stock: OwnedStock
    },
    Sell {
        // when user's sell listing is resolved
        // give money
        user_id: u128,
        // listing_id: u128,
        total_value: f64
    }
}
