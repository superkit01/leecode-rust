use std::collections::*;

pub struct Solution {}

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut i = 0;
        while i < nums.len() - 1 {
            if nums[i] == key {
                let count = map.entry(nums[i + 1]).or_insert(0);
                *count += 1;
            }
            i += 1;
        }

        // let mut count = 0;
        // let mut target = 0;

        // for (k, v) in map.iter() {
        //     if *v > count {
        //         target = *k;
        //         count = *v;
        //     }
        // }

        let max = map.iter().max_by(|x, y| return (*x.1).cmp(&*y.1)).unwrap();

        return *max.0;
    }
}
