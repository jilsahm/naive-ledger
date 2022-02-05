#[macro_use] extern crate log;

mod configuration;
mod ledger;

pub use configuration::Configuration;

pub fn run(config: Configuration) {
    info!("starting naive ledger for {:?}", config.source);
}