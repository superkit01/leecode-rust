pub struct Solution {}

use std::collections::LinkedList;

impl Solution {
    pub fn cal_points(ops: Vec<String>) -> i32 {
        let mut deque: LinkedList<String> = LinkedList::new();

        for i in ops.iter() {
            if i == "C" {
                deque.pop_back();
            } else if i == "D" {
                let tmp = deque.pop_back().unwrap();
                deque.push_back(tmp.clone());
                deque.push_back((tmp.parse::<i32>().unwrap() * 2).to_string())
            } else if i == "+" {
                let tmp1 = deque.pop_back().unwrap();
                let tmp2 = deque.pop_back().unwrap();
                deque.push_back(tmp2.clone());
                deque.push_back(tmp1.clone());
                deque.push_back(
                    (tmp1.parse::<i32>().unwrap() + tmp2.parse::<i32>().unwrap()).to_string(),
                );
            } else {
                deque.push_back(String::from(i));
            }
        }

        let mut result: i32 = 0;
        for i in deque.iter() {
            result += i.parse::<i32>().unwrap();
        }
        return result;
    }
}
