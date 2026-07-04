use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::{
    config::{DatasetConfig, DatasetType},
    controller::{orders, orders_details},
    database::{
        connection,
        inserts::{
            self, insert_categories, insert_order_items, insert_orders, insert_products,
            insert_status,
        },
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
            let config = DatasetConfig::orders_config();

            let data = orders::process_orders_csv(reader, &config, &mut dims, header)?;

            println!("{:?}", data[0]);

            println!("{:?}", dims.get("Status"));

            insert_status(&mut db, &dims);
            insert_orders(&mut db, &data);
        }

        DatasetType::OrderItems => {
            let config = DatasetConfig::order_items_config();

            let data = orders_details::process_order_items_csv(reader, &config, &mut dims, header)?;

            insert_order_items(&mut db, &data);
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
