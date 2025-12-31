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

// Problem 2385

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, start: i32) -> Result<(i32, i32), i32> {
            // Ok((infection_time, distance))
            // Err(subtree_depth)

            let Some(node) = node else {
                return Err(0);
            };

            let TreeNode { val, left, right } = Rc::try_unwrap(node).unwrap().into_inner();

            // let inner = RefCell::borrow(&node);
            // let left = inner.left.clone();
            // let right = inner.right.clone();
            // let val = inner.val;
            // drop(inner);

            let left_result = dfs(left, start);
            let right_result = dfs(right, start);

            let result = if val == start {
                let depth = left_result.unwrap_err().max(right_result.unwrap_err());
                Ok((depth, 1))
            } else {
                match (left_result, right_result) {
                    (Err(left_depth), Err(right_depth)) => {
                        let depth = left_depth.max(right_depth);
                        Err(depth + 1)
                    }
                    (Ok((time, dist)), Err(depth)) | (Err(depth), Ok((time, dist))) => {
                        let time = time.max(depth + dist);
                        Ok((time, dist + 1))
                    }
                    (Ok(_), Ok(_)) => unreachable!(),
                }
            };
            // println!("{}: {:?}", val, result);
            result
        }

        dfs(root, start).unwrap().0
    }
}
