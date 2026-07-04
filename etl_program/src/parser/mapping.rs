use std::collections::HashMap;

pub fn map_rows(header: &Vec<String>, row: &Vec<String>) -> HashMap<String, String> {
    let mut map = HashMap::new();

    for (h, v) in header.iter().zip(row.iter()) {
        map.insert(h.clone(), v.clone());
    }

    map
}
