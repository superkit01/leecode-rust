use std::collections::*;

pub struct Solution {}

impl Solution {
    pub fn most_frequent(nums: Vec<i32>, key: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut i = 0;
        while i < nums.len() {
            if nums[i] == key {
                if i + 1 < nums.len() {
                    let  c = map.entry(nums[i + 1]).or_insert(0);
                    *c+=1;
                }
                i += 2;
            } else {
                i += 1;
            }
        }

        let mut  count = 0;
        let mut target = 0;

        for (k, v) in map.iter() {
            if *v > count {
                target = *k;
                count = *v;
            }
        }

        return target;
    }
}
