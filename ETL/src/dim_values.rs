use std::collections::HashMap;

#[derive(Debug)]
pub struct DimensionStore {
    pub dims: HashMap<String, HashMap<String, usize>>,
    pub counters: HashMap<String, usize>,
}

impl DimensionStore {
    pub fn new() -> Self {
        Self {
            dims: HashMap::new(),
            counters: HashMap::new(),
        }
    }

    pub fn get_or_create(&mut self, dim: &str, key: String) -> usize {
        let map = self
            .dims
            .entry(dim.to_string())
            .or_insert_with(HashMap::new);

        let counter = self.counters.entry(dim.to_string()).or_insert(1);

        if let Some(id) = map.get(&key) {
            *id
        } else {
            let id = *counter;
            *counter += 1;
            map.insert(key, id);
            id
        }
    }

    pub fn get(&self, dim: &str) -> Option<&HashMap<String, usize>> {
        self.dims.get(dim)
    }
}
