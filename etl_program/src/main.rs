mod config;
mod etl;
mod parser;
mod validation;

use std::{
    env,
    fs::File,
    io::{BufRead, Read},
};

use config::{ConfigData, Dataset, connection};
use parser::{customers, orders, orders_details, products};
use std::io::BufReader;

fn main() {
    let path: Vec<String> = env::args().collect();
    let db = &mut connection().unwrap();
    let csv_config = ConfigData::init();

    let file = File::open(&path[1]).expect("File does not exist");
    let mut reader = BufReader::new(file);
    let header = reader
        .by_ref()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    let data_to_insert = csv_config.which_dataset(&header).unwrap();
    match data_to_insert {
        Dataset::Customers => match customers::process_customers_csv(db, &mut reader) {
            Ok(stats) => stats.print_summary("Customers"),
            Err(e) => {
                eprintln!("Error processing CSV: {}", e);
                if let Some(db_err) = e.downcast_ref::<postgres::Error>() {
                    eprintln!("DB ERROR: {:?}", db_err);
                }
            }
        },

        Dataset::OrdersDetails => {
            match orders_details::process_order_details_csv(db, &mut reader) {
                Ok(stats) => stats.print_summary("Orders"),
                Err(e) => {
                    eprintln!("❌ Error processing CSV: {}", e);

                    let mut source = e.source();
                    while let Some(err) = source {
                        eprintln!("➡️ Caused by: {}", err);
                        source = err.source();
                    }
                }
            }
        }

        Dataset::Orders => match orders::process_orders_csv(db, &mut reader) {
            Ok(stats) => stats.print_summary("Orders"),
            Err(e) => eprintln!("Error processing CSV: {}", e),
        },

        Dataset::Products => match products::process_products_csv(db, &mut reader) {
            Ok(stats) => stats.print_summary("Products"),
            Err(e) => eprintln!("Error processing CSV: {}", e),
        },
    }
}
