pub mod preprocesser {
    
    use crate::ProblemInstance;

    use std::collections::HashMap;
    use std::collections::HashSet;

    pub fn preprocess(p: &ProblemInstance) -> ProblemInstance {

        //Remove expansion letters (or complete expansions) which do not occur in s or expansions which do not occur in the t_i's
        let mut expansion: HashMap<char, Vec<String>> = HashMap::new();
        for (key, value) in &(p.r) {

            let mut dont_add_kv: bool = true;
            for ti in &(p.t) {
                let mut break_outer: bool = false;
                for c in ti.chars() {
                    if c == *key {
                        dont_add_kv = false;
                        break_outer = true;
                        break;
                    }
                }
                if break_outer {
                    break;
                }
            }
            if dont_add_kv {
                continue;
            }

            let mut val: Vec<String> = Vec::new();
            for letters in &*value {
                if (&(p.s)).contains(letters) {
                    let copy = letters.clone();
                    val.push(copy);
                }
            }
            if val.len() != 0 {
                //Also sort so smallest (length) letters appear first in vector
                val.sort_by(|a, b| a.len().cmp(&b.len()));
                expansion.insert(*key, val);
            }
        }
        
        //Sort t, such higher amounts of lowercase letters in t_i appears first
        let mut t: Vec<String> = ((&*(p.t)).to_vec()).clone();
        for i in 0..t.len() {
            for j in 0..t.len() - i - 1 {
                let mut lowercase_characters: String = t[j].clone();
                lowercase_characters.retain(|l| l.is_lowercase());
                let mut lowercase_characters_2: String = t[j+1].clone();
                lowercase_characters_2.retain(|l| l.is_lowercase());
                if lowercase_characters.len() < lowercase_characters_2.len() {
                    let temp: String = t[j].clone();
                    let temp_2: String = t[j+1].clone();
                    t[j] = temp_2;
                    t[j+1] = temp;
                }
            }
        }
        //Would be much easier to do the following, but I cannot get it to work
        //t.sort_by(|a, b| (b.retain(|l| l.is_lowercase()).collect()).len().cmp((&a.retain(|l| l.is_lowercase()).collect()).len()));

        ProblemInstance {
            s: (&*(p.s)).to_string(),
            t: t,
            r: expansion
        }
    }

    pub fn generate_possible_substrings(p: &ProblemInstance) -> HashMap<usize, HashSet<String>> {
        let mut result: HashMap<usize, HashSet<String>> = HashMap::new();
        // For 1 <= i <= len(s) create a set of all substrings of s of length=i
        for i in 1..p.s.len()+1 {
            result.insert(i, create_substrings_of_length(&p.s, i));
        }
        result
    }

    fn create_substrings_of_length(s: &String, length: usize) -> HashSet<String> {
        let mut result: HashSet<String> = HashSet::new();
        for i in 0..s.len()-length+1 {
            result.insert(s[i..i+length].to_owned());
        }
        result
    }
}