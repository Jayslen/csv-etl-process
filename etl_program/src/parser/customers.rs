use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

use crate::config::DatasetConfig;
use crate::dim_values::DimensionStore;
use crate::parser::utils::{Value, map_rows, parse_data, string_to_vec};

pub fn process_customers_csv(
    reader: &mut BufReader<File>,
    config: &DatasetConfig,
    dims: &mut DimensionStore,
    header: &Vec<String>,
) -> Result<Vec<Vec<Value>>, Error> {
    if !config.has_same_structure(&header) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "CSV header does not match expected structure",
        ));
    }

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let arr = string_to_vec(Ok(line))?;

        let row_map = map_rows(&header, &arr);

        let parsed = parse_data(&row_map, config, &header, dims);

        rows.push(parsed);
    }

    Ok(rows)
}
