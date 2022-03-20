pub struct Solution {}

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len()==0 {
            return 0;
        }

        let mut i = 0;
        let mut j = 0;
        while i < nums.len() && j < nums.len() {
            if nums[j] == val {
                j += 1;
            } else {
                nums[i] = nums[j];
                i += 1;
                j += 1;
            }
        }

        return i as i32;
    }
}
