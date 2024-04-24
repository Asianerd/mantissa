use std::{collections::HashMap, fs::{self, File}, io::Read};

use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

use crate::{listing::{Listing, ListingType, Status}, stock, utils};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u128,
    pub name: String,
    // password stored in json
    pub bot: bool,
    // add some kinda risk factor?

    pub capital: f64,
    // amount of money in the account

    pub inventory: Vec<stock::OwnedStock>,
    // inventory needs to store amount of stock + price it was bought at

    // purchase history in json
    // pub listings: Vec<u128>
    // listings not stored, queried stock-by-stock instead

    // store listing per stock <- current implementation
    //      - querying listings will need to iterate every stock
    //          - but its rare that a user will want to see every listing from all their stocks
    //          - a user comparing all listings from a single stock only is more common
    // store listing per user?
    //      - querying listings will need to iterate every user
    // store id to listing
    //      - query faster
}
impl User {
    // fn remove_from_inventory(&mut self, stock_id: u128, value: f64, amount: u128) {
    fn remove_from_inventory(&mut self, index: usize, amount: u128) {
        // removes x amount of OwnedStock from inventory

        self.inventory[index].volume -= amount;
        self.remove_empty_stocks();
    }

    fn remove_empty_stocks(&mut self) {
        // remove OwnedStocks that have 0 amount
        self.inventory = self.inventory.iter().filter(|x| x.volume > 0).map(|x| x.clone()).collect::<Vec<stock::OwnedStock>>().clone();
    }

    // simulate for multiple stocks
    pub fn simulate(&mut self, rng: &mut ThreadRng, stocks: &Vec<stock::Stock>) -> Vec<Listing> {
        let mut result: Vec<Listing> = vec![];
        for i in stocks {
            let r = self.per_stock_simulate(rng, i);
            if r.is_none() {
                continue;
            }
            result.push(r.unwrap());
        }

        result
    }

    // simulate for one stock only
    fn per_stock_simulate(&mut self, rng: &mut ThreadRng, stock: &stock::Stock) -> Option<Listing> {
        // if rng.gen_bool(0.99999) {
        // chance for the user not to make any listings
        if rng.gen_bool(0.9) {
            return None;
        }

        //let available: &Vec<OwnedStock> = self.inventory.iter().filter(|x| x.stock_id == stock.id).collect::<Vec<&stock::OwnedStock>>();
        let temp = self.inventory.clone();
        let available = temp.iter().enumerate().map(|(index, x)| (index, x)).filter(|(_, x)| x.stock_id == stock.id).collect::<Vec<(usize, &stock::OwnedStock)>>();

        // need to have a bias towards already existing listings
        if rng.gen_bool(0.7) {
            // fulfil one of the existing listings

            if rng.gen_bool(0.5) && (available.len() != 0) {
                // sell
                self.simulate_existing_listing(ListingType::Sell, rng, stock, available);

                // for i in &stock.content {
                //     if i.status == Status::Done {
                //         continue;
                //     }
                //     if i.user_id == self.id {
                //         // dont buy your own stocks
                //         continue;
                //     }
                //     if i.listing_type == ListingType::Buy {
                //         // if someone else wants to buy this stock

                //         // if i have enough stocks to sell to them
                //         // + if i will make a profit selling the stocks

                //         for (a_index, a) in &available {
                //             // if amount of owned stocks >= amount in listing
                //             if a.value > i.value {
                //                 // ignore this owned stock if i bought it for higher price than the listing
                //                 // eg.
                //                 // owned (bought for)               : $1.05
                //                 // listing wants wants to buy for   : $1.00
                //                 // ignore this listing^
                //                 continue;
                //             }

                //             if a.volume < i.amount {
                //                 // listing wants to buy more than what i own
                //                 continue;
                //             }

                //             // remove the amount from inventory
                //             self.remove_from_inventory(*a_index, i.amount);

                //             // sell the amount listed
                //             return Some(Listing {
                //                 user_id: self.id,
                //                 stock_id: stock.id,
                //                 value: i.value,
                //                 amount: i.amount,
                //                 time: utils::get_time(),
                //                 status: Status::Pending,
                //                 listing_type: ListingType::Sell
                //             });
                //         }
                //     }
                // }
            } else {
                // buy
                self.simulate_existing_listing(ListingType::Buy, rng, stock, available);
            }

            None
        } else {
            if rng.gen_bool(0.5) && (available.len() != 0) {
                return self.simulate_new_listing(ListingType::Sell, rng, stock, available);
            } else {
                return self.simulate_new_listing(ListingType::Buy, rng, stock, available);
            }
            // if rng.gen_bool(0.5) && (available.len() != 0) {
            //     // sell
    
            //     let rand_index = rng.gen_range(0..available.len());
            //     let selection = available[rand_index].clone();
    
            //     self.inventory.remove(rand_index);
    
            //     return Some(Listing {
            //         user_id: self.id,
            //         stock_id: stock.id,
            //         // randomized value needs more work (obviously)
            //         value: (((selection.value * rng.gen_range(0.999..=1.001)) * 100.0) as i64) as f64 / 100.0,
            //         amount: selection.volume,
            //         time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            //         status: crate::listing::Status::Pending,
            //         listing_type: ListingType::Sell
            //     });
            // } else {
            //     // buy
    
            //     let value = (((stock.content.last().unwrap().value * rng.gen_range(0.999..1.001)) * 100.0) as i64) as f64 / 100.0;
            //     let amount = rng.gen_range(1..=2) * 100;
            //     let cost = value * (amount as f64);
            //     if cost > self.capital {
            //         return None;
            //     }
    
            //     self.capital -= cost;
    
            //     return Some(Listing {
            //         user_id: self.id,
            //         stock_id: stock.id,
            //         value: value,
            //         amount: amount,
            //         time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            //         status: crate::listing::Status::Pending,
            //         listing_type: ListingType::Buy
            //     });
            // }
        }
    }

    fn simulate_existing_listing(&mut self, listing_type: ListingType, _: &mut ThreadRng, stock: &stock::Stock, available: Vec<(usize, &stock::OwnedStock)>) -> Option<Listing> {
        match listing_type {
            ListingType::Buy => {
                for i in &stock.content {
                    if i.status == Status::Done {
                        continue;
                    }
                    if i.user_id == self.id {
                        // dont buy your own stocks
                        continue;
                    }
                    if i.listing_type == ListingType::Sell {
                        // if someone else wants to sell this stock

                        // if i have enough money to buy this stock

                        if i.cost() <= self.capital {
                            return Some(Listing {
                                user_id: self.id,
                                stock_id: i.stock_id,
                                value: i.value,
                                amount: i.amount,
                                time: utils::get_time(),
                                status: Status::Pending,
                                listing_type: ListingType::Buy
                            });
                        }
                    }
                }
            }
            ListingType::Sell => {
                for i in &stock.content {
                    if i.status == Status::Done {
                        continue;
                    }
                    if i.user_id == self.id {
                        // ignore own listings
                        continue;
                    }
                    if i.listing_type == ListingType::Buy {
                        // if someone else wants to buy this stock
        
                        // if i have enough stocks to sell to them
                        // + if i will make a profit selling the stocks
        
                        for (a_index, a) in &available {
                            // if amount of owned stocks >= amount in listing
                            if a.value > i.value {
                                // ignore this owned stock if i bought it for higher price than the listing
                                // eg.
                                // owned (bought for)               : $1.05
                                // listing wants wants to buy for   : $1.00
                                // ignore this listing^
                                continue;
                            }
        
                            if a.volume < i.amount {
                                // listing wants to buy more than what i own
                                continue;
                            }
        
                            // remove the amount from inventory
                            self.remove_from_inventory(*a_index, i.amount);
        
                            // sell the amount listed
                            return Some(Listing {
                                user_id: self.id,
                                stock_id: stock.id,
                                value: i.value,
                                amount: i.amount,
                                time: utils::get_time(),
                                status: Status::Pending,
                                listing_type: ListingType::Sell
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn simulate_new_listing(&mut self, listing_type: ListingType, rng: &mut ThreadRng, stock: &stock::Stock, available: Vec<(usize, &stock::OwnedStock)>) -> Option<Listing> {
        match listing_type {
            ListingType::Buy => {
                // buy

                let value = (((stock.content.last().unwrap().value * rng.gen_range(0.999..1.001)) * 100.0) as i64) as f64 / 100.0;
                let amount = rng.gen_range(1..=2) * 100;
                let cost = value * (amount as f64);
                if cost > self.capital {
                    return None;
                }

                self.capital -= cost;

                return Some(Listing {
                    user_id: self.id,
                    stock_id: stock.id,
                    value: value,
                    amount: amount,
                    time: utils::get_time(),
                    status: crate::listing::Status::Pending,
                    listing_type: ListingType::Buy
                });
            },
            ListingType::Sell => {
                let rand_index = rng.gen_range(0..available.len());
                let selection = available[rand_index].1.clone();

                self.inventory.remove(rand_index);

                return Some(Listing {
                    user_id: self.id,
                    stock_id: stock.id,
                    // randomized value needs more work (obviously)
                    value: (((selection.value * rng.gen_range(0.999..=1.001)) * 100.0) as i64) as f64 / 100.0,
                    amount: selection.volume,
                    time: utils::get_time(),
                    status: crate::listing::Status::Pending,
                    listing_type: ListingType::Sell
                });
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountHandler {
    pub users: HashMap<u128, User>,
}
impl AccountHandler {
    pub fn new() -> AccountHandler {        
        AccountHandler {
            users: HashMap::new()
        }
    }

    fn passwords() -> HashMap<u128, String> {
        serde_json::from_str(fs::read_to_string("passwords.json").unwrap().as_str()).unwrap()

        // let mut buffer = "".to_string();
        // File::open("passwords.json").unwrap().read_to_string(&mut buffer).unwrap();
        // serde_json::from_str(buffer.as_str()).unwrap()
    }

    pub fn username_exists(&self, username: &String) -> bool {
        for (_, u) in &self.users {
            if u.name == *username {
                return true;
            }
        }
        false
    }

    pub fn fetch_user_id(&self, username: &String) -> Option<u128> {
        for (_, u) in &self.users {
            if u.name == *username {
                return Some(u.id);
            }
        }
        None
    }

    pub fn login(&self, username: &String, password: &String) -> AccountResult {
        let user_id = self.fetch_user_id(username);
        if user_id.is_none() {
            return AccountResult::UsernameNoExist;
        }

        let passwords = AccountHandler::passwords();
        let fetch_result = passwords.get(&user_id.unwrap());
        if fetch_result.is_none() {
            return AccountResult::UserIDNoExist;
        }
        if fetch_result.unwrap() == password {
            return AccountResult::Success(user_id.unwrap());
        }

        AccountResult::PasswordWrong
    }

    pub fn load() -> AccountHandler {
        let mut result = AccountHandler::new();

        let mut buffer = "".to_string();
        File::open("users.json").unwrap().read_to_string(&mut buffer).unwrap();
        result.users = serde_json::from_str(buffer.as_str()).unwrap();

        result
    }

    pub fn save(&self) {
        fs::write("users.json", serde_json::to_string_pretty(&self.users).unwrap()).unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum AccountResult {
    Success(u128),
    UsernameNoExist,
    UserIDNoExist,
    PasswordWrong,
}

