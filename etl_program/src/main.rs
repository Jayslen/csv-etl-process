mod config;
mod controller;
mod db;
mod dim_values;
use std::{env, fs::File, io::Error};

use std::io::{BufRead, BufReader};

use crate::parser::utils::string_to_vec;

mod parser;
fn main() -> Result<(), Error> {
    let path: Vec<String> = env::args().collect();

    let file = File::open(&path[1]).expect("File does not exist");
    let mut reader = BufReader::new(file);

    let mut raw_header = String::new();
    reader.read_line(&mut raw_header)?;
    let header = string_to_vec(Ok(raw_header)).unwrap();

    controller::controller(&header, &mut reader)?;

    Ok(())
}
