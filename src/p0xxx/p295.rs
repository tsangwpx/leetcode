// Problem 295

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MedianFinder {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
        }
    }

    fn add_num(&mut self, num: i32) {
        self.left.push(num);

        // Two things:
        // 1. len(left) >= len(right)  and 0 <= len(left) - len(right) <= 1
        // 2. TOP(left) <= TOP(right)

        if !self.left.is_empty() && !self.right.is_empty() {
            loop {
                let left_peek = self.left.peek_mut().unwrap();
                let right_peek = self.right.peek_mut().unwrap();

                let left_value = *left_peek;
                if left_value <= right_peek.0 {
                    break;
                }

                let mut left_peek = left_peek;
                *left_peek = right_peek.0;

                let mut right_peek = right_peek;
                *right_peek = Reverse(left_value);
            }
        }

        if self.left.len() - self.right.len() >= 2 {
            self.right.push(Reverse(self.left.pop().unwrap()));
        }
    }

    fn find_median(&self) -> f64 {
        let delta = self.left.len() - self.right.len();
        match delta {
            0 => (*self.left.peek().unwrap() as f64 + self.right.peek().unwrap().0 as f64) / 2.0f64,
            1 => *self.left.peek().unwrap() as f64,
            _ => unreachable!("{}", delta),
        }
    }
}

/**
 * Your MedianFinder object will be instantiated and called as such:
 * let obj = MedianFinder::new();
 * obj.add_num(num);
 * let ret_2: f64 = obj.find_median();
 */
struct dummy();
