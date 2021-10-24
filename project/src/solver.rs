pub mod solver {

    //TODO: In the case of a YES, your algorithm has to construct
    // a solution r1...rm and must output the solution
    
    use crate::ProblemInstance;

    /*use itertools::Itertools;
    use std::iter::FromIterator;
    use std::collections::HashMap;

    pub fn solve(p: &ProblemInstance) -> bool {

        let letters = Vec::from_iter(p.r.keys());
        let mut letters_order_dict = HashMap::new();
        for id in 0..letters.len() {
            letters_order_dict.insert(letters[id], id);
        }

        let multi_prod = letters.into_iter().map(|l| p.r.get(l).cloned().unwrap_or(Vec::new()))
            .multi_cartesian_product();
        for option in multi_prod {
            let mut found = true;
            for t_i in &p.t {
                let substring = t_i.chars().map(|letter| 
                    match letters_order_dict.get(&letter) {
                        Some(i) => option.get(*i).cloned().unwrap_or(String::new()),
                        _ => if matches!(letter, 'a'..='z') { letter.to_string() } else { panic!("letter") },
                    }
                ).fold(String::new(), |a, b| a + &b);
                if !p.s.contains(&substring) {
                    found = false;
                    break;
                }
            }
            if found {
                return true
            }
        }
        false
    }*/

    pub fn solve_2(p: &ProblemInstance) -> String {
        let mut keys: Vec<char> = Vec::new();
        let mut values: Vec<Vec<String>> = Vec::new();
        let mut dim_sizes: Vec<usize> = Vec::new();
        for (key, value) in &(p.r) {
            keys.push(*key);
            let val = &*value.clone();
            values.push(val.to_vec());
            dim_sizes.push(value.len());
        }
        let dims = keys.len();
        //println!("dims: {}", dims);
        //println!("keys: {:?}", keys);
        //println!("values: {:?}", values);
        //println!("dim_sizes: {:?}", dim_sizes);
        let partial_result: Vec<(char,String)> = Vec::new();
        let ret: String = recurse_combinations(0, dims, dim_sizes, keys, values, partial_result, (&*p.t).to_vec(), (&*p.s).to_string());
        return ret;
    }

    pub fn recurse_combinations(current_dim: usize, dims: usize, dim_sizes: Vec<usize>, keys: Vec<char>, values: Vec<Vec<String>>, mut partial_result: Vec<(char,String)>, t: Vec<String>, s: String) -> String {
        let mut ret = String::new();
        if current_dim >= dims {
            //println!("{:?}", partial_result);
            for ti in t {
                let mut substring = String::new();
                for c in ti.chars() {
                    if c.is_lowercase() {
                        substring.push(c);
                    } else {
                        for pr in &partial_result {
                            if pr.0 == c {
                                for c2 in pr.1.chars() {
                                    substring.push(c2);
                                }
                                break;
                            }
                        }
                    }
                }
                if !s.contains(&substring) {
                    return "NO".to_string();
                }
            }
            let mut ret_builder = String::new();
            for pr in &partial_result {
                ret_builder.push_str(&(format!("{}:{}", pr.0, pr.1)).to_string());
                ret_builder.push_str("\n");
            }
            ret_builder.pop();
            return ret_builder;
        }
        for i in 0..dim_sizes[current_dim] {
            let mut add: bool = true;
            for j in 0..partial_result.len() {
                if partial_result[j].0 == keys[current_dim] {
                    let val: String = values[current_dim][i].clone();
                    partial_result[j].1 = val;
                    add = false;
                    break;
                }
            }
            if add {
                let val: String = values[current_dim][i].clone();
                partial_result.push((keys[current_dim], val));
            }
            ret = recurse_combinations(current_dim + 1, dims, (&*dim_sizes).to_vec(), (&*keys).to_vec(), (&*values).to_vec(), (&*partial_result).to_vec(), (&*t).to_vec(), (&*s).to_string());
            if ret != "NO".to_string() {
                break;
            }
        }
        return ret;
    }
}