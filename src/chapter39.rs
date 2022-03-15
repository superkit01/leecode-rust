pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let tmp: Vec<i32>=Vec::new();

        Self::digui(&candidates, 0, target, &tmp, &mut result);

        return result;
    }

    pub fn digui(candidates: &Vec<i32>, i: i32, remaining: i32, tmp:&Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if i >= candidates.len() as i32 {
            return;
        }
        if remaining < 0 {
            return;
        }
        if remaining == 0 {
            result.push(tmp.to_vec());
            return;
        }
        
        let mut new_vec= tmp.clone();
        new_vec.push(candidates[i as usize]);

        Self::digui(candidates, i, remaining - candidates[i as usize], &new_vec, result);

        Self::digui(candidates, i+1, remaining, tmp, result);

  
    }
}
