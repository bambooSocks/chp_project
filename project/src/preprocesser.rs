pub mod preprocesser {
    
    use crate::ProblemInstance;

    //use itertools::Itertools;
    //use std::iter::FromIterator;
    use std::collections::HashMap;

    pub fn preprocess(p: &ProblemInstance) -> ProblemInstance {

        //Remove expansion letters (or complete expansions) which do not occur in s
        let mut expansion: HashMap<char, Vec<String>> = HashMap::new();
        for (key, value) in &(p.r) {
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
        ProblemInstance {
            s: (&*(p.s)).to_string(),
            t: (&*(p.t)).to_vec(),
            r: expansion
        }
    }
}