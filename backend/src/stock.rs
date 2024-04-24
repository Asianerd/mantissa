use std::{collections::HashMap, fs::{self, File}, io::Read};

use serde::{Deserialize, Serialize};

use crate::listing::{self, Status};

// use crate::transaction;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Stock {
    pub id: u128,
    pub name: String,
    pub volume: u128,
    pub content: Vec<listing::Listing>, // only keep track of previous 100 in memory
    // pub history: Vec<listing::Listing>, rename history to content
    // history: Vec<u128>
    // load history from transaction.rs

    pub appeal: f64,
    // represents mean on the normal distribution graph
    // +ve -> sell for higher than market average
    // -ve -> sell for lower than market average

    // another appeal whether to buy or sell?
    // temporary status effect? 
    // eg: higher appeal for 1 week



    // load entire history in memory?
    // load previous X amount? <- current implementation (100 cached)
}
impl Stock {
    pub fn resolve_listings(&mut self) -> Vec<listing::ListingReward> { // u8 is a temporary value
        // needs to return actions so that
        //      users can get money when stocks are their stocks sold
        //      
        //      users can get stocks when stocks are bought

        let mut result: Vec<listing::ListingReward> = vec![];

        for _ in 0..=100 {
            // resolves a max amount of listings at one time

            // sets listing to done
            let mut a: Option<usize> = None;
            let mut b: Option<usize> = None;
            for i in self.content.iter().enumerate() {
                if i.1.status == Status::Done {
                    continue;
                }
                for ii in self.content.iter().enumerate() {
                    if ii.1.status == Status::Done {
                        continue;
                    }

                    if i.0 == ii.0 {
                        continue;
                    }

                    if (i.1.listing_type != ii.1.listing_type) && (i.1.amount == ii.1.amount) && (i.1.value == ii.1.value) {
                        a = Some(i.0);
                        b = Some(ii.0);
                        break;
                    }
                }
                if a.is_some() {
                    break;
                }
            }

            if a.is_none() {
                break;
            }

            self.content[a.unwrap()].status = Status::Done;
            self.content[b.unwrap()].status = Status::Done;
            // use the user ids of a and b to reward both users

            let mut sold = &self.content[a.unwrap()];
            let mut bought = &self.content[b.unwrap()];
            if self.content[a.unwrap()].listing_type == listing::ListingType::Buy {
                bought = &self.content[a.unwrap()];
                sold = &self.content[b.unwrap()];
            }

            result.push(listing::ListingReward::Buy {
                user_id: bought.user_id,
                stock: OwnedStock {
                    stock_id: bought.stock_id, 
                    name: self.name.clone(), 
                    volume: bought.amount, 
                    value: bought.value
                }
            });
            result.push(listing::ListingReward::Sell {
                user_id: sold.user_id,
                total_value: sold.value * (sold.amount as f64)
            })
        }
        result
    }

    pub fn load() -> HashMap<u128, Stock> {
        let mut buffer = "".to_string();
        File::open("stocks.json").unwrap().read_to_string(&mut buffer).unwrap();
        serde_json::from_str(buffer.as_str()).unwrap()
    }

    pub fn save(item: HashMap<u128, Stock>) {
        fs::write("stocks.json", serde_json::to_string_pretty(&item).unwrap()).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct OwnedStock {
    // when user buys a stock, this is appended to their inventory
    pub stock_id: u128, // reference to the actual stock
    //pub listing_id: u128, // reference to the listing (redundant?)
    pub name: String, // update once in awhile
    pub volume: u128,

    pub value: f64,
}
impl OwnedStock {

}

