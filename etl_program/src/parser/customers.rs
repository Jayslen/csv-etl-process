use postgres::Client;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};

use crate::etl::EtlStats;
use crate::validation::*;

pub fn process_customers_csv(
    client: &mut Client,
    reader: &mut BufReader<File>,
) -> Result<EtlStats, Box<dyn std::error::Error>> {
    let mut stats = EtlStats::default();

    let mut countries_set = HashSet::new();
    let mut cities_set = HashSet::new();
    let mut seen_emails = HashSet::new();

    // 🔹 FIRST PASS
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                stats.rejected += 1;
                stats.errors.push(e.to_string());
                continue;
            }
        };

        if line.contains("CustomerID") {
            continue;
        }

        let cols: Vec<&str> = line.split(',').collect();

        if validate_columns(&cols, 7).is_err() {
            stats.rejected += 1;
            continue;
        }

        countries_set.insert(cols[6].to_string());
        cities_set.insert((cols[5].to_string(), cols[6].to_string()));
    }

    // 🔹 INSERT COUNTRIES
    let mut writer =
        client.copy_in("COPY countries (country_name) FROM STDIN WITH (FORMAT csv)")?;

    for c in &countries_set {
        writer.write_all(format!("{}\n", c).as_bytes())?;
    }
    writer.finish()?;

    // 🔹 BUILD MAPS
    let mut country_map = HashMap::new();
    for row in client.query("SELECT country_id, country_name FROM countries", &[])? {
        country_map.insert(row.get::<_, String>(1), row.get::<_, i32>(0));
    }

    let mut writer =
        client.copy_in("COPY cities (city_name, country_id) FROM STDIN WITH (FORMAT csv)")?;

    for (city, country) in &cities_set {
        if let Some(country_id) = country_map.get(country) {
            writer.write_all(format!("{},{}\n", city, country_id).as_bytes())?;
        }
    }
    writer.finish()?;

    let mut city_map = HashMap::new();
    for row in client.query("SELECT city_id, city_name FROM cities", &[])? {
        city_map.insert(row.get::<_, String>(1), row.get::<_, i32>(0));
    }

    // 🔹 SECOND PASS
    reader.seek(SeekFrom::Start(0))?;

    let mut writer = client.copy_in(
        "COPY customers (first_name, last_name, email, phone, city_id, country_id) FROM STDIN WITH (FORMAT csv)",
    )?;

    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                stats.rejected += 1;
                stats.errors.push(e.to_string());
                continue;
            }
        };

        if line.contains("CustomerID") {
            continue;
        }

        stats.processed += 1;

        let cols: Vec<&str> = line.split(',').collect();

        if validate_columns(&cols, 7).is_err() {
            stats.rejected += 1;
            continue;
        }

        if validate_not_empty(cols[3], "email").is_err() {
            stats.rejected += 1;
            continue;
        }

        if !seen_emails.insert(cols[3].to_string()) {
            stats.rejected += 1;
            continue;
        }

        let city_id = match city_map.get(cols[5]) {
            Some(v) => v,
            None => {
                stats.rejected += 1;
                continue;
            }
        };

        let country_id = match country_map.get(cols[6]) {
            Some(v) => v,
            None => {
                stats.rejected += 1;
                continue;
            }
        };

        let row = format!(
            "{},{},{},{},{},{}\n",
            cols[1], cols[2], cols[3], cols[4], city_id, country_id
        );

        if writer.write_all(row.as_bytes()).is_ok() {
            stats.inserted += 1;
        } else {
            stats.rejected += 1;
        }
    }

    writer.finish()?;

    Ok(stats)
}
