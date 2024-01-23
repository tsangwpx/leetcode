mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 938
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

// Problem 938
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        let mut start = root;
        let mut pending = vec![];

        loop {
            let node = start.clone();
            let inner = if let Some(node) = &node {
                RefCell::<TreeNode>::borrow(node)
            } else {
                break;
            };

            let val = inner.val;

            if val > high {
                start = inner.left.clone();
            } else if val < low {
                start = inner.right.clone();
            } else {
                pending.push(start.clone().unwrap());
                break;
            }
        }

        let mut sum = 0;

        while let Some(node) = pending.pop() {
            let inner = RefCell::borrow(&node);
            let val = inner.val;

            // println!("Find {}", val);

            if val > high {
                if let Some(child) = &inner.left {
                    pending.push(child.clone());
                }
                continue;
            } else if val < low {
                if let Some(child) = &inner.right {
                    pending.push(child.clone());
                }
                continue;
            }

            sum += val;
            if let Some(child) = &inner.left {
                pending.push(child.clone());
            }

            if let Some(child) = &inner.right {
                pending.push(child.clone());
            }
        }

        sum
    }
}
