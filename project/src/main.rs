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
    match instance {
        Some(p) => {
            let preprocessed = preprocess(&p);
            let result = solve(&preprocessed);
            if result == "NO".to_string() {
                println!("NO")
            }
            else {
                println!("{}", result)
            }
        },
        None => println!("NO")
    }
}
