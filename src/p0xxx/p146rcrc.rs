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

// Problem 146
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

type RCNode = Rc<RefCell<LRUNode>>;

struct LRUNode {
    key: i32,
    value: i32,
    prev: Option<RCNode>,
    next: Option<RCNode>,
}

impl std::fmt::Debug for LRUNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LRUNode")
            .field("key", &self.key)
            .field("value", &self.value)
            .field(
                "prev",
                &match &self.prev {
                    Some(node) => RefCell::borrow(&node).value,
                    None => -1,
                },
            )
            .field(
                "next",
                &match &self.next {
                    Some(node) => RefCell::borrow(&node).value,
                    None => -1,
                },
            )
            .finish()
    }
}

struct LRUCache {
    capacity: usize,
    table: HashMap<i32, RCNode>,
    head: Option<RCNode>,
    tail: Option<RCNode>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        assert!(capacity >= 1);
        // println!("capacity: {}", capacity);

        let capacity = capacity as usize;
        Self {
            capacity: capacity,
            table: HashMap::with_capacity(capacity),
            head: Option::None,
            tail: Option::None,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.table.get(&key).map(|s| s.clone()) {
            let value = node.borrow().value;
            self._refresh_node(node);

            // println!(
            //     "after get({}): head={:?}, tail={:?}, table={:?}",
            //     key, self.head, self.tail, self.table
            // );
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // println!(
        //     "put key={}, value={}: head={}, tail={}, table={:?}",
        //     key, value, self.head, self.tail, self.table
        // );

        let node = self.table.get(&key).map(|s| s.clone());

        // only remove the least recently used entry if space is needed
        if node.is_none() && self.table.len() == self.capacity {
            // update the new tail, point to the new tail, and remove the old tail

            if self.capacity == 1 {
                // one size cache

                self.table.clear();
                self.head = Option::None;
                self.tail = Option::None;
            } else {
                let removed_tail = std::mem::take(&mut self.tail).unwrap();
                let new_tail;

                {
                    let mut removed_node = RefCell::borrow_mut(&removed_tail);
                    self.table.remove(&removed_node.key);
                    new_tail = std::mem::take(&mut removed_node.prev).unwrap();
                }

                {
                    let mut tail_node = RefCell::borrow_mut(&new_tail);
                    tail_node.next = Option::None;
                }

                self.tail = Some(new_tail);
            }
        }

        if let Some(node) = node {
            RefCell::borrow_mut(&node).value = value;
            self._refresh_node(node);
        } else {
            // insert the new node

            let old_head = std::mem::take(&mut self.head);

            let new_node = Rc::new(RefCell::new(LRUNode {
                key: key,
                value: value,
                prev: Option::None,
                next: old_head.clone(),
            }));

            if let Some(old_head) = old_head {
                let mut old_head = RefCell::borrow_mut(&old_head);
                old_head.prev = Some(new_node.clone());
            } else {
                // old_head = None imply tail is also None
                self.tail = Some(new_node.clone());
            }

            self.head = Some(new_node.clone());
            self.table.insert(key, new_node);
        }

        // println!(
        //     "after put({}, {}): head={:?}, tail={:?}, table={:?}",
        //     key, value, self.head, self.tail, self.table
        // );
    }

    #[inline]
    fn _refresh_node(&mut self, node: RCNode) {
        // update the existing node
        // assume the key exist in the table

        let mut inner = RefCell::borrow_mut(&node);

        if inner.prev.is_none() {
            // we are good
        } else if inner.next.is_none() {
            // Set the new tail
            {
                let new_tail = std::mem::take(&mut inner.prev).unwrap();

                {
                    // we need a scope here because borrow checker is too smart.
                    let mut tail_node = RefCell::borrow_mut(&new_tail);
                    tail_node.next = Option::None;
                }

                self.tail = Some(new_tail);
            }

            // Set the new head
            {
                inner.next = self.head.clone();

                {
                    let mut prev_node = RefCell::borrow_mut(inner.next.as_deref().unwrap());
                    prev_node.prev = Some(node.clone());
                }

                self.head = Some(node.clone());
            }
        } else {
            let prev = std::mem::take(&mut inner.prev).unwrap();
            let next = std::mem::take(&mut inner.next).unwrap();
            RefCell::borrow_mut(&prev).next = Some(next.clone());
            RefCell::borrow_mut(&next).prev = Some(prev);

            // Set the new head
            {
                inner.next = self.head.clone();

                {
                    let mut prev_node = RefCell::borrow_mut(inner.next.as_deref().unwrap());
                    prev_node.prev = Some(node.clone());
                }

                self.head = Some(node.clone());
            }
        }
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn dummy() {}
