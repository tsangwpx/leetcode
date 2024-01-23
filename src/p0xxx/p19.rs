// mod done;

use std::ops::BitAnd;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    // Problem 19
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let n = n as usize;
        let mut tmp = [0i32; 30];
        let mut count = 0;

        let head = head;
        let mut item = &head;

        while let Some(node) = item {
            tmp[count] = node.val;
            count += 1;
            item = &node.next;
        }

        tmp.copy_within((count - n + 1)..count, count - n);
        count -= 1;

        let mut root = Some(Box::from(ListNode { val: 0, next: head }));
        let mut item = &mut root;
        for i in 0..count {
            item = &mut item.as_mut().unwrap().next;
            item.as_mut().unwrap().val = tmp[i];
        }

        item.as_mut().unwrap().next = None;
        root.unwrap().next
    }
}
