use serde::{Deserialize, Serialize};

use crate::listing::ListingType;

// not used anymore
// functionality replaced by listing

// used as buy/sell history log only
#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: u128,
    pub user_id: u128,
    pub stock_id: u128,
    pub listing_type: ListingType, // dictates whether its a buy/sell
    pub value: f64
}
