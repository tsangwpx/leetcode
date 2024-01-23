mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

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

// Problem 1026
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
            let Some(node) = node else {
                return (0, i32::MAX, i32::MIN);
            };

            let node = Rc::try_unwrap(node).unwrap().into_inner();
            let TreeNode { val, left, right } = node;

            let (left_diff, left_min, left_max) = dfs(left);
            let (right_diff, right_min, right_max) = dfs(right);
            let min = val.min(left_min).min(right_min);
            let max = val.max(left_max).max(right_max);

            let diff = [left_diff, right_diff, (val - min).abs(), (val - max).abs()]
                .iter()
                .max()
                .copied()
                .unwrap();

            // println!("{}: {} {} {}", val, diff, min, max);

            (diff, min, max)
        }

        dfs(root).0
    }
}
