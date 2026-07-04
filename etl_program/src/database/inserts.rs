use crate::config::Value;
use crate::dim_values::DimensionStore;
use postgres::Client;
use std::error::Error;

pub fn insert_countries_and_cities(
    client: &mut Client,
    dims: &DimensionStore,
) -> Result<(), Box<dyn Error>> {
    // ---------------- COUNTRIES ----------------
    if let Some(countries) = dims.get("countries") {
        for (country, id) in countries {
            client.execute(
                "INSERT INTO countries (country_id, country_name)
                 VALUES ($1, $2)
                 ON CONFLICT (country_name) DO NOTHING",
                &[&(*id as i32), country],
            )?;
        }
    }

    // ---------------- CITIES ----------------
    // expected key format: "city|country"
    if let Some(cities) = dims.get("cities") {
        for (key, city_id) in cities {
            let parts: Vec<&str> = key.split('|').collect();
            if parts.len() != 2 {
                continue; // invalid key format
            }

            let city = parts[0];
            let country = parts[1];

            let country_map = match dims.get("countries") {
                Some(m) => m,
                None => continue,
            };

            let country_id = match country_map.get(country) {
                Some(id) => *id,
                None => continue,
            };

            client.execute(
                "INSERT INTO cities (city_id, city_name, country_id)
                 VALUES ($1, $2, $3)
                 ON CONFLICT (city_name, country_id) DO NOTHING",
                &[&(*city_id as i32), &city, &(country_id as i32)],
            )?;
        }
    }

    Ok(())
}
pub fn insert_customers(
    client: &mut Client,
    data: &Vec<Vec<Value>>,
) -> Result<(), Box<dyn std::error::Error>> {
    for row in data {
        let customer_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => continue,
        };

        let first_name = match row.get(1) {
            Some(Value::String(s)) => s.as_str(),
            _ => "",
        };

        let last_name = match row.get(2) {
            Some(Value::String(s)) => s.as_str(),
            _ => "",
        };

        let email = match row.get(3) {
            Some(Value::String(s)) => s.as_str(),
            _ => "",
        };

        let phone: Option<&str> = match row.get(4) {
            Some(Value::String(s)) => Some(s.as_str()),
            _ => None,
        };

        let city_id = match row.get(5) {
            Some(Value::Number(n)) => *n as i32,
            _ => continue,
        };

        client.execute(
            "INSERT INTO customers
             (customer_id, first_name, last_name, email, phone, city_id)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (customer_id) DO NOTHING",
            &[
                &customer_id,
                &first_name,
                &last_name,
                &email,
                &phone,
                &city_id,
            ],
        )?;
    }

    Ok(())
}
pub fn insert_products(client: &mut Client, data: &Vec<Vec<Value>>) -> Result<(), Box<dyn Error>> {
    for row in data {
        let product_id = match row.get(0) {
            Some(Value::Number(n)) => *n as i32,
            _ => continue,
        };

        let product_name = match row.get(1) {
            Some(v) => v.as_string(),
            _ => continue,
        };

        let category_id = match row.get(2) {
            Some(Value::Number(id)) => *id as i32,
            _ => continue,
        };

        let price = row.get(3).map(|v| v.as_f64()).unwrap_or(0.0);
        let stock = row.get(4).map(|v| v.as_f64()).unwrap_or(0.0) as i32;

        let result = client.execute(
            "INSERT INTO products
             (product_id, product_name, category_id, price, stock)
             VALUES ($1, $2, $3, $4, $5)",
            &[&product_id, &product_name, &category_id, &price, &stock],
        );

        if let Err(e) = result {
            println!("DB INSERT ERROR: {:?}", e);
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
