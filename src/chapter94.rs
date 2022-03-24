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

use std::cell::RefCell;
use std::rc::Rc;
pub struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = Vec::new();

        Self::digui(root, &mut result);

        return result;
    }

    pub fn digui(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        match node {
            None => {
                return;
            }
            Some(v) => {
                let tmp = v.as_ref().borrow_mut();
                Self::digui(tmp.left.clone(), result);
                result.push(tmp.val);
                Self::digui(tmp.right.clone(), result);
            }
        }
    }
}

// func inorderTraversal(root *TreeNode) []int {
// 	result := make([]int, 0)
// 	digui(root, &result)
// 	return result
// }

// func digui(root *TreeNode, result *[]int) {
// 	if root == nil {
// 		return
// 	}
//     digui(root.Left, result)
// 	*result = append(*result, root.Val)
// 	digui(root.Right, result)
// }
