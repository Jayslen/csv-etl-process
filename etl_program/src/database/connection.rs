use postgres::{Client, NoTls};

pub fn connect() -> Result<Client, Box<dyn std::error::Error>> {
    let client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=sales",
        NoTls,
    )?;
    return Ok(client);
}
