use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().peekable();
    let k: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let s: String = iterator.next().unwrap().unwrap().trim().to_string();
    let mut t: Vec<String> = Vec::new();
    for _ in 0..k {
        t.push(iterator.next().unwrap().unwrap().trim().to_string());
    }
    let mut expansion: HashMap<char, Vec<String>> = HashMap::new();
    let mut j: u32 = 0;
    while iterator.peek().is_some() {
        let string_expansion: String = iterator.next().unwrap().unwrap().trim().to_string();
        let mut key: char = '-';
        let mut value: Vec<String> = Vec::new();
        let mut single_value: String = String::new();
        let mut is_value: bool = false;
        for c in string_expansion.chars() {
            if !is_value {
                if c == ':' {
                    is_value = true;
                    continue;
                }
                key = c;
            } else {
                if c == ',' {
                    value.push(single_value);
                    single_value = String::new();
                    continue;
                } else {
                    single_value.push(c);
                }
            }
        }
        value.push(single_value);
        expansion.insert(key, value);
    }

    /*
    //Now we have the results in the respective variables (k, s, t, expansions)
    //Printing decoder results
    println!("Results. k: {}, s: {}, t: {:?}. Expansion on following lines:", k, s, t);
    let mut its = expansion.iter().collect::<Vec<_>>();
    for (m, n) in &its {
        println!("{}: {:?}", m, n);
    }
    */
}
