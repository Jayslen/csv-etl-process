use std::collections::HashMap;

use crate::{
    config::{DataType, DatasetConfig, Value},
    dim_values::DimensionStore,
};

pub fn parse_values(
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
                .map(|v| dims.get_or_create("countries", v.clone()))
                .unwrap_or(0);

            parsed_row.push(Value::Number(id as f64));
            continue;
        }

        if col_name == "City" {
            let id = raw_value
                .map(|city| {
                    let country = map_row.get("Country").expect("Country must exist for City");

                    let key = format!("{}|{}", city, country);

                    dims.get_or_create("cities", key)
                })
                .unwrap_or(0);

            parsed_row.push(Value::Number(id as f64));
            continue;
        }

        if col_name == "Category" {
            let id = raw_value
                .map(|v| dims.get_or_create("categories", v.clone()))
                .unwrap_or(0);

            parsed_row.push(Value::Number(id as f64));
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
                DataType::Number => {
                    let cleaned = v.trim().replace(",", ".").replace("$", "");

                    match cleaned.parse::<f64>() {
                        Ok(num) => Value::Number(num),
                        Err(_) => Value::Null,
                    }
                }
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
