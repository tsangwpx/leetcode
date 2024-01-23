// mod done;

use std::ops::BitAnd;

fn main() {
    println!("Hello, world!");
}

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

struct Solution {}
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // Problem 606
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let mut repr = "".to_owned();
        Self::_recurse(&mut repr, root);
        repr
    }

    fn _recurse(repr: &mut String, node: Option<Rc<RefCell<TreeNode>>>) {
        use std::fmt::Write;

        if let Some(node) = node {
            let borrowed = node.borrow();
            write!(repr, "{}", borrowed.val).unwrap();

            if borrowed.left.is_some() || borrowed.right.is_some() {
                repr.push('(');
                Self::_recurse(repr, borrowed.left.clone());
                repr.push(')');
            }

            if borrowed.right.is_some() {
                repr.push('(');
                Self::_recurse(repr, borrowed.right.clone());
                repr.push(')');
            }
        }
    }
}
