use postgres::{Client, NoTls};

use crate::dim_values::DimensionStore;

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
