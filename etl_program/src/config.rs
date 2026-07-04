use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum DatasetType {
    Customers,
    OrderItems,
    Orders,
    Products,
    Unknown,
}

#[derive(Debug)]
pub enum DataType {
    Number,
    String,
}

#[derive(Debug)]
pub enum Value {
    Number(usize),
    String(String),
    Null,
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

pub fn detect_dataset(header: &Vec<String>) -> DatasetType {
    let header_set: HashSet<&str> = header.iter().map(|s| s.as_str()).collect();

    let customers = HashSet::from([
        "CustomerID",
        "FirstName",
        "LastName",
        "Email",
        "Phone",
        "City",
        "Country",
    ]);

    let order_items = HashSet::from(["OrderID", "ProductID", "Quantity", "TotalPrice"]);

    let orders = HashSet::from(["OrderID", "CustomerID", "OrderDate", "Status"]);

    let products = HashSet::from(["ProductID", "ProductName", "Category", "Price", "Stock"]);

    if header_set == customers {
        DatasetType::Customers
    } else if header_set == order_items {
        DatasetType::OrderItems
    } else if header_set == orders {
        DatasetType::Orders
    } else if header_set == products {
        DatasetType::Products
    } else {
        DatasetType::Unknown
    }
}
