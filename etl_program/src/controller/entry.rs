use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::{
    config::{DatasetConfig, DatasetType, detect_dataset},
    database::{connection, inserts},
    dim_values::DimensionStore,
};

use super::customers::process_customers_csv;

pub fn entry_point(header: &Vec<String>, reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut db = connection::connect().unwrap();
    let mut dims = DimensionStore::new();

    match detect_dataset(header) {
        DatasetType::Customers => {
            let config = DatasetConfig::customer_config();

            let data = process_customers_csv(reader, &config, &mut dims, header)?;

            inserts::insert_countries_and_cities(&mut db, &dims).unwrap();
            inserts::insert_customers(&mut db, &data).unwrap();
        }

        DatasetType::Orders => {
            println!("Orders dataset detected");
        }

        DatasetType::OrderItems => {
            println!("OrderItems dataset detected");
        }

        DatasetType::Products => {
            println!("Products dataset detected");
        }

        DatasetType::Unknown => {
            println!("Unknown dataset structure ❌");
        }
    }

    Ok(())
}
