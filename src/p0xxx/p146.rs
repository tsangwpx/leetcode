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
use std::collections::HashMap;

#[derive(Debug)]
struct LRUNode {
    value: i32,
    prev: i32,
    next: i32,
}

struct LRUCache {
    capacity: usize,
    table: HashMap<i32, LRUNode>,
    head: i32,
    tail: i32,
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
            head: -1,
            tail: -1,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        // println!(
        //     "get key={}: head={}, tail={}, table={:?}",
        //     key, self.head, self.tail, self.table
        // );

        if let Some(&LRUNode { value, prev, next }) = self.table.get(&key) {
            self._update_node(key, Option::None, prev, next);

            // println!(
            //     "after: head={}, tail={}, table={:?}",
            //     self.head, self.tail, self.table
            // );
            value
        } else {
            -1
        }
    }

    #[inline]
    fn _update_node(&mut self, key: i32, value: Option<i32>, prev: i32, next: i32) {
        // update the existing node
        // assume the key exist in the table

        if self.head == key {
            // update value. Node order is good.
            let node = self.table.get_mut(&key).unwrap();
            if let Some(value) = value {
                node.value = value;
            }
        } else if self.tail == key {
            let old_head = self.head;

            // set the new tail node
            let node = self.table.get_mut(&prev).unwrap();
            node.next = -1;

            // update the cache tail
            self.tail = prev;

            // set the new head node
            let node = self.table.get_mut(&key).unwrap();
            node.next = self.head;
            node.prev = -1;

            if let Some(value) = value {
                node.value = value;
            }

            // set the old head node
            let node = self.table.get_mut(&old_head).unwrap();
            node.prev = key;

            // update the cache head
            self.head = key;
        } else {
            let old_head = self.head;

            // set the prev node
            let node = self.table.get_mut(&prev).unwrap();
            node.next = next;

            // set the next node
            let node = self.table.get_mut(&next).unwrap();
            node.prev = prev;

            // set this node
            let node = self.table.get_mut(&key).unwrap();
            node.prev = -1;
            node.next = old_head;

            if let Some(value) = value {
                node.value = value;
            }

            // set the old head
            let node = self.table.get_mut(&old_head).unwrap();
            node.prev = key;

            // set the cache head
            self.head = key;
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // println!(
        //     "put key={}, value={}: head={}, tail={}, table={:?}",
        //     key, value, self.head, self.tail, self.table
        // );

        let link = self.table.get(&key).map(|x| (x.prev, x.next));

        // only remove the least recently used entry if space is needed
        if link.is_none() && self.table.len() == self.capacity {
            // update the new tail, point to the new tail, and remove the old tail

            if self.head == self.tail {
                // one size cache

                self.table.clear();
                self.head = -1;
                self.tail = -1;
            } else {
                let old_tail = self.tail;
                let old_node = self.table.remove(&old_tail).unwrap();
                let new_tail = old_node.prev;

                let new_node = self.table.get_mut(&new_tail).unwrap();
                new_node.next = -1;
                self.tail = new_tail;
            }
        }

        if let Some((prev, next)) = link {
            self._update_node(key, Some(value), prev, next);
        } else {
            // insert the new node
            let old_head = self.head;

            let new_node = LRUNode {
                value,
                prev: -1,
                next: old_head,
            };

            self.table.insert(key, new_node);

            if old_head == -1 {
                self.head = key;
                self.tail = key;
            } else {
                self.head = key;
                let node = self.table.get_mut(&old_head).unwrap();
                node.prev = key;
            }
        }

        // println!(
        //     "after: head={}, tail={}, table={:?}",
        //     self.head, self.tail, self.table
        // );
    }
}

/**
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
fn dummy() {}
