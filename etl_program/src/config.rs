use std::collections::HashMap;

#[derive(Debug)]
pub enum DataType {
    Number,
    String,
}

#[derive(Debug)]
pub struct Cols {
    pub data_type: DataType,
    pub length: Option<usize>,
    pub transformation: Option<fn(&String) -> String>,
}

pub struct DatasetConfig {
    pub cols: HashMap<String, Cols>,
}

impl DatasetConfig {
    pub fn customer_config() -> DatasetConfig {
        let mut config = HashMap::new();

        config.insert(
            "CustomerID".to_string(),
            Cols {
                data_type: DataType::Number,
                length: None,
                transformation: None,
            },
        );

        config.insert(
            "FirstName".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(50),
                transformation: None,
            },
        );

        config.insert(
            "LastName".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(50),
                transformation: None,
            },
        );

        config.insert(
            "Email".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(100),
                transformation: None,
            },
        );

        config.insert(
            "Phone".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(10),
                transformation: Some(remove_special_chars),
            },
        );

        config.insert(
            "City".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(50),
                transformation: None,
            },
        );

        config.insert(
            "Country".to_string(),
            Cols {
                data_type: DataType::String,
                length: Some(50),
                transformation: None,
            },
        );

        DatasetConfig { cols: config }
    }

    pub fn has_same_structure(&self, header: &Vec<String>) -> bool {
        let mut is_valid = true;
        if self.cols.len() != header.len() {
            is_valid = false
        }

        for col in header {
            let has_col = self.cols.get(col);

            if has_col.is_none() {
                is_valid = false;
                break;
            }
        }

        is_valid
    }
}
fn remove_special_chars(value: &String) -> String {
    value.chars().filter(|c| c.is_ascii_digit()).collect()
}
