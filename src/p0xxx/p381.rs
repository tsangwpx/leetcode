// Problem 381
extern crate rand;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Default, Debug)]
struct RandomizedCollection {
    table: HashMap<i32, HashSet<usize>>,
    items: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedCollection {
    fn new() -> Self {
        Self::default()
    }

    fn insert(&mut self, val: i32) -> bool {
        let idx = self.items.len();
        self.items.push(val);

        let set = self.table.entry(val).or_insert_with(|| HashSet::new());
        set.insert(idx);

        set.len() == 1
    }

    fn remove(&mut self, val: i32) -> bool {
        let Some(indices) = self.table.get_mut(&val) else {
            return false;
        };

        let removed_idx = *indices.iter().next().unwrap();
        indices.remove(&removed_idx);

        if indices.is_empty() {
            self.table.remove(&val);
        }

        let last_idx = self.items.len() - 1;
        let last_item = self.items[last_idx];

        self.items.swap_remove(removed_idx);

        if removed_idx != last_idx {
            let indices = self.table.get_mut(&last_item).unwrap();
            indices.remove(&last_idx);
            indices.insert(removed_idx);
        }

        true
    }

    fn get_random(&self) -> i32 {
        let idx = thread_rng().gen_range(0, self.items.len());
        self.items[idx]
    }
}

/**
 * Your RandomizedCollection object will be instantiated and called as such:
 * let obj = RandomizedCollection::new();
 * let ret_1: bool = obj.insert(val);
 * let ret_2: bool = obj.remove(val);
 * let ret_3: i32 = obj.get_random();
 */
fn f() {}
