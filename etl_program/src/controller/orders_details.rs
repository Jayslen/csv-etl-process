use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

use crate::{
    config::{DatasetConfig, Value},
    dim_values::DimensionStore,
    parser::{mapping::map_rows, parse::parse_values},
    utils::string_to_vec,
};

pub fn process_order_items_csv(
    reader: &mut BufReader<File>,
    config: &DatasetConfig,
    dims: &mut DimensionStore,
    header: &Vec<String>,
) -> Result<Vec<Vec<Value>>, Error> {
    if !config.has_same_structure(header) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "CSV header does not match expected structure",
        ));
    }

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let arr = string_to_vec(Ok(line))?;

        let row_map = map_rows(header, &arr);

        let parsed = parse_values(&row_map, config, header, dims);

        rows.push(parsed);
    }

    Ok(rows)
}
