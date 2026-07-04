mod config;
mod controller;
mod database;
mod dim_values;
mod parser;
mod utils;

use crate::controller::entry::entry_point;
use std::io::{BufRead, BufReader};
use std::{env, fs::File, io::Error};
use utils::string_to_vec;

fn main() -> Result<(), Error> {
    let path: Vec<String> = env::args().collect();

    let file = File::open(&path[1]).expect("File does not exist");
    let mut reader = BufReader::new(file);

    let mut raw_header = String::new();
    reader.read_line(&mut raw_header)?;
    let header = string_to_vec(Ok(raw_header)).unwrap();
    entry_point(&header, &mut reader)?;

    Ok(())
}
