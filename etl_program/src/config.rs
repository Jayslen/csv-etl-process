use postgres::{Client, NoTls};
use std::collections::HashMap;

pub fn connection() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=sales",
        NoTls,
    )?;
    return Ok(client);
}

pub struct ConfigData {
    datasets: HashMap<String, Vec<String>>,
}

pub enum Dataset {
    Customers,
    OrdersDetails,
    Orders,
    Products,
}

impl ConfigData {
    pub fn init() -> ConfigData {
        let mut data = ConfigData {
            datasets: HashMap::new(),
        };
        data.datasets.insert(
            "customers".to_string(),
            vec![
                "CustomerID".to_string(),
                "FirstName".to_string(),
                "LastName".to_string(),
                "Email".to_string(),
                "Phone".to_string(),
                "City".to_string(),
                "Country".to_string(),
            ],
        );
        data.datasets.insert(
            "orders_details".to_string(),
            vec![
                "OrderID".to_string(),
                "ProductID".to_string(),
                "Quantity".to_string(),
                "TotalPrice".to_string(),
            ],
        );
        data.datasets.insert(
            "orders".to_string(),
            vec![
                "OrderID".to_string(),
                "CustomerID".to_string(),
                "OrderDate".to_string(),
                "Status".to_string(),
            ],
        );
        data.datasets.insert(
            "products".to_string(),
            vec![
                "ProductID".to_string(),
                "ProductName".to_string(),
                "Category".to_string(),
                "Price".to_string(),
                "Stock".to_string(),
            ],
        );
        return data;
    }

    pub fn which_dataset(&self, header: &Vec<String>) -> Option<Dataset> {
        for (key, value) in self.datasets.iter() {
            if same_structure(header, value) {
                return Some(match key.as_str() {
                    "customers" => Dataset::Customers,
                    "orders_details" => Dataset::OrdersDetails,
                    "orders" => Dataset::Orders,
                    "products" => Dataset::Products,
                    _ => continue,
                });
            }
        }
        None
    }
}

fn same_structure(header: &Vec<String>, dataset: &Vec<String>) -> bool {
    if header.len() != dataset.len() {
        return false;
    }
    for (h, d) in header.iter().zip(dataset.iter()) {
        if h != d {
            return false;
        }
    }
    return true;
}
