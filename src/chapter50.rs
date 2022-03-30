pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let tmp_n=n as i64;
        if tmp_n < 0 {
            return 1.0 / Self::square(x, -tmp_n) ;
        } else {
            return Self::square(x, tmp_n);
        }
    }

    pub fn square(result: f64, tmp: i64) -> f64 {
        if tmp == 0 {
            return 1.0;
        }

        if tmp % 2 == 1 {
            let y = Self::square(result, tmp / 2);
            return result * y * y;
        } else {
            let y = Self::square(result, tmp / 2);
            return y * y;
        }
    }
}
