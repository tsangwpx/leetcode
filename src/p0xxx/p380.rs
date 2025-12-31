// Problem 380
extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;

#[derive(Debug)]
struct RandomizedSet {
    table: HashMap<i32, usize>,
    numbers: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        Self {
            table: Default::default(),
            numbers: Default::default(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        match self.table.entry(val) {
            std::collections::hash_map::Entry::Occupied(_) => false,
            std::collections::hash_map::Entry::Vacant(entry) => {
                let index = self.numbers.len();
                entry.insert(index);
                self.numbers.push(val);
                true
            }
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(index) = self.table.remove(&val) {
            self.numbers.swap_remove(index);

            if let Some(&moved) = self.numbers.get(index) {
                self.table.insert(moved, index);
            }

            true
        } else {
            false
        }
    }

    fn get_random(&mut self) -> i32 {
        *self.numbers.choose(&mut thread_rng()).unwrap()
    }
}

/**
 * Your RandomizedSet object will be instantiated and called as such:
 * let obj = RandomizedSet::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn dummy() {}
