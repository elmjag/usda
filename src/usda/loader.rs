///! Load and parse USDA file
use super::{parser, proc};
use crate::Prim;
use std::fs;

pub fn load_usda_file(file_path: &str) -> Vec<Prim> {
    let text = fs::read_to_string(file_path).unwrap();
    let pairs = parser::parse(&text);

    proc::process(pairs)
}
