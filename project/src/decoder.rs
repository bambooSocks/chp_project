pub mod decoder {

    use crate::ProblemInstance;

    use std::io;
    use std::io::prelude::*;
    use std::collections::HashMap;

    pub fn decode() -> ProblemInstance {
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines().peekable();
        let k: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
        let s: String = iterator.next().unwrap().unwrap().trim().to_string();
        let mut t: Vec<String> = Vec::new();
        for _ in 0..k {
            let t_i = iterator.next().unwrap().unwrap().trim().to_string();
            if !t.contains(&t_i) {
                t.push(t_i);
            }
        }
        let mut expansion: HashMap<char, Vec<String>> = HashMap::new();
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
        ProblemInstance {
            s,
            t,
            r: expansion
        }
    }
}