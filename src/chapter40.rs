pub struct Solution {}

impl Solution {
    // let candidates = vec![10,1,2,7,6,1,5];  //8
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let tmp: Vec<i32> = Vec::new();

        let mut sorted_candidates = candidates.clone();
        sorted_candidates.sort();

        Self::digui(&sorted_candidates, 0, target, &tmp, &mut result);

        result.dedup_by(|a, b| {
            if a.len() != b.len() {
                return false;
            };

            let mut i = 0;
            while i < a.len() {
                if a[i] != b[i] {
                    return false;
                }
                i += 1;
            }
            return true;
        });

        return result;
    }

    pub fn digui(
        candidates: &Vec<i32>,
        i: i32,
        remaining: i32,
        tmp: &Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if remaining == 0 {
            result.push(tmp.to_vec());
            return;
        }

        if i >= candidates.len() as i32 {
            return;
        }
        if remaining < 0 {
            return;
        }

        let mut new_vec = tmp.clone();
        new_vec.push(candidates[i as usize]);
        Self::digui(candidates, i + 1, remaining - candidates[i as usize], &new_vec, result);

        Self::digui(candidates, i + 1, remaining, tmp, result);
    }
}
