pub mod solver {
    
    use crate::ProblemInstance;

    use itertools::Itertools;
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
    }
}