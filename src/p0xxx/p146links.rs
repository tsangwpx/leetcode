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

#[derive(Debug, Clone, Copy)]
struct Link {
    key: i32,
    prev: i16,
    next: i16,
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    value: i32,
    link_id: i16,
}

struct LRUCache {
    capacity: usize,
    table: HashMap<i32, Entry>,
    links: Box<[Link]>,
    head: i16,
    tail: i16,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        assert!(capacity >= 1);
        // println!("capacity: {}", capacity);

        let head = capacity as i16;
        let tail = capacity as i16 + 1;
        let capacity = capacity as usize;

        let mut links = vec![];
        links.reserve_exact(capacity);
        links.resize(
            capacity,
            Link {
                key: -1,
                prev: -1,
                next: -1,
            },
        );

        Self {
            capacity: capacity,
            // table: HashMap::with_capacity(capacity),
            table: HashMap::new(),
            links: links.into_boxed_slice(),
            head: -1,
            tail: -1,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(&entry) = self.table.get(&key) {
            self._refresh_link(entry.link_id);

            // println!(
            //     "after get({}): head={:?}, tail={:?}, table={:?}, links={:?}",
            //     key, self.head, self.tail, self.table, self.links
            // );
            entry.value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        // println!(
        //     "put key={}, value={}: head={}, tail={}, table={:?}, links={:?}",
        //     key, value, self.head, self.tail, self.table, self.links,
        // );

        use std::collections::hash_map::Entry::{Occupied, Vacant};

        enum KeyEvent {
            Create,
            Replace,
            Update,
        }

        let usage = self.table.len();

        let (link_id, event) = match self.table.entry(key) {
            Occupied(mut entry) => {
                let entry = entry.get_mut();
                entry.value = value;

                (entry.link_id, KeyEvent::Update)
            }
            Vacant(entry) => {
                let (link_id, event) = if usage >= self.capacity {
                    (self.tail, KeyEvent::Replace)
                } else {
                    (usage as i16, KeyEvent::Create)
                };
                entry.insert(Entry { value, link_id });

                (link_id, event)
            }
        };

        match event {
            KeyEvent::Create => {
                self.links[link_id as usize].key = key;
            }
            KeyEvent::Replace => {
                // println!("replace {}: {} {}", link_id, key, value);
                let link = &mut self.links[link_id as usize];
                self.table.remove(&link.key);
                self.links[link_id as usize].key = key;
            }
            KeyEvent::Update => {}
        }

        self._refresh_link(link_id);

        // println!(
        //     "after put({}, {}): head={:?}, tail={:?}, table={:?}, links={:?}",
        //     key, value, self.head, self.tail, self.table, self.links
        // );
    }

    fn _refresh_link(&mut self, link_id: i16) {
        if self.head == link_id {
            // we are good.
            return;
        }

        let links = &mut self.links[..];

        let Link { prev, next, .. } = links[link_id as usize];

        if prev >= 0 {
            links[prev as usize].next = next;
        }

        if next >= 0 {
            links[next as usize].prev = prev;
        }

        links[link_id as usize].prev = -1;
        links[link_id as usize].next = self.head;

        if self.head >= 0 {
            links[self.head as usize].prev = link_id;
        }

        self.head = link_id;

        if self.tail == link_id {
            self.tail = prev;
        } else if self.tail < 0 {
            self.tail = link_id;
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
