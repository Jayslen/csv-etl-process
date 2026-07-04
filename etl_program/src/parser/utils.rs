use crate::config::{DataType, DatasetConfig};
use crate::dim_values::DimensionStore;
use std::{collections::HashMap, io::Error};

#[derive(Debug)]
pub enum Value {
    Number(usize),
    String(String),
    Null,
}

pub fn string_to_vec(value: Result<String, Error>) -> Result<Vec<String>, Error> {
    let v = value?;
    let list = v
        .split(',')
        .map(|s| s.to_string().replace("\n", ""))
        .collect();
    Ok(list)
}

pub fn map_rows(header: &Vec<String>, row: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (h, v) in header.iter().zip(row.iter()) {
        map.insert(h.clone(), v.clone());
    }

    map
}

pub fn parse_data(
    map_row: &HashMap<String, String>,
    config: &DatasetConfig,
    header: &[String],
    dims: &mut DimensionStore,
) -> Vec<Value> {
    let mut parsed_row = Vec::new();

    for col_name in header {
        let col_config = config.cols.get(col_name);

        let raw_value = map_row.get(col_name);

        if col_name == "Country" {
            let id = raw_value
                .map(|v| dims.get_or_create_country(v))
                .unwrap_or(0);

            parsed_row.push(Value::Number(id));
            continue;
        }

        if col_name == "City" {
            let id = raw_value
                .map(|city| {
                    let country = map_row.get("Country").expect("Country must exist for City");

                    dims.get_or_create_city(city, country)
                })
                .unwrap_or(0);

            parsed_row.push(Value::Number(id));
            continue;
        }

        let transformed = match (raw_value, col_config) {
            (Some(value), Some(cfg)) => {
                let value = match cfg.transformation {
                    Some(f) => f(value),
                    None => value.clone(),
                };

                Some(value)
            }
            _ => None,
        };

        let final_value = match (transformed, col_config) {
            (Some(v), Some(cfg)) => match cfg.data_type {
                DataType::Number => match v.parse::<usize>() {
                    Ok(num) => Value::Number(num),
                    Err(_) => Value::Null,
                },

                DataType::String => {
                    if let Some(max) = cfg.length {
                        if v.len() <= max {
                            Value::String(v)
                        } else {
                            Value::Null
                        }
                    } else {
                        Value::String(v)
                    }
                }
            },

            _ => Value::Null,
        };

        parsed_row.push(final_value);
    }

    parsed_row
}
