pub struct Solution {}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a_chars = a.as_bytes();
        let b_chars = b.as_bytes();
        let mut add = 0;
        let mut i = a.len() as i32 - 1;
        let mut j = b.len() as i32 - 1;
        let mut result = String::from("");
        while i >= 0 || j >= 0  ||add > 0 {
            let k1 = if i >= 0 { a_chars[i as usize] - ('0' as u8) } else { 0 };
            let k2 = if j >= 0 { b_chars[j as usize] - ('0' as u8) } else { 0 };

            let sum = k1 + k2 + add;
            result = (sum % 2).to_string()+&result;
            add = sum / 2;
            j -= 1;
            i -= 1;
        }

        return result;
    }
}
