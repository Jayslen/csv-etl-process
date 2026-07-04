use postgres::Client;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

use chrono::NaiveDate;

use crate::etl::EtlStats;
use crate::validation::*;

pub fn process_orders_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<EtlStats, Box<dyn std::error::Error>> {
    let mut stats = EtlStats::default();

    // ----------------------------------
    // LOAD customers (FK check in memory)
    // ----------------------------------
    let mut customer_set = HashSet::new();
    for row in client.query("SELECT customer_id FROM customers", &[])? {
        let id: i32 = row.get(0);
        customer_set.insert(id);
    }

    // ----------------------------------
    // PASS 1: Collect unique statuses
    // ----------------------------------
    let mut status_set = HashSet::new();

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("OrderID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();
        if validate_columns(&cols, 4).is_err() {
            continue;
        }

        status_set.insert(cols[3].trim().to_string());
    }

    // ----------------------------------
    // INSERT STATUS
    // ----------------------------------
    for status in &status_set {
        client.execute(
            "INSERT INTO status (status_name)
             VALUES ($1)
             ON CONFLICT (status_name) DO NOTHING",
            &[status],
        )?;
    }

    // ----------------------------------
    // BUILD STATUS MAP
    // ----------------------------------
    let mut status_map = HashMap::new();
    for row in client.query("SELECT status_id, status_name FROM status", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        status_map.insert(name, id);
    }

    // ----------------------------------
    // RESET FILE
    // ----------------------------------
    reader.seek(SeekFrom::Start(0))?;

    // ----------------------------------
    // START COPY (NOW SAFE)
    // ----------------------------------
    let mut writer = client.copy_in(
        "COPY orders (customer_id, order_date, status_id)
         FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("OrderID") {
            continue;
        }

        stats.processed += 1;

        let cols: Vec<&str> = line.split(',').collect();
        if validate_columns(&cols, 4).is_err() {
            stats.rejected += 1;
            continue;
        }

        // -----------------------
        // CUSTOMER ID (no DB call now)
        // -----------------------
        let customer_id: i32 = match cols[1].trim().parse() {
            Ok(v) => {
                if !customer_set.contains(&v) {
                    eprintln!("❌ Invalid customer_id: {}", v);
                    stats.rejected += 1;
                    continue;
                }
                v
            }
            Err(_) => {
                stats.rejected += 1;
                continue;
            }
        };

        // -----------------------
        // DATE
        // -----------------------
        let raw_date = cols[2].trim();

        let order_date = match NaiveDate::parse_from_str(raw_date, "%Y-%m-%d") {
            Ok(d) => d.format("%Y-%m-%d").to_string(),
            Err(_) => {
                eprintln!("❌ Invalid date: {}", raw_date);
                stats.rejected += 1;
                continue;
            }
        };

        // -----------------------
        // STATUS
        // -----------------------
        let status_name = cols[3].trim();

        let status_id = match status_map.get(status_name) {
            Some(id) => id,
            None => {
                eprintln!("❌ Missing status: {}", status_name);
                stats.rejected += 1;
                continue;
            }
        };

        // -----------------------
        // WRITE
        // -----------------------
        let row = format!("{},{},{}\n", customer_id, order_date, status_id);

        if let Err(e) = writer.write_all(row.as_bytes()) {
            eprintln!("❌ COPY write error: {:?}", e);
            stats.rejected += 1;
        } else {
            stats.inserted += 1;
        }
    }

    // ----------------------------------
    // FINISH COPY
    // ----------------------------------
    use postgres::error::DbError;

    if let Err(e) = writer.finish() {
        eprintln!("❌ COPY failed: {}", e);

        if let Some(db_err) = e.source().and_then(|e| e.downcast_ref::<DbError>()) {
            eprintln!("➡️ DB ERROR MESSAGE: {}", db_err.message());
            eprintln!("➡️ DETAIL: {:?}", db_err.detail());
            eprintln!("➡️ HINT: {:?}", db_err.hint());
            eprintln!("➡️ COLUMN: {:?}", db_err.column());
            eprintln!("➡️ TABLE: {:?}", db_err.table());
            eprintln!("➡️ CONSTRAINT: {:?}", db_err.constraint());
        }

        return Err(Box::new(e));
    }

    Ok(stats)
}
