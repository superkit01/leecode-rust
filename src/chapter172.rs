use std::cmp::min;

pub struct Solution {}

impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut factor2 = 0;
        let mut factor5 = 0;
        'outer: for i in 1..n + 1 {
            let mut j = i;
            while j > 0 {
                if j % 2 == 0 {
                    factor2 += 1;
                    j /= 2;
                } else if j % 5 == 0 {
                    factor5 += 1;
                    j /= 5;
                } else {
                    continue 'outer;
                }
            }
        }
        return min(factor2, factor5);
    }
}
