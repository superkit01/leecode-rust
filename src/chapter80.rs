pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len()==1 {
            return 1;
        }

        let mut i = 1;
        let mut tmp = (nums[0], 1);
        for j in 1..nums.len() {
            if nums[j] == tmp.0 {
                if tmp.1 < 2 {
                    tmp.1 += 1;
                    nums[i]=nums[j];
                    i += 1;
                }
            }else{
                tmp=(nums[j],1);
                nums[i]=nums[j];
                i+=1;
            }
        }

        return i as i32;
    }
}
