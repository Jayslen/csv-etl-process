use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::{
    config::{DatasetConfig, DatasetType},
    database::{
        connection,
        inserts::{self, insert_categories, insert_products},
    },
    dim_values::DimensionStore,
    utils::detect_dataset,
};

use super::customers::process_customers_csv;
use super::products::process_products_csv;

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
            let config = DatasetConfig::products_config();

            let data = process_products_csv(reader, &config, &mut dims, header)?;
            println!("{:?}", data[0]);
            insert_categories(&mut db, &dims).unwrap();
            insert_products(&mut db, &data).unwrap();
        }

        DatasetType::Unknown => {
            println!("Unknown dataset structure ❌");
        }
    }

    Ok(())
}
