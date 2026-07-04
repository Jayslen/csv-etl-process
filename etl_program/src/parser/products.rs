use postgres::Client;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

use crate::etl::EtlStats;
use crate::validation::*;

pub fn process_products_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<EtlStats, Box<dyn std::error::Error>> {
    let mut stats = EtlStats::default();
    let mut categories_set = HashSet::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("ProductID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        if validate_columns(&cols, 5).is_err() {
            stats.rejected += 1;
            continue;
        }

        categories_set.insert(cols[2].to_string());
    }

    let mut writer =
        client.copy_in("COPY categories (category_name) FROM STDIN WITH (FORMAT csv)")?;

    for c in &categories_set {
        writer.write_all(format!("{}\n", c).as_bytes())?;
    }
    writer.finish()?;

    let mut category_map = HashMap::new();
    for row in client.query("SELECT category_id, category_name FROM categories", &[])? {
        category_map.insert(row.get::<_, String>(1), row.get::<_, i32>(0));
    }

    reader.seek(SeekFrom::Start(0))?;

    let mut writer = client.copy_in(
        "COPY products (product_name, category_id, price, stock) FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in reader.lines() {
        let line = line?;
        if line.contains("ProductID") {
            continue;
        }

        stats.processed += 1;

        let cols: Vec<&str> = line.split(',').collect();

        if validate_columns(&cols, 5).is_err() {
            stats.rejected += 1;
            continue;
        }

        let category_id = match category_map.get(cols[2]) {
            Some(v) => v,
            None => {
                stats.rejected += 1;
                continue;
            }
        };

        let row = format!("{},{},{},{}\n", cols[1], category_id, cols[3], cols[4]);

        if writer.write_all(row.as_bytes()).is_ok() {
            stats.inserted += 1;
        } else {
            stats.rejected += 1;
        }
    }

    writer.finish()?;

    Ok(stats)
}
