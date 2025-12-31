// Problem 124
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
impl Solution {
    // Problem 124
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn recurse(max_path: &mut i32, node: Rc<RefCell<TreeNode>>) -> i32 {
            let node = node.borrow();

            let left_partial = node
                .left
                .clone()
                .map(|tree| recurse(max_path, tree))
                .unwrap_or(0);

            let right_partial = node
                .right
                .clone()
                .map(|tree| recurse(max_path, tree))
                .unwrap_or(0);

            let node_max = node.val + [0, left_partial, right_partial].iter().max().unwrap();
            *max_path = *[*max_path, node_max, left_partial + right_partial + node.val]
                .iter()
                .max()
                .unwrap();

            node_max
        }

        let root = root.unwrap();

        let mut max_path = root.borrow().val;

        recurse(&mut max_path, root);

        max_path
    }
}
