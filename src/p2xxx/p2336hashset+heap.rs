mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2336

struct SmallestInfiniteSet {
    start: i32, // start of the infinite consective integers
    table: std::collections::HashSet<i32>,
    heap: std::collections::BinaryHeap<std::cmp::Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SmallestInfiniteSet {
    fn new() -> Self {
        Self {
            start: 1,
            table: Default::default(),
            heap: Default::default(),
        }
    }

    fn pop_smallest(&mut self) -> i32 {
        while let Some(std::cmp::Reverse(number)) = self.heap.pop() {
            if self.table.remove(&number) {
                return number;
            }
        }

        let smallest = self.start;
        self.start += 1;
        smallest
    }

    fn add_back(&mut self, num: i32) {
        if num >= self.start {
            return;
        }

        if num + 1 == self.start {
            self.start -= 1;
            self.table.remove(&num);
            return;
        }

        if self.table.insert(num) {
            self.heap.push(std::cmp::Reverse(num));
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
