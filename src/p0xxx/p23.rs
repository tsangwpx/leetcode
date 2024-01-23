// mod done;

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
    // Problem 23
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        impl std::cmp::PartialOrd for Box<ListNode> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                self.val.partial_cmp(&other.val)
            }
        }

        impl std::cmp::Ord for Box<ListNode> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.val.cmp(&other.val)
            }
        }

        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let mut heap = BinaryHeap::<Reverse<Box<ListNode>>>::new();
        for mut head in lists.into_iter() {
            while let Some(mut node) = head {
                head = std::mem::take(&mut node.next);
                heap.push(Reverse(node));
            }
        }

        let mut root = Box::from(ListNode { val: 0, next: None });
        let mut tail = &mut root;

        while let Some(Reverse(item)) = heap.pop() {
            tail.next = Some(item);
            tail = tail.next.as_mut().unwrap();
        }

        root.next
    }
}
