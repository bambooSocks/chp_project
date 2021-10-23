use std::collections::HashMap;
mod decoder;
mod preprocesser;
mod solver;

use crate::decoder::decoder::decode;
use crate::preprocesser::preprocesser::preprocess;
use crate::solver::solver::solve;

#[derive(Debug)]
pub struct ProblemInstance {
    s: String,
    t: Vec<String>,
    r: HashMap<char, Vec<String>>
}

fn main() {
    let instance = decode();
    let preprocessed = preprocess(&instance);
    let result = solve(&preprocessed);
    if result {
        println!("YES")
    }
    else {
        println!("NO")
    }
}
