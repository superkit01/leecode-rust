pub struct Solution {}

impl Solution {
    //ERROR
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s3.len() != s1.len() + s2.len() {
            return false;
        }

        let s1_chars: Vec<char> = s1.chars().collect();
        let s3_chars: Vec<char> = s3.chars().collect();
        let mut j = 0;
        let mut rest: Vec<char> = Vec::new();

        for i in 0..s3_chars.len() {
            if j<s1_chars.len() && s3_chars[i] == s1_chars[j]  {
                j += 1;
            }else{
                rest.push(s3_chars[i])
            }
        }

        if j!=s1.len() || rest.iter().collect::<String>() != s2{
            return false
        }
        return true;
    }
}
