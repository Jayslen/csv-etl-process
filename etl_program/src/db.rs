use postgres::{Client, NoTls};

use crate::{dim_values::DimensionStore, parser::utils::Value};

pub fn connection() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=sales",
        NoTls,
    )?;
    return Ok(client);
}

pub fn insert_countries_and_cities(
    client: &mut Client,
    dims: &DimensionStore,
) -> Result<(), Box<dyn std::error::Error>> {
    for (country, id) in &dims.countries {
        client.execute(
            "INSERT INTO countries (country_id, country_name) VALUES ($1, $2)
             ON CONFLICT (country_name) DO NOTHING",
            &[&(*id as i32), country],
        )?;
    }

    for ((city, country), city_id) in &dims.cities {
        let country_id = dims.countries.get(country).expect("Country must exist");

        client.execute(
            "INSERT INTO cities (city_id, city_name, country_id) VALUES ($1, $2, $3)
             ON CONFLICT (city_name, country_id) DO NOTHING",
            &[&(*city_id as i32), city, &(*country_id as i32)],
        )?;
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
