use std::{collections::HashSet, io::Error};

use crate::config::DatasetType;

pub fn string_to_vec(value: Result<String, Error>) -> Result<Vec<String>, Error> {
    let v = value?;
    let list = v
        .split(',')
        .map(|s| s.to_string().replace("\n", ""))
        .collect();
    Ok(list)
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
