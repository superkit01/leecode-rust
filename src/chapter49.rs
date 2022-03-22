pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for e in strs.iter() {
            let mut chars: Vec<char> = e.chars().collect();
            chars.sort_by(|a, b| b.cmp(&a));
            let value = map.entry(chars.iter().collect()).or_insert(Vec::new());
            value.push(e.clone());
        }
        
        for e in map.iter(){
            result.push(e.1.clone());
        }
        return result;
    }
}
