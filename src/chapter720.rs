use std::collections::HashSet;
use std::cmp::Ordering;

pub struct Solution {}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut mut_words=words;

        let mut set: HashSet<String> = HashSet::new();
        mut_words.sort();

        for elem in mut_words.iter() {
            if elem.len() == 1 {
                set.insert(elem.clone());
                continue;
            }
            if set.contains(&elem[0..elem.len() - 1]) {
                set.insert(elem.clone());
                continue;
            }
        }


       let max= set.iter().max_by(|x, y| {
            let xl = x.len() as i32;
            let yl = y.len() as i32;
            if xl>yl {
                return Ordering::Greater;
            }else if xl<yl {
                return Ordering::Less;
            } else{
                return y.cmp(&x);
            }  
        });

        match max {
            None=> {return String::from("");}
            Some(v)=>{return v.clone();}
        }
    }
}
