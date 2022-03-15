// mod chapter38;

// mod chapter21;
// use chapter21::*;

// mod chapter39;
// use chapter39::*;

mod chapter40;
use chapter40::*;


fn main() {
    println!("hello world");

    // ## chapter 38

    // let result = chapter38::Solution::count_and_say(4);
    // println!("{}", result);

    //## chapter 21
    // let list1: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
    //     1,
    //     Some(Box::new(ListNode::new(
    //         2,
    //         Some(Box::new(ListNode::new(4, None))),
    //     ))),
    // )));
    // println!("{:?}", list1);

    // let list2: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
    //     1,
    //     Some(Box::new(ListNode::new(
    //         3,
    //         Some(Box::new(ListNode::new(4, None))),
    //     ))),
    // )));
    // println!("{:?}", list2);
    // let result21 = Solution::merge_two_lists(list1, list2);
    // println!("{:?}", result21)

    let candidates = vec![10,1,2,7,6,1,5];
    let target = 8;
    let result: Vec<Vec<i32>> = Solution::combination_sum(candidates, target);
    println!("{:?}", result)
}
