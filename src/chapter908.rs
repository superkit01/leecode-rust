pub struct Solution {}

impl Solution {
    pub fn smallest_range_i(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        if len==1{
            return 0;
        }


        let mut mut_nums = nums;
        mut_nums.sort();


        let abs = mut_nums[len - 1] - mut_nums[0];

        if abs < 2 * k {
            return 0;
        } else {
            return abs - 2 * k;
        }
    }
}
