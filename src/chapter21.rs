#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode {
            next: next,
            val: val,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {

        let mut head1 = match list1 {
            None => return list2,
            Some(v) => v,
        };

        let mut head2 = match list2 {
            None => return Some(head1),
            Some(v) => v,
        };

        if head1.val > head2.val {
            head2.next = Self::merge_two_lists(Some(head1), head2.next);
            return Some(head2);
        } else {
            head1.next = Self::merge_two_lists(head1.next, Some(head2));
            return Some(head1);
        }
    }
}
