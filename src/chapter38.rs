pub struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return String::from("1");
        }
        return Self::digui(1, String::from("1"), n);
    }

    fn digui(i: i32, tmp: String, target: i32) -> String {
        if i >= target {
            return tmp;
        }
    
        let arr: Vec<char> = tmp.chars().collect();
        let mut count: i32 = 1;
        let mut temp_char: char = arr[0];
        let mut result = String::from("");
        for i in 1..arr.len() {
            if arr[i] == arr[i - 1] {
                count = count + 1;
            } else {
                result += &count.to_string();
                result += &temp_char.to_string();
                count=1;
                temp_char=arr[i];
            }
        }
        result += &count.to_string();
        result += &temp_char.to_string();
    
        Self::digui(i + 1, result, target)
    }
}

