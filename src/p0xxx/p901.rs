mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 901

#[derive(Default)]
struct StockSpanner {
    day: i32,
    stack: Vec<(i32, i32)>, // decreasing stack
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {
    fn new() -> Self {
        Default::default()
    }

    fn next(&mut self, price: i32) -> i32 {
        self.day += 1;
        let today = self.day;

        while let Some((hist_price, _)) = self.stack.last().copied() {
            if hist_price > price {
                break;
            }

            self.stack.pop();
        }

        // println!("{:?}", self.stack);

        let span = if let Some((_, hist_day)) = self.stack.last() {
            today - hist_day
        } else {
            today
        };

        self.stack.push((price, today));
        span
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
fn f() {}
