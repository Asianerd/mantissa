use core::fmt;
use std::collections::HashMap;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    listing::{self, ListingReward}, stock::{self, Stock}, user::{self, AccountHandler, User}, utils
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataInstance {
    pub stocks: HashMap<u128, Stock>,
    pub account_handler: AccountHandler
    // pub users: HashMap<u128, User>,
}
impl DataInstance {
    pub fn new() -> DataInstance {
        let mut result = DataInstance {
            stocks: HashMap::new(),
            account_handler: AccountHandler::new()
            // users: HashMap::new(),
            // transactions: HashMap::new(),
            // listings: HashMap::new()
        };

        let mut rng = rand::thread_rng();

        // for i in ["Hydrogen","Helium","Lithium","Beryllium","Boron","Carbon","Nitrogen","Oxygen","Fluorine","Neon","Sodium","Magnesium","Aluminum","Silicon","Phosphorus","Sulfur","Chlorine","Argon","Potassium","Calcium","Scandium","Titanium","Vanadium","Chromium","Manganese","Iron","Cobalt","Nickel","Copper","Zinc","Gallium","Germanium","Arsenic","Selenium","Bromine","Krypton","Rubidium","Strontium","Yttrium","Zirconium","Niobium","Molybdenum","Technetium","Ruthenium","Rhodium","Palladium","Silver","Cadmium","Indium","Tin","Antimony","Tellurium","Iodine","Xenon","Cesium","Barium","Lanthanum","Cerium","Praseodymium","Neodymium","Promethium","Samarium","Europium","Gadolinium","Terbium","Dysprosium","Holmium","Erbium","Thulium","Ytterbium","Lutetium","Hafnium","Tantalum","Wolfram","Rhenium","Osmium","Iridium","Platinum","Gold","Mercury","Thallium","Lead","Bismuth","Polonium","Astatine","Radon","Francium","Radium","Actinium","Thorium","Protactinium","Uranium","Neptunium","Plutonium","Americium","Curium","Berkelium","Californium","Einsteinium","Fermium","Mendelevium","Nobelium","Lawrencium","Rutherfordium","Dubnium","Seaborgium","Bohrium","Hassium","Meitnerium","Darmstadtium","Roentgenium","Copernicium","Nihonium","Flerovium","Moscovium","Livermorium","Tennessine","Oganesson"] {
        for i in ["Hydrogen","Helium","Lithium","Beryllium","Boron","Carbon"] {
            result.stocks.insert(result.stocks.len() as u128, Stock {
                id: result.stocks.len() as u128,
                name: i.to_string(),
                volume: 1_000_000,
                content: vec![
                    listing::Listing {
                        user_id: 0,
                        stock_id: result.stocks.len() as u128,
                        value: 1.0,
                        amount: 100,
                        time: 0,
                        status: listing::Status::Pending,
                        listing_type: listing::ListingType::Sell
                    },
                    listing::Listing {
                        user_id: 0,
                        stock_id: result.stocks.len() as u128,
                        value: 10.0,
                        amount: 100,
                        time: 0,
                        status: listing::Status::Pending,
                        listing_type: listing::ListingType::Buy
                    }
                ],
                appeal: 0f64
            });
        }
        

        for _ in 0..2 {
            result.account_handler.users.insert(result.account_handler.users.len() as u128, User {
                id: result.account_handler.users.len() as u128,
                name: utils::generate_name(&mut rng),
                bot: true,
                inventory: vec![],
                capital: 1_000_000.0
                // listings: vec![]
            });
        }

        for u in &mut result.account_handler.users {
            for s in &result.stocks {
                if rng.gen_bool(0.5) {
                    continue;
                }
                
                u.1.inventory.push(stock::OwnedStock {
                    stock_id: s.1.id,
                    name: s.1.name.clone(),
                    volume: rng.gen_range(1..=5) * 100,
                    value: (((rng.gen_range(1.0..10.0)) * 100.0) as i64) as f64 / 100.0
                });
            }
        }

        // result.stocks.insert(0, Stock {
        //     id: 0,
        //     name: "Random".to_string(),
        //     volume: 100,
        //     content: vec![
        //         listing::Listing {
        //             user_id: 1,
        //             stock_id: 0,
        //             value: 1.00,
        //             amount: 200,
        //             time: 0,
        //             status: listing::Status::Pending,
        //             listing_type: listing::ListingType::Buy
        //         }
        //     ],
        //     appeal: 0.0
        // });

        // result.users.insert(0, User {
        //     id: 0,
        //     name: "Lorem".to_string(),
        //     bot: true,
        //     capital: 300.0,
        //     inventory: vec![
        //         stock::OwnedStock {
        //             stock_id: 0,
        //             name: "".to_string(),
        //             volume: 300,
        //             value: 1.00
        //         }
        //     ]
        // });

        // result.users.insert(1, User {
        //     id: 1,
        //     name: "Ipsum".to_string(),
        //     bot: true,
        //     capital: 0.0,
        //     inventory: vec![]
        // });

        result
    }

    pub fn simulate(&mut self, rng: &mut ThreadRng) {
        let mut listings: Vec<listing::Listing> = vec![];
        let stocks = self.stocks.values().cloned().collect::<Vec<Stock>>();
        // copied so that it doesnt have to make a new values() vec every iteration

        for (_, v) in &mut self.account_handler.users {
            listings.append(&mut v.simulate(rng, &stocks).clone());
        }

        for l in listings {
            self.stocks.get_mut(&l.stock_id).unwrap().content.push(l);
        }
    }

    pub fn resolve_listings(&mut self) {
        // if theres a sell listing of 100 stock a at $1.00
        // and buy listing of a 100 stock b at $1.00, then theyre resolved
        let mut result: Vec<ListingReward> = vec![];
        for stock in &mut self.stocks.values_mut() {
            result.append(&mut stock.resolve_listings());
        }

        for reward in result {
            match reward {
                ListingReward::Buy { user_id, stock} => {
                    self.account_handler.users.get_mut(&user_id).unwrap().inventory.push(stock.clone());
                },
                ListingReward::Sell { user_id, total_value } => {
                    self.account_handler.users.get_mut(&user_id).unwrap().capital += total_value;
                }
            }
        }
    }

    pub fn load() -> DataInstance {
        // load from memory
        let mut result = DataInstance::new();

        result.account_handler = user::AccountHandler::load();
        result.stocks = stock::Stock::load();

        result
    }

    pub fn save(&self) {
        // save to memory

        self.account_handler.save();
        stock::Stock::save(self.stocks.clone());
    }
}
impl fmt::Display for DataInstance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}
