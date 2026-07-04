#[derive(Default)]
pub struct EtlStats {
    pub processed: usize,
    pub inserted: usize,
    pub rejected: usize,
    pub errors: Vec<String>,
}

impl EtlStats {
    pub fn print_summary(&self, name: &str) {
        println!("\n=== ETL Summary: {} ===", name);
        println!("Processed: {}", self.processed);
        println!("Inserted: {}", self.inserted);
        println!("Rejected: {}", self.rejected);

        if !self.errors.is_empty() {
            println!("Errors: {}", self.errors.len());
        }
    }
}
