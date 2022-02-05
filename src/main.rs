use clap::Parser;
use naive_ledger::Configuration;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = Configuration::parse();
    naive_ledger::run(config);
}