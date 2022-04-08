pub struct Solution {}
use std::cmp;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len()==1{
            return intervals
        }

        let mut result: Vec<Vec<i32>> = Vec::new();

        // 排序
        let mut mut_intervals = intervals.clone();
        mut_intervals.sort_by(|a, b| a[0].cmp(&(b[0])));

        //合并
        let mut prev = mut_intervals[0].clone();

        for i in 1..mut_intervals.len() {
            let current = mut_intervals[i].clone();
            if current[0] > prev[1] || current[1] < prev[0] {
                result.push(prev);
                prev = current;
            } else {
                prev[0] = cmp::min(prev[0], current[0]);
                prev[1] = cmp::max(prev[1], current[1]);
            }
            if i == mut_intervals.len() - 1 {
                result.push(prev.clone())
            }
        }

        return result;
    }
}
