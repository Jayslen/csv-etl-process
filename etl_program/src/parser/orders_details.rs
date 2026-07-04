use postgres::Client;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

use crate::etl::EtlStats;
use crate::validation::*;

pub fn process_order_details_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<EtlStats, Box<dyn std::error::Error>> {
    let mut stats = EtlStats::default();

    // 🔹 Build order map
    let mut order_map: HashMap<i32, i32> = HashMap::new();
    for row in client.query("SELECT order_id FROM orders", &[])? {
        let id: i32 = row.get(0);
        order_map.insert(id, id);
    }

    // 🔹 Build product map
    let mut product_map: HashMap<i32, i32> = HashMap::new();
    for row in client.query("SELECT product_id FROM products", &[])? {
        let id: i32 = row.get(0);
        product_map.insert(id, id);
    }

    // 🔥 Debug (VERY IMPORTANT)
    println!("Loaded orders: {}", order_map.len());
    println!("Loaded products: {}", product_map.len());

    // 🔹 Reset reader
    reader.seek(SeekFrom::Start(0))?;

    let mut lines = reader.lines();

    // ✅ Skip header safely
    lines.next();

    // 🔹 COPY writer
    let mut writer = client.copy_in(
        "COPY order_details (order_id, product_id, quantity, total) FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                stats.rejected += 1;
                stats.errors.push(e.to_string());
                continue;
            }
        };

        stats.processed += 1;

        let cols: Vec<&str> = line.split(',').collect();

        if validate_columns(&cols, 4).is_err() {
            stats.rejected += 1;
            continue;
        }

        // ✅ Parse + trim (CRITICAL FIX)
        let order_id: i32 = match cols[0].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                stats.rejected += 1;
                continue;
            }
        };

        let product_id: i32 = match cols[1].trim().parse() {
            Ok(v) => v,
            Err(_) => {
                stats.rejected += 1;
                continue;
            }
        };

        // 🔍 Validate existence
        if !order_map.contains_key(&order_id) {
            println!("❌ Missing order_id: {}", order_id); // debug
            stats.rejected += 1;
            continue;
        }

        if !product_map.contains_key(&product_id) {
            println!("❌ Missing product_id: {}", product_id); // debug
            stats.rejected += 1;
            continue;
        }

        // ✅ Build row
        let row = format!(
            "{},{},{},{}\n",
            order_id,
            product_id,
            cols[2].trim(),
            cols[3].trim()
        );

        if let Err(e) = writer.write_all(row.as_bytes()) {
            stats.rejected += 1;
            stats.errors.push(e.to_string());
        } else {
            stats.inserted += 1;
        }
    }

    writer.finish()?;

    Ok(stats)
}
