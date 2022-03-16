
pub struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut i:usize =0;
        let mut j=i+1;
        while i < nums.len() -1  && j< nums.len()  {
            if nums[i] != nums[j]{
                i+=1;
                nums[i]=nums[j];
                j+=1;
                continue;
            }else{
                j+=1;
            }
        }
        return i as i32 +1;

    }
}