pub mod decoder {

    use crate::ProblemInstance;

    use std::io;
    use std::io::prelude::*;
    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn decode() -> Option<ProblemInstance> {
        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines().peekable();

        let k: usize = match iterator.next() {
            Some(_k) => match _k.unwrap().trim().parse() {
                Ok(__k) => __k,
                Err(_) => return None
            },
            None => return None
        };

        let s: String = match iterator.next() {
            Some(_s) => _s.unwrap().trim().to_string(),
            None => return None
        };

        // Check if s consists only of letters a-z
        for s_i in s.chars() {
            if !matches!(s_i, 'a'..='z') {
                return None
            }
        }

        let mut t: Vec<String> = Vec::new();
        for _ in 0..k {
            let t_i = match iterator.next() {
                Some(_t_i) => _t_i.unwrap().trim().to_string(),
                None => return None
            };

            // Check if t_i consists only of letters A-Z and a-z
            for t_ii in t_i.chars() {
                if !matches!(t_ii, 'A'..='Z' | 'a'..='z') {
                    return None
                }
            }
            
            // Ignore duplicate t_is
            if !t.contains(&t_i) {
                t.push(t_i);
            }
        }
        let mut expansion: HashMap<char, Vec<String>> = HashMap::new();
        let mut key_set: HashSet<char> = HashSet::new();
        while iterator.peek().is_some() {
            let string_expansion: String = iterator.next().unwrap().unwrap().trim().to_string();

            // Check if expansion is in correct format
            let mut string_expansion_chars = string_expansion.chars();
            // First two characters must be a capital letter and a colon (:)
            if !matches!(string_expansion_chars.next().unwrap(), 'A'..='Z') {
                return None
            }
            if string_expansion_chars.next().unwrap() != ':' {
                return None
            }
            // The rest of line can be either letters a-z on a comma (,)
            // An extension must consist of at least one string of length at least 1
            if !matches!(match string_expansion_chars.next() {
                Some(c) => c,
                None => return None
            }, 'a'..='z') {
                return None
            }
            for _ in 3..string_expansion.len()-1 {
                if !matches!(string_expansion_chars.next().unwrap(), 'a'..='z' | ',') {
                    return None
                }
            }
            // Last character in line must be a letter a-z (it cannot be e.g. a comma)
            if string_expansion.len() > 3 {
                if !matches!(string_expansion_chars.next().unwrap(), 'a'..='z') {
                return None
                }
            }

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
                    // Check if the key is not duplicated
                    if key_set.contains(&c) {
                        return None
                    }
                    key_set.insert(c);
                    key = c;
                } else {
                    if c == ',' {
                        if !value.contains(&single_value) {
                            value.push(single_value);
                        }
                        single_value = String::new();
                        continue;
                    } else {
                        single_value.push(c);
                    }
                }
            }
            if !value.contains(&single_value) {
                value.push(single_value);
            }
            expansion.insert(key, value);
        }

        // Check if all the letters in strings t_i have their expansions
        for t_i in 0..t.len() {
            for letter in t[t_i].chars() {
                if matches!(letter, 'A'..='Z') {
                    if !expansion.contains_key(&letter) {
                        return None
                    }
                }
            }
        }
        Some(ProblemInstance {
            s,
            t,
            r: expansion
        })
    }
}