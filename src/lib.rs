#[macro_use] extern crate log;

mod configuration;
mod ledger;

use std::{fs::File, io::BufReader};

pub use configuration::Configuration;

use crate::ledger::{Ledger, Transaction};

pub fn run(config: Configuration) {
    info!("starting naive ledger for {:?}", config.source);
    match File::open(config.source) {
        Err(e) => panic!("{}", e),
        Ok(file) => {
            let buffer = BufReader::new(file);
            let mut ledger = Ledger::default();
            for transaction in Transaction::reader(buffer) {
                match transaction {
                    Err(e) => warn!("skipping CSV entry because: {}", e),
                    Ok(transaction) => {
                        if let Err(what) = ledger.update(transaction) {
                            warn!("{}", what);
                        }
                    }
                }
            }
            ledger.print();
        }
    }    
}