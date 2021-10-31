pub mod solver {

    use crate::ProblemInstance;

    pub fn solve(p: &ProblemInstance) -> String {
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
        let ret: String = recurse_combinations(0, dims, kv_tuple, partial_result, (&*p.t).to_vec(), (&*p.s).to_string());
        return ret;
    }

    pub fn recurse_combinations(current_dim: usize, dims: usize, kv_tuple: Vec<(usize, char, Vec<String>)>, mut partial_result: Vec<(char,String)>, t: Vec<String>, s: String) -> String {
        let mut ret = String::new();

        //If-statement after we've built one possible combination
        if current_dim >= dims {
            //Below represents one of all the possible combinations
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
                if !s.contains(&substring) {
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
            ret = recurse_combinations(current_dim + 1, dims, (&*kv_tuple).to_vec(), (&*partial_result).to_vec(), (&*t).to_vec(), (&*s).to_string());
            
            //Premature break in the case a possible combination was found
            if ret != "NO".to_string() {
                break;
            }
        }
        return ret;
    }
}