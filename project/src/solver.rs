pub mod solver {

    //TODO: In the case of a YES, your algorithm has to construct
    // a solution r1...rm and must output the solution
    
    use crate::ProblemInstance;

    use std::collections::HashMap;
    use std::collections::HashSet;

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

    pub fn solve_2(p: &ProblemInstance, possible_substrings: &HashMap<usize, HashSet<String>>) -> String {
        //kv_tuple is a triple of dimension size for the expansion, the character of the expansion, and the expansion
        let mut kv_tuple: Vec<(usize, char, Vec<String>)> = Vec::new();
        for (key, value) in &(p.r) {
            let val = &*value.clone();
            kv_tuple.push((value.len(), *key, val.to_vec()));
        }
        let dims = kv_tuple.len();
        let mut partial_result: Vec<(char,String)> = Vec::new();
        kv_tuple.sort_by(|a, b| (a.1).cmp(&b.1));
        for kv in &kv_tuple {
            partial_result.push((kv.1, "-".to_string()));
        }
        let ret: String = recurse_combinations(0, dims, kv_tuple, partial_result, (&*p.t).to_vec(), (&*p.s).to_string(), possible_substrings);
        return ret;
    }

    pub fn recurse_combinations(current_dim: usize, dims: usize, kv_tuple: Vec<(usize, char, Vec<String>)>, 
            mut partial_result: Vec<(char,String)>, t: Vec<String>, s: String, 
            possible_substrings: &HashMap<usize, HashSet<String>>) -> String {
        let mut ret = String::new();

        //If-statement after we've built one possible combination
        if current_dim >= dims {
            //Below represents one of all the possible combinations
            //println!("{:?}", partial_result);

            //For each t_i, build the represented string and see if it is contained in s
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
                if !does_s_contain(&substring, possible_substrings) {
                    return "NO".to_string();
                }
            }

            //It was contained, build the string
            let mut ret_builder = String::new();
            for pr in &partial_result {
                ret_builder.push_str(&(format!("{}:{}", pr.0, pr.1)).to_string());
                ret_builder.push_str("\n");
            }
            ret_builder.pop();
            return ret_builder;
        }

        //Recursive call to build all possible combinations
        for i in 0..kv_tuple[current_dim].0 {
            //Update the possible combination with a new update key value
            partial_result[current_dim].1 = (kv_tuple[current_dim].2)[i].clone();

            //Recurse and update the next expansions of a combination
            ret = recurse_combinations(current_dim + 1, dims, (&*kv_tuple).to_vec(), (&*partial_result).to_vec(), 
                    (&*t).to_vec(), (&*s).to_string(), possible_substrings);
            
            //Premature break in the case a possible combination was found
            if ret != "NO".to_string() {
                break;
            }
        }
        return ret;
    }

    fn does_s_contain(substring: &String, possible_substrings: &HashMap<usize, HashSet<String>>) -> bool {
        if !possible_substrings.contains_key(&substring.len()) {
            return false
        }

        possible_substrings[&substring.len()].contains(substring)
    }
}