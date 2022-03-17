pub struct Solution {}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();

        if nums.len() == 1 {
            result.push(nums);
            return result;
        }
        let mut tmp: Vec<i32> = Vec::new();
        Self::dp(&mut tmp, nums, &mut result);

        return result;
    }

    pub fn dp(tmp: &mut Vec<i32>, rest: Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if rest.len() == 0 {
            result.push(tmp.clone());
            return;
        }

        for (index, elem) in rest.iter().enumerate() {
            let mut tmp_v = tmp.clone();
            tmp_v.push(elem.clone());
            let mut rest_v = rest.clone();
            rest_v.remove(index);
            Self::dp(&mut tmp_v, rest_v, result)
        }
    }
}
