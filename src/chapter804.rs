pub struct Solution {}

use std::collections::{HashMap,HashSet};

impl Solution {
    //[".-","-...","-.-.","-..",".","..-.","--.",
    //"....","..",".---","-.-",".-..","--","-.",
    //"---",".--.","--.-",
    //".-.","...","-",
    //"..-","...-",".--",
    //"-..-","-.--","--.."]

    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let map: HashMap<char, String> = HashMap::from([
            ('a', String::from(".-")),
            ('b', String::from("-...")),
            ('c', String::from("-.-.")),
            ('d', String::from("-..")),
            ('e', String::from(".")),
            ('f', String::from("..-.")),
            ('g', String::from("--.")),
            ('h', String::from("....")),
            ('i', String::from("..")),
            ('j', String::from(".---")),
            ('k', String::from("-.-")),
            ('l', String::from(".-..")),
            ('m', String::from("--")),
            ('n', String::from("-.")),
            ('o', String::from("---")),
            ('p', String::from(".--.")),
            ('q', String::from("--.-")),
            ('r', String::from(".-.")),
            ('s', String::from("...")),
            ('t', String::from("-")),
            ('u', String::from("..-")),
            ('v', String::from("...-")),
            ('w', String::from(".--")),
            ('x', String::from("-..-")),
            ('y', String::from("-.--")),
            ('z', String::from("--..")),
        ]);

        let mut  result: HashSet<String> = HashSet::new();
        
        for word in words{
            let c:Vec<char>=word.as_str().chars().collect();


           let temp_vec:Vec<String>=c.iter().map(|x| map.get(x).unwrap().to_owned()).collect();
           
           let mut temp=String::from("");

            for ele in temp_vec{
                temp+=ele.as_str();
            }

            result.insert(temp);

        }



        return result.len() as i32;
    }
}
