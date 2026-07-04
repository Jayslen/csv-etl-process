use postgres::Client;
use std::collections::HashSet;
use std::error::Error;

pub fn load_customer_ids(client: &mut Client) -> Result<HashSet<i32>, Box<dyn Error>> {
    let mut set = HashSet::new();

    for row in client.query("SELECT customer_id FROM customers", &[])? {
        let id: i32 = row.get(0);
        set.insert(id);
    }

    Ok(set)
}

pub fn load_orders(client: &mut Client) -> Result<HashSet<i32>, Box<dyn Error>> {
    let mut set = HashSet::new();

    for row in client.query("SELECT order_id FROM orders", &[])? {
        set.insert(row.get(0));
    }

    Ok(set)
}

pub fn load_products(client: &mut Client) -> Result<HashSet<i32>, Box<dyn Error>> {
    let mut set = HashSet::new();

    for row in client.query("SELECT product_id FROM products", &[])? {
        set.insert(row.get(0));
    }

    Ok(set)
}
