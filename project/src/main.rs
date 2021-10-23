use std::collections::HashMap;
mod decoder;
mod solver;

use crate::decoder::decoder::decode;
use crate::solver::solver::solve;

#[derive(Debug)]
pub struct ProblemInstance {
    s: String,
    t: Vec<String>,
    r: HashMap<char, Vec<String>>
}

fn main() {
    let instance = decode();
    let result = solve(&instance);
    if result {
        println!("YES")
    }
    else {
        println!("NO")
    }
}
