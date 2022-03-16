pub struct Solution {}

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let mut result: Vec<Vec<i32>> = Vec::new();
        let tmp: Vec<i32> = Vec::new();

        let mut sorted_candidates = candidates.clone();
        sorted_candidates.sort();

        Self::digui(&sorted_candidates, 0, target, &tmp, &mut result);

        println!("{:?}",result.len());
        Self::distinct(&mut result);

        return result;
    }

    pub fn digui( candidates: &Vec<i32>, i: i32, remaining: i32, tmp: &Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if remaining == 0 {
            result.push(tmp.to_vec());
            return ;
        }

        if i >= candidates.len() as i32 {
            return ;
        }
        if remaining < 0 {
            return ;
        }

        let mut new_vec = tmp.clone();
        new_vec.push(candidates[i as usize]);
        Self::digui(candidates, i + 1, remaining - candidates[i as usize], &new_vec,result);

        Self::digui(candidates, i + 1, remaining, tmp, result);
      
    }



    pub fn distinct (v: &mut Vec<Vec<i32>>) {

        let mut uniques:Vec<Vec<i32>> = Vec::new();
        v.retain(|e| {
            for elem in uniques.iter() {
               if  e.len()==elem.len() && e==elem {
                   return false;
               }
            }
            uniques.push(e.to_vec());
            return true;

        });
    }
}
