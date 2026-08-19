use std::{
    fs::File,
    io::{BufReader, Error},
};

use crate::{
    config::{DatasetConfig, DatasetType, EtlStats},
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
            let mut stats = EtlStats::default();
            let data = process_customers_csv(reader, &config, &mut dims, header, &mut stats)?;

            inserts::insert_countries_and_cities(&mut db, &dims, &mut stats).unwrap();
            inserts::insert_customers(&mut db, &data, &mut stats).unwrap();

            print_summary(&stats);
        }

        DatasetType::Orders => {
            let config = DatasetConfig::orders_config();
            let mut stats = EtlStats::default();
            let data = orders::process_orders_csv(reader, &config, &mut dims, header, &mut stats)?;

            insert_status(&mut db, &dims, &mut stats).unwrap();
            insert_orders(&mut db, &data, &mut stats).unwrap();

            print_summary(&stats);
        }

        DatasetType::OrderItems => {
            let config = DatasetConfig::order_items_config();
            let mut stats = EtlStats::default();
            let data = orders_details::process_order_items_csv(
                reader, &config, &mut dims, header, &mut stats,
            )?;

            insert_order_items(&mut db, &data, &mut stats).unwrap();
            print_summary(&stats);
        }

        DatasetType::Products => {
            let config = DatasetConfig::products_config();
            let mut stats = EtlStats::default();
            let data = process_products_csv(reader, &config, &mut dims, header, &mut stats)?;
            //println!("{:?}", data);
            //println!("{:?}", stats);
            insert_categories(&mut db, &dims).unwrap();
            insert_products(&mut db, &data, &mut stats).unwrap();

            print_summary(&stats);
        }

        DatasetType::Unknown => {
            println!("Unknown dataset structure ");
        }
    }

    Ok(())
}

pub fn print_summary(stats: &EtlStats) {
    println!("\n========== ETL Resumen ==========");
    println!("Procesados: {}", stats.processed);
    println!("Insertados : {}", stats.inserted);
    println!("Salteados  : {}", stats.skipped);
    println!("Valores nulos    : {}", stats.nulls);
    println!("Errores   : {}", stats.errors);
    println!("=================================\n");
}
