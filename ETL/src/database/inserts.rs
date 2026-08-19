use crate::config::{EtlStats, Value};
use crate::database::preload::{load_customer_ids, load_orders, load_products};
use crate::dim_values::DimensionStore;
use chrono::NaiveDate;
use postgres::Client;
use std::error::Error;

pub fn insert_countries_and_cities(
    client: &mut Client,
    dims: &DimensionStore,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    if let Some(countries) = dims.get("countries") {
        for (country, id) in countries {
            stats.processed += 1;

            if let Err(_) = client.execute(
                "INSERT INTO countries (country_id, country_name)
                 VALUES ($1, $2)
                 ON CONFLICT DO NOTHING",
                &[&(*id as i32), country],
            ) {
                stats.errors += 1;
            } else {
                stats.inserted += 1;
            }
        }
    }

    if let Some(cities) = dims.get("cities") {
        for (key, city_id) in cities {
            stats.processed += 1;

            let parts: Vec<&str> = key.split('|').collect();
            if parts.len() != 2 {
                stats.skipped += 1;
                continue;
            }

            let city = parts[0];
            let country = parts[1];

            let country_map = match dims.get("countries") {
                Some(m) => m,
                None => {
                    stats.skipped += 1;
                    continue;
                }
            };

            let country_id = match country_map.get(country) {
                Some(id) => *id as i32,
                None => {
                    stats.skipped += 1;
                    continue;
                }
            };

            if let Err(_) = client.execute(
                "INSERT INTO cities (city_id, city_name, country_id)
                 VALUES ($1, $2, $3)
                 ON CONFLICT DO NOTHING",
                &[&(*city_id as i32), &city, &country_id],
            ) {
                stats.errors += 1;
            } else {
                stats.inserted += 1;
            }
        }
    }

    Ok(())
}
pub fn insert_customers(
    client: &mut Client,
    data: &Vec<Vec<Value>>,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    for row in data {
        stats.processed += 1;

        let customer_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let first_name = row.get(1).map(|v| v.as_string()).unwrap_or_default();
        let last_name = row.get(2).map(|v| v.as_string()).unwrap_or_default();
        let email = row.get(3).map(|v| v.as_string()).unwrap_or_default();
        let phone = row.get(4).map(|v| Some(v.as_string()));

        let city_id = match row.get(5) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        match client.execute(
            "INSERT INTO customers
             (customer_id, first_name, last_name, email, phone, city_id)
             VALUES ($1,$2,$3,$4,$5,$6)
             ON CONFLICT DO NOTHING",
            &[
                &customer_id,
                &first_name,
                &last_name,
                &email,
                &phone,
                &city_id,
            ],
        ) {
            Ok(_) => stats.inserted += 1,
            Err(_) => stats.errors += 1,
        }
    }

    Ok(())
}
pub fn insert_products(
    client: &mut Client,
    data: &Vec<Vec<Value>>,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    for row in data {
        stats.processed += 1;

        let product_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let product_name = match row.get(1) {
            Some(v) => v.as_string(),
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let category_name = match row.get(2) {
            Some(Value::String(name)) if !name.is_empty() => name.clone(),
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let category_id: i32 = match client.query_one(
            "INSERT INTO categories (category_name)
             VALUES ($1)
             ON CONFLICT (category_name) DO UPDATE SET category_name = EXCLUDED.category_name
             RETURNING category_id",
            &[&category_name],
        ) {
            Ok(row) => row.get(0),
            Err(_) => {
                stats.errors += 1;
                continue;
            }
        };

        let price = match row.get(3) {
            Some(Value::Number(n)) => *n,
            _ => {
                stats.nulls += 1;
                0.0
            }
        };

        let stock = match row.get(4) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.nulls += 1;
                0
            }
        };

        let result = client.execute(
            "INSERT INTO products
                   (product_id, product_name, category_id, price, stock)
                   VALUES ($1, $2, $3, $4, $5)
                   ON CONFLICT (product_id) DO UPDATE
                   SET product_name = EXCLUDED.product_name,
                       category_id = EXCLUDED.category_id,
                       price = EXCLUDED.price,
                       stock = EXCLUDED.stock",
            &[&product_id, &product_name, &category_id, &price, &stock],
        );

        match result {
            Ok(_) => stats.inserted += 1,
            Err(_) => stats.errors += 1,
        }
    }

    Ok(())
}
pub fn insert_categories(
    client: &mut Client,
    dims: &DimensionStore,
) -> Result<(), Box<dyn std::error::Error>> {
    let categories = match dims.get("categories") {
        Some(c) => c,
        None => return Ok(()),
    };

    if categories.is_empty() {
        return Ok(()); // 🔥 important safety guard
    }

    for (name, id) in categories {
        client.execute(
            "INSERT INTO categories (category_id, category_name)
             VALUES ($1, $2)
             ON CONFLICT (category_name) DO NOTHING",
            &[&(*id as i32), name],
        )?;
    }

    Ok(())
}

pub fn insert_status(
    client: &mut Client,
    dims: &DimensionStore,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    let status_map = match dims.get("status") {
        Some(s) => s,
        None => return Ok(()),
    };

    for (name, id) in status_map {
        stats.processed += 1;

        match client.execute(
            "INSERT INTO status (status_id, status_name)
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
            &[&(*id as i32), name],
        ) {
            Ok(_) => stats.inserted += 1,
            Err(_) => stats.errors += 1,
        }
    }

    Ok(())
}

pub fn insert_orders(
    client: &mut Client,
    data: &Vec<Vec<Value>>,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    let customer_ids = load_customer_ids(client)?;

    for row in data {
        stats.processed += 1;

        let order_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let customer_id = match row.get(1) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let order_date = match row.get(2) {
            Some(v) => match NaiveDate::parse_from_str(&v.as_string(), "%Y-%m-%d") {
                Ok(d) => d,
                Err(_) => {
                    stats.skipped += 1;
                    continue;
                }
            },
            None => {
                stats.skipped += 1;
                continue;
            }
        };

        let status_id = match row.get(3) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        if !customer_ids.contains(&customer_id) {
            stats.skipped += 1;
            continue;
        }

        match client.execute(
            "INSERT INTO orders
             (order_id, customer_id, order_date, status_id)
             VALUES ($1,$2,$3,$4)
             ON CONFLICT DO NOTHING",
            &[&order_id, &customer_id, &order_date, &status_id],
        ) {
            Ok(_) => stats.inserted += 1,
            Err(_) => stats.errors += 1,
        }
    }

    Ok(())
}
pub fn insert_order_items(
    client: &mut Client,
    data: &Vec<Vec<Value>>,
    stats: &mut EtlStats,
) -> Result<(), Box<dyn Error>> {
    let orders = load_orders(client)?;
    let products = load_products(client)?;

    for row in data {
        stats.processed += 1;

        let order_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let product_id = match row.get(1) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.skipped += 1;
                continue;
            }
        };

        let quantity = match row.get(2) {
            Some(Value::Number(n)) => *n as i32,
            _ => {
                stats.nulls += 1;
                0
            }
        };

        let total_price = match row.get(3) {
            Some(Value::Number(n)) => *n,
            _ => {
                stats.nulls += 1;
                0.0
            }
        };

        if !orders.contains(&order_id) || !products.contains(&product_id) {
            stats.skipped += 1;
            continue;
        }

        match client.execute(
            "INSERT INTO order_details
             (order_id, product_id, quantity, total)
             VALUES ($1,$2,$3,$4)
             ON CONFLICT DO NOTHING",
            &[&order_id, &product_id, &quantity, &total_price],
        ) {
            Ok(_) => stats.inserted += 1,
            Err(_) => stats.errors += 1,
        }
    }

    Ok(())
}
