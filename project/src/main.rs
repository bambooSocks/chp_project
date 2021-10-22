use std::collections::HashMap;
mod decoder;

use crate::decoder::decoder::decode;

#[derive(Debug)]
pub struct ProblemInstance {
    s: String,
    t: Vec<String>,
    r: HashMap<char, Vec<String>>
}

fn main() {
    let instance = decode();
    println!("{:?}", instance)
}
