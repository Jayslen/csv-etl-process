mod config;
mod db;
mod dim_values;

use std::{env, fs::File, io::Error};

use std::io::BufReader;

use crate::config::DatasetConfig;
use crate::db::{connection, insert_countries_and_cities};
use crate::dim_values::DimensionStore;

mod parser;
use parser::customers;
fn main() -> Result<(), Error> {
    let path: Vec<String> = env::args().collect();

    let file = File::open(&path[1]).expect("File does not exist");
    let mut reader = BufReader::new(file);

    let config = DatasetConfig::customer_config();
    let mut db = connection().unwrap();
    let mut dims = DimensionStore::new();
    let data = customers::process_customers_csv(&mut reader, &config, &mut dims)?;

    insert_countries_and_cities(&mut db, &dims);

    Ok(())
}
