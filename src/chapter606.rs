use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        return Self::dp(root);
    }

    fn dp(node: Option<Rc<RefCell<TreeNode>>>) -> String {
        match node {
            None => {
                return String::from("");
            }
            Some(v) => {
                let node = v.as_ref().borrow_mut();

                let mut s = node.val.to_string();

                if node.left != None || node.right != None {
                    s += "(";
                    s += Self::dp(node.left.clone()).as_str();
                    s += ")";
                }
                if node.right != None {
                    s += "(";
                    s += Self::dp(node.right.clone()).as_str();
                    s += ")";
                }

                return s;
            }
        }
    }
}

// func tree2str(root *TreeNode) string {
// 	return dfs(root)
// }

// func dfs(node *TreeNode) string {
// 	if node == nil {
// 		return ""
// 	}
// 	s := strconv.Itoa(node.Val)
// 	if node.Left != nil || node.Right != nil {
// 		s += "(" + dfs(node.Left) + ")"
// 	}
// 	if node.Right != nil {
// 		s += "(" + dfs(node.Right) + ")"
// 	}
// 	return s
// }
