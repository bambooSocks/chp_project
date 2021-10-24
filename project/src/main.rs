use std::collections::HashMap;
mod decoder;
mod preprocesser;
mod solver;

use crate::decoder::decoder::decode;
use crate::preprocesser::preprocesser::preprocess;
//use crate::solver::solver::solve;
use crate::solver::solver::solve_2;

#[derive(Debug)]
pub struct ProblemInstance {
    s: String,
    t: Vec<String>,
    r: HashMap<char, Vec<String>>
}

fn main() {
    let instance = decode();
    let preprocessed = preprocess(&instance);
    let result = solve_2(&preprocessed);
    if result {
        println!("YES")
    }
    else {
        println!("NO")
    }
}
