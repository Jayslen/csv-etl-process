use std::collections::HashMap;

#[derive(Debug)]
pub struct DimensionStore {
    pub countries: HashMap<String, usize>,
    pub cities: HashMap<(String, String), usize>,
    pub country_counter: usize,
    pub city_counter: usize,
}

impl DimensionStore {
    pub fn new() -> Self {
        Self {
            countries: HashMap::new(),
            cities: HashMap::new(),
            country_counter: 1,
            city_counter: 1,
        }
    }

    pub fn get_or_create_country(&mut self, name: &str) -> usize {
        if let Some(id) = self.countries.get(name) {
            *id
        } else {
            let id = self.country_counter;
            self.country_counter += 1;
            self.countries.insert(name.to_string(), id);
            id
        }
    }
    pub fn get_or_create_city(&mut self, city: &str, country: &str) -> usize {
        let key = (city.to_string(), country.to_string());

        if let Some(id) = self.cities.get(&key) {
            *id
        } else {
            let id = self.city_counter;
            self.city_counter += 1;

            self.cities.insert(key, id);
            id
        }
    }
}
