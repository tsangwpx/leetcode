use std::borrow::Borrow;
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
use std::rc::Rc;
use std::cell::RefCell;

pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut count = 0i32;

        #[derive(Copy, Clone, PartialEq)]
        enum Status {
            Camera,
            Watched,
            Unwatched,
        }

        use Status::*;

        fn compute(node: &Option<Rc<RefCell<TreeNode>>>) -> (Status, i32) {
            let node = match node {
                None => return (Watched, 0),
                Some(rc) => rc.try_borrow().unwrap(),
            };

            let (left_status, left_camera) = compute(&node.left);
            let (right_status, right_camera) = compute(&node.right);

            match (left_status, right_status) {
                (Unwatched, _) | (_, Unwatched) => (Camera, left_camera + right_camera + 1),
                (Camera, _) | (_, Camera) => (Watched, left_camera + right_camera),
                _ => (Unwatched, left_camera + right_camera),
            }
        }

        let (status, camera) = compute(&root);

        match status {
            Camera | Watched => camera,
            Unwatched => camera + 1,
        }
    }
}