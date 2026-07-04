use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::{
    config::{DatasetConfig, DatasetType, detect_dataset},
    db::{connection, insert_countries_and_cities, insert_customers},
    dim_values::DimensionStore,
    parser::customers,
};

pub fn controller(header: &Vec<String>, reader: &mut BufReader<File>) -> Result<(), Error> {
    let mut db = connection().unwrap();
    let mut dims = DimensionStore::new();

    match detect_dataset(header) {
        DatasetType::Customers => {
            let config = DatasetConfig::customer_config();

            let data = customers::process_customers_csv(reader, &config, &mut dims, header)?;

            insert_countries_and_cities(&mut db, &dims).unwrap();
            insert_customers(&mut db, &data).unwrap();
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
