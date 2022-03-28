pub struct Solution {}

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut tmp: i32 = n / 2;
        let mut v = n % 2;
        while tmp > 0 {
            let yu = tmp % 2;
            tmp = tmp / 2;

            if v == yu {
                return false;
            }
            v = yu;
        }
        return true;
    }
}
