use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // TODO 全局变量
        let cache: HashMap<String, i32> = HashMap::from([
            (String::from("I"), 1),
            (String::from("IV"), 4),
            (String::from("V"), 5),
            (String::from("IX"), 9),
            (String::from("X"), 10),
            (String::from("XL"), 40),
            (String::from("L"), 50),
            (String::from("XC"), 90),
            (String::from("C"), 100),
            (String::from("CD"), 400),
            (String::from("D"), 500),
            (String::from("CM"), 900),
            (String::from("M"), 1000),
        ]);

        let mut tmp = s;
        let mut result: i32 = 0;
        while tmp.len() > 0 {
            if tmp.len() > 1 {
                let tmp_char = String::from(&tmp[0..2]);
                if cache.contains_key(&tmp_char) {
                    result += cache.get(&tmp_char).unwrap();
                    tmp = String::from(&tmp[2..]);
                    continue;
                }
            }
            let tmp_char = String::from(&tmp[0..1]);

            if cache.contains_key(&tmp_char) {
                result += cache.get(&tmp_char).unwrap();
                tmp = String::from(&tmp[1..]);
            }
        }
        return result;
    }
}
