mod leetcode_prelude;

use leetcode_prelude::*;
use rand::distributions::uniform::UniformSampler;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2336

struct SmallestInfiniteSet {
    boundary: i32,
    btree: std::collections::BTreeSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            boundary: 1,
            btree: Default::default(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        match self.btree.pop_first() {
            Some(smallest) => smallest,
            None => {
                let smallest = self.boundary;
                self.boundary += 1;
                smallest
            }
        }
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.boundary {
            return;
        } else if num + 1 == self.boundary {
            self.boundary -= 1;
            self.btree.remove(&num);
        } else {
            self.btree.insert(num);
        }
    }
}

/**
 * Your SmallestInfiniteSet object will be instantiated and called as such:
 * let obj = SmallestInfiniteSet::new();
 * let ret_1: i32 = obj.pop_smallest();
 * obj.add_back(num);
 */
fn dummy() {}
