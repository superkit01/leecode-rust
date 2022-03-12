mod chapter38;

mod chapter21;
use chapter21::*;

fn main() {
    println!("hello world");

    //## chapter 38

    // let result = chapter38::Solution::count_and_say(4);
    // println!("{}", result);

    //## chapter 21
    let list1: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            2,
            Some(Box::new(ListNode::new(4, None))),
        ))),
    )));
    println!("{:?}", list1);

    let list2: Option<Box<ListNode>> = Some(Box::new(ListNode::new(
        1,
        Some(Box::new(ListNode::new(
            3,
            Some(Box::new(ListNode::new(4, None))),
        ))),
    )));
    println!("{:?}", list2);
    let result21 = Solution::merge_two_lists(list1, list2);
    println!("{:?}", result21)
}
